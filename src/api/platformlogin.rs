#![allow(non_snake_case)]

use crate::{
    data::{
        TencerData, 
        accounts::Account
    },
    Global,
    create_jwt,
};
use rocket::{
    request::Form,
    State,
};
use rocket_contrib::json::Json;
use rocket_contrib::databases::diesel::prelude::*;
use serde::{Serialize, Deserialize};
use chrono::{NaiveDateTime, DateTime, Utc};
use jsonwebtoken::EncodingKey;

pub mod v1 {
    use super::*;

    #[derive(FromForm, Serialize, Deserialize)]
    pub struct PlatformIdPair {
        Platform: String,
        PlatformId: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Profile {
        Id: i64,
        Username: String,
        DisplayName: String,
        XP: i32,
        Level: i32,
        Verified: bool,
        Developer: bool,
        HasEmail: bool,
        CanReceiveInvites: bool,
        ProfileImageName: String,
        JuniorProfile: bool,
        ForceJuniorImages: bool,
        PendingJunior: bool,
        HasBirthday: bool,
        AvoidJuniors: bool,
        EmailEnteredAt: DateTime<Utc>,
        PlayerReputation: PlayerNoteriety,
        PlatformIds: Vec<PlatformIdPair>,
    }

    impl Profile {
        pub fn from_account(account: &Account) -> Profile {
            Profile {
                Id: account.id as i64,
                Username: account.username.to_owned(),
                DisplayName: account.displayname.to_owned(),
                XP: account.xp,
                Level: 99,
                Verified: true,
                Developer: true,
                HasEmail: true,
                CanReceiveInvites: true,
                ProfileImageName: "no".to_owned(),
                JuniorProfile: false,
                ForceJuniorImages: false,
                PendingJunior: false,
                HasBirthday: true,
                AvoidJuniors: true,
                EmailEnteredAt: DateTime::from_utc(NaiveDateTime::from_timestamp(0,0), Utc),
                PlayerReputation: PlayerNoteriety::default(),
                PlatformIds: vec![],
            }
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct CachedLogin {
        Id: i64,
        Username: String,
        DisplayName: String,
        XP: i32,
        Level: i32,
        RegistrationStatus: i32,
        Developer: bool,
        CanReceiveInvites: bool,
        ProfileImageName: String,
        JuniorProfile: bool,
        ForceJuniorImages: bool,
        PendingJunior: bool,
        HasBirthday: bool,
        AvoidJuniors: bool,
        PlayerReputation: PlayerNoteriety,
        PlatformIds: Vec<PlatformIdPair>,
    }

    impl CachedLogin {
        pub fn from_account(account: &Account) -> CachedLogin {
            CachedLogin {
                Id: account.id as i64,
                Username: account.username.to_owned(),
                DisplayName: account.displayname.to_owned(),
                XP: account.xp,
                Level: 99,
                RegistrationStatus: 2, // registered
                Developer: true,
                CanReceiveInvites: true,
                ProfileImageName: "no".to_owned(),
                JuniorProfile: false,
                ForceJuniorImages: false,
                PendingJunior: false,
                HasBirthday: true,
                AvoidJuniors: true,
                PlayerReputation: PlayerNoteriety::default(),
                PlatformIds: vec![],
            }
        }
    }

    #[derive(Serialize, Deserialize)]
    #[allow(dead_code)]
    pub struct LoginCachedBody {
        Platform: i32,
        PlatformId: String,
        AppVersion: String,
        BuildTimestamp: i64,
        ClientTimestamp: i64,
        DeviceId: String,
        PlayerId: i32,
    }

    #[derive(Serialize, Deserialize, Default)]
    struct PlayerNoteriety {
        Noteriety: i32,
        CheerGeneral: i32,
        CheerHelpful: i32,
        CheerGreatHost: i32,
        CheerSportsman: i32,
        CheerCreative: i32,
        CheerCredit: i32,
        SelectedCheer: Option<i32>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct LoginResponse {
        Error: Option<String>,
        Player: Option<CachedLogin>,
        Token: Option<String>,
        FirstLoginOfTheDay: bool,
        AnalyticsSessionId: i64,
    }

    #[post("/platformlogin/v1/profiles", format = "form", data = "<body>")]
    pub fn profiles(state: State<Global>, db: TencerData, body: Form<PlatformIdPair>) -> QueryResult<Json<Vec<Profile>>> {
        let mut result: Vec<Profile> = db.get_linked_accounts(&body.Platform, &body.PlatformId)?
            .iter()
            .map(|a| Profile::from_account(a))
            .collect();
        
        if result.len() == 0 {
            // TODO should probably check validity of platform id
            let new = db.generate_account(&body.Platform, &body.PlatformId, state.ad_hoc_hosting)?;
            result.push(Profile::from_account(&new));
        }
        
        Ok(Json(result))
    }
    
    #[post("/platformlogin/v1/getcachedlogins", format = "form", data = "<body>")]
    pub fn cached_logins(state: State<Global>, db: TencerData, body: Form<PlatformIdPair>) -> QueryResult<Json<Vec<CachedLogin>>> {
        let mut result: Vec<CachedLogin> = db.get_linked_accounts(&body.Platform, &body.PlatformId)?
            .iter()
            .map(|a| CachedLogin::from_account(a))
            .collect();
        
        if result.len() == 0 {
            // TODO should probably check validity of platform id
            let new = db.generate_account(&body.Platform, &body.PlatformId, state.ad_hoc_hosting)?;
            result.push(CachedLogin::from_account(&new));
        }
        
        Ok(Json(result))
    }

    #[post("/platformlogin/v1/logincached", format = "json", data = "<body>")]
    pub fn login_cached(db: TencerData, encode: State<EncodingKey>, body: Json<LoginCachedBody>) -> QueryResult<Json<LoginResponse>> {
        if db.check_account_linked(&body.Platform.to_string(), &body.PlatformId, body.PlayerId)? {
            Ok(Json(LoginResponse {
                Error: None,
                Player: Some(CachedLogin::from_account(&db.get_account(body.PlayerId)?.unwrap())),
                Token: Some(create_jwt(&*encode, body.PlayerId, body.AppVersion.clone())),
                FirstLoginOfTheDay: false,
                AnalyticsSessionId: 0,
            }))
        } else {
            Ok(Json(LoginResponse {
                Error: Some("account not linked".to_owned()),
                Player: None,
                Token: None,
                FirstLoginOfTheDay: false,
                AnalyticsSessionId: 0,
            }))
        }
    }
}

pub mod v6 {
    use super::*;
    #[derive(FromForm)]
    #[allow(dead_code)]
    pub struct LoginBody {
        Platform: String,
        PlatformId: String,
        Name: String,
        AppVersion: String,
        BuildTimestamp: String,
        ClientTimestamp: String,
        DeviceId: String,
        PlayerId: String,
        Verify: String,
        AuthParams: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct LoginResponse {
        PlayerId: i64,
        Token: Option<String>,
        Error: Option<String>,
        FirstLoginOfTheDay: bool,
    }

    #[post("/platformlogin/v6", format = "form", data = "<body>")]
    pub fn post(encode: State<EncodingKey>, db: TencerData, body: Form<LoginBody>) -> QueryResult<Json<LoginResponse>> {
        // TODO properly handle bad input
        let account_id = body.PlayerId.parse::<i32>().unwrap_or(-1);
        
        if db.check_account_linked(&body.Platform, &body.PlatformId, account_id)? {
            Ok(Json(LoginResponse {
                PlayerId: account_id as i64,
                Token: Some(create_jwt(&*encode, account_id, body.AppVersion.clone())),
                Error: None,
                FirstLoginOfTheDay: false,
            }))
        } else {
            Ok(Json(LoginResponse {
                PlayerId: -1,
                Token: None,
                Error: Some("account not linked".to_owned()),
                FirstLoginOfTheDay: false,
            }))
        }
    }
}



