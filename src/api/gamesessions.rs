#![allow(non_snake_case)]

use crate::{
    Global,
    Player,
    photon,
};
use rocket_contrib::json::Json;
use rocket::{
    State,
};
use serde::{
    Serialize, 
    Deserialize,
};
use rand::prelude::*;

pub mod v2 {
    use super::*;

    const MSG_UPDATE_PRESENCE: i32 = 12;
    // const MSG_UPDATE_GAMESESSION: i32 = 13;

    #[derive(Serialize, Deserialize)]
    pub struct CloudRegionPing {
        RegionId: String,
        Ping: i32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct JoinRandomRoomRequest {
        ActivityLevelIds: Vec<String>,
        ExpectedPlayerIds: Vec<i64>,
        RegionPings: Vec<CloudRegionPing>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct CreateRoomRequest {
        ActivityLevelId: String,
        ExpectedPlayerIds: Vec<i64>,
        RegionPings: Vec<CloudRegionPing>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct JoinResponse {
        Result: i32,
        #[serde(skip_serializing_if="Option::is_none")]
        GameSession: Option<GameSession>,
    }

    #[allow(dead_code)]
    impl JoinResponse {
        const SUCCESS: i32 = 0;
        const NO_SUCH_GAME: i32 = 1;
        const PLAYER_NOT_ONLINE: i32 = 2;
        const INSUFFICIENT_SPACE: i32 = 3;
        const EVENT_NOT_STARTED: i32 = 4;
		const EVENT_ALREADY_FINISHED: i32 = 5;
		const EVENT_CREATOR_NOT_READY: i32 = 6;
		const BLOCKED: i32 = 7;
		const PROFILE_LOCKED: i32 = 8;
		const NO_BIRTHDAY: i32 = 9;
		const MARKED_FOR_DELETE: i32 = 10;
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct JoinResult {
        GameSessionId: i64,
        RegionId: String,
        RoomId: String,
        Result: i32,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct GameSession {
        GameSessionId: i64,
        RegionId: String,
        RoomId: String,
        EventId: Option<i64>,
        CreatorPlayerId: Option<i64>,
        Name: String,
        ActivityLevelId: String,
        Private: bool,
        GameInProgress: bool,
        MaxCapacity: i32,
        IsFull: bool,
    }

    impl GameSession {
        pub fn from_photon_instance(session_id: i32, instance: photon::Instance, creator: Option<i32>) -> GameSession {
            GameSession {
                GameSessionId: session_id as i64,
                RegionId: instance.region,
                RoomId: instance.photon_id,
                EventId: None,
                CreatorPlayerId: creator.map(|c| c as i64),
                Name: "".to_owned(),
                ActivityLevelId: instance.activity_id,
                Private: instance.private,
                GameInProgress: instance.in_progress,
                MaxCapacity: instance.capacity as i32,
                IsFull: instance.player_count >= instance.capacity,
            }
        }
    }

    #[post("/gamesessions/v2/joinrandom", format = "json", data = "<request>")]
    pub fn join_random(player: Player, state: State<Global>, request: Json<JoinRandomRoomRequest>) -> Json<JoinResponse> {
        if request.ActivityLevelIds.len() == 0 {
            return Json(JoinResponse { Result: JoinResponse::NO_SUCH_GAME, GameSession: None });
        }

        // println!("got matchmaking request from {} on version {}, activity ids: {:?}", player.account_id, player.game_ver, request.ActivityLevelIds);

        let idx = thread_rng().gen_range(0, request.ActivityLevelIds.len());
        
        let session: GameSession;
        if state.ad_hoc_hosting {
            session = GameSession {
                GameSessionId: 567,
                RegionId: state.ad_hoc_region.to_owned(),
                RoomId: format!("public {}", request.ActivityLevelIds[idx]),
                EventId: None,
                CreatorPlayerId: None,
                Name: "".to_owned(),
                ActivityLevelId: request.ActivityLevelIds[idx].to_owned(),
                Private: false,
                GameInProgress: false,
                MaxCapacity: 40,
                IsFull: false,
            };
        } else {
            let region = request.RegionPings.iter()
                .min_by(|a, b| a.Ping.cmp(&b.Ping))
                .map(|r| r.RegionId.as_ref())
                .unwrap_or("us");
            let (id, instance) = state.reserve_photon_instance(request.ActivityLevelIds[idx].as_ref(), &player.game_ver, region);
            session = GameSession::from_photon_instance(id, instance, None);
        }

        let update = PresenceUpdate {
            PlayerId: player.account_id,
            IsOnline: true,
            GameSession: session.clone(),
        };

        match state.send_msg_to_subscribed(MSG_UPDATE_PRESENCE, &update, player.account_id) {
            Ok(_) => Json(JoinResponse {
                Result: JoinResponse::SUCCESS,
                GameSession: Some(session),
            }),
            Err(_) => Json(JoinResponse {
                Result: JoinResponse::PLAYER_NOT_ONLINE,
                GameSession: None,
            }),
        }
    }

    #[post("/gamesessions/v2/reportjoinresult", format = "json", data = "<_result>")]
    pub fn report_join_result(_player: Player, _state: State<Global>, _result: Json<JoinResult>) {
        // TODO

        // println!("got join result: {:#?}", _result);
    }

    #[get("/gamesessions/v2/listpublicevents")]
    pub fn list_public_events() -> Json<Vec<GameSession>> {
        // TODO
        Json(vec![])
    }

    #[post("/gamesessions/v2/create", format = "json", data = "<request>")]
    pub fn create(player: Player, state: State<Global>, request: Json<CreateRoomRequest>) -> Json<JoinResponse> {
        let region = request.RegionPings.iter()
            .min_by(|a, b| a.Ping.cmp(&b.Ping))
            .map(|r| r.RegionId.as_ref())
            .unwrap_or("us");

        let session: GameSession;
        if state.ad_hoc_hosting {
            session = GameSession {
                GameSessionId: 567,
                RegionId: state.ad_hoc_region.to_owned(),
                RoomId: format!("custom {}", request.ActivityLevelId),
                EventId: None,
                CreatorPlayerId: None,
                Name: "".to_owned(),
                ActivityLevelId: request.ActivityLevelId.to_owned(),
                Private: false,
                GameInProgress: false,
                MaxCapacity: 40,
                IsFull: false,
            };
        } else {
            // TODO private rooms
            let (id, instance) = state.create_photon_instance(request.ActivityLevelId.as_ref(), player.game_ver.as_ref(), region, false);
            session = GameSession::from_photon_instance(id, instance, Some(player.account_id));
        }

        let update = PresenceUpdate {
            PlayerId: player.account_id,
            IsOnline: true,
            GameSession: session.clone(),
        };

        match state.send_msg_to_subscribed(MSG_UPDATE_PRESENCE, &update, player.account_id) {
            Ok(_) => Json(JoinResponse {
                Result: JoinResponse::SUCCESS,
                GameSession: Some(session),
            }),
            Err(_) => Json(JoinResponse { 
                Result: JoinResponse::PLAYER_NOT_ONLINE, 
                GameSession: None, 
            }),
        }
    }

    #[derive(Serialize, Deserialize)]
    struct PresenceUpdate {
        PlayerId: i32,
        IsOnline: bool,
        GameSession: GameSession,
    }
}