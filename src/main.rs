#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;

use data::TencerData;
use rocket::{
    fairing::AdHoc,
    request::{
        FromRequest,
        Request,
        Outcome,
    },
    http::Status,
    State,
};
use tokio_tungstenite::tungstenite::Message;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{
    DecodingKey,
    EncodingKey,
    Validation,
    Algorithm,
    Header,
};
use chrono::Utc;
use uuid::Uuid;
use std::{
    sync::{
        Mutex,
    },
    collections::{
        HashMap,
    },
};

mod data;
mod api;
mod ws;
mod photon;

embed_migrations!();

#[tokio::main]
async fn main() {
    // websocket loop started in create_rocket
    create_rocket().launch();
}

fn create_rocket() -> rocket::Rocket {
    let api = routes![
        api::versioncheck_v3,

        api::config::v2,
        api::config::v1_amplitude,
        
        api::platformlogin::v1::profiles,
        api::platformlogin::v1::cached_logins,
        api::platformlogin::v1::login_cached,
        api::platformlogin::v6::post,

        api::playerreporting::v1::ban_details,

        api::relationships::v1::bulk_ignore_platform_users,

        api::players::v1::player,
        api::players::v1::complete_objectives,
        api::players::v2::set_display_name,

        api::messages::v2::get,

        api::relationships::v2::get,

        api::avatar::v2::get,
        api::avatar::v2::set,
        api::avatar::v2::gifts,
        api::avatar::v3::unlocked,

        api::settings::v2::get,
        api::settings::v2::set,

        api::equipment::v1::unlocked,

        api::events::v3::list,

        api::activities::charades::v1::words,

        api::challenge::v1::current,

        api::gamesessions::v2::join_random,
        api::gamesessions::v2::report_join_result,
        api::gamesessions::v2::list_public_events,
        api::gamesessions::v2::create,

        api::images::v1::named,
    ];

    let r = rocket::ignite()
        .attach(TencerData::fairing())
        .mount("/api", api)
        .attach(AdHoc::on_launch("Database setup", |r| {
            let db = TencerData::get_one(r).unwrap();
            embedded_migrations::run(&db.0).expect("Failed to run database migrations");
        }));
    
    let secret = r.config().get_str("jwt_secret").expect("couldn't get jwt_secret from config");
    let decode = DecodingKey::from_base64_secret(secret).expect("failed to create jwt decoding key");
    let encode = EncodingKey::from_base64_secret(secret).expect("failed to create jwt encoding key");

    let global = Global {
        player_map: ws::PlayerMap::new(Mutex::new(HashMap::new())),
        sessions: photon::InstanceMap::new(Mutex::new(HashMap::new())),
        ad_hoc_hosting: r.config().get_bool("ad_hoc_hosting").expect("couldn't get ad_hoc_hosting from config"),
        ad_hoc_region: r.config().get_string("ad_hoc_region").expect("couldn't get ad_hoc_region from config"),
    };
    
    r
        .manage(encode)
        .manage(decode)
        .manage(global)
        .attach(AdHoc::on_launch("Websocket server", |r| {
            let port = r.config().get_int("ws_port").unwrap_or(56701) as u16;
            let addr = r.config().get_str("address").unwrap_or("localhost");
            let bind = format!("{}:{}", addr, port);
            let player_map = r.state::<Global>().unwrap().player_map.clone();
            tokio::spawn(ws::start(bind, player_map));
        }))
}

pub struct Global {
    player_map: ws::PlayerMap,
    sessions: photon::InstanceMap,
    ad_hoc_hosting: bool,
    ad_hoc_region: String,
}

impl Global {    
    pub fn send_msg_to_subscribed<T: ?Sized + serde::Serialize>(&self, msg_id: i32, msg: &T, account_id: i32) -> Result<(), futures::channel::mpsc::SendError> {
        let msg = format!("{{\"Id\":{},\"Msg\":{}}}", msg_id, serde_json::to_string(msg).unwrap());
        let mut players = self.player_map.lock().unwrap();
        let targets = players.get_mut(&account_id).unwrap().subbed_by.clone();
        
        for p in targets {
            players.get_mut(&p).unwrap().ws_tx.start_send(Message::Text(msg.to_owned()))?;
        }

        Ok(())
    }

    pub fn reserve_photon_instance(&self, activity_id: &str, game_version: &str, pref_region: &str) -> (i32, photon::Instance) {
        for (id, instance) in self.sessions.lock().unwrap().iter_mut() {
            if !instance.private && 
                instance.player_count + 1 <= instance.capacity &&
                instance.activity_id == activity_id &&
                instance.game_version == game_version 
            {
                // println!("found existing session: {}", id);
                return (*id, instance.clone());
            }

            // println!("did not match: {}\n{:#?}", id, instance);
        }

        let new = photon::Instance {
            region: pref_region.to_owned(),
            photon_id: Uuid::new_v4().to_hyphenated().to_string(),
            activity_id: activity_id.to_owned(),
            game_version: game_version.to_owned(),
            private: false,
            in_progress: false,
            player_count: 0,
            capacity: get_capacity_for_activity(activity_id),
        };

        // println!("creating new session:\n{:#?}", new);
        
        let mut sessions = self.sessions.lock().unwrap();
        for i in 0..i32::MAX {
            if sessions.contains_key(&i) {
                continue;
            }

            sessions.insert(i, new.clone());
            return (i, new);
        }

        panic!("no space in photon instance map");
    }

    pub fn create_photon_instance(&self, activity_id: &str, game_version: &str, pref_region: &str, private: bool) -> (i32, photon::Instance) {
        let new = photon::Instance {
            region: pref_region.to_owned(),
            photon_id: Uuid::new_v4().to_hyphenated().to_string(),
            activity_id: activity_id.to_owned(),
            game_version: game_version.to_owned(),
            private: private,
            in_progress: false,
            player_count: 0,
            capacity: get_capacity_for_activity(activity_id),
        };

        let mut sessions = self.sessions.lock().unwrap();
        for i in 0..i32::MAX {
            if sessions.contains_key(&i) {
                continue;
            }

            sessions.insert(i, new.clone());
            return (i, new);
        }

        panic!("no space in photon instance map");
    }
}

fn get_capacity_for_activity(_activity_id: &str) -> u32 {
    // TODO
    40
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: i32,
    exp: i64,
    vers: String,
}

pub struct Player {
    pub account_id: i32,
    pub game_ver: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for Player {
    type Error = AuthError;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let decode = request.guard::<State<DecodingKey>>().expect("no decoding key state");
        let validate = Validation::new(Algorithm::HS256);

        for auth in request.headers().get("Authorization") {
            let parts: Vec<&str> = auth.split(' ').collect();
            
            if parts.len() != 2 || parts[0] != "Bearer" {
                return Outcome::Failure((Status::Unauthorized, AuthError::InvalidAuthHeader));
            }

            let claims = match jsonwebtoken::decode::<Claims>(parts[1], &*decode, &validate) {
                Ok(c) => c.claims,
                Err(e) => return Outcome::Failure((Status::Unauthorized, AuthError::TokenError(e))),
            };

            let player = Player {
                account_id: claims.sub,
                game_ver: claims.vers,
            };

            return Outcome::Success(player);
        }

        Outcome::Failure((Status::Unauthorized, AuthError::NoAuthHeader))
    }
}

pub fn create_jwt(encode: &EncodingKey, account_id: i32, game_ver: String) -> String {
    let claims = Claims {
        exp: Utc::now().timestamp() + 3600 * 24,
        sub: account_id,
        vers: game_ver,
    };

    jsonwebtoken::encode(&Header::new(Algorithm::HS256), &claims, encode).expect("failed to create jwt")
}

#[derive(Debug)]
pub enum AuthError {
    NoAuthHeader,
    InvalidAuthHeader,
    TokenError(jsonwebtoken::errors::Error),
}
