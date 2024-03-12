use crate::{
    data::{TencerData},
    Global,
    Player,
    api::ResultResponse,
};
use rocket_contrib::json::Json;
use rocket_contrib::databases::diesel::prelude::*;
use rocket::{
    request::Form,
    State,
};
use tokio_tungstenite::{
    tungstenite::Message,
};

pub mod v1 {
    use super::*;
    use crate::api::platformlogin::v1::Profile;

    #[get("/players/v1/<account_id>")]
    pub fn player(_player: Player, db: TencerData, account_id: i32) -> QueryResult<Option<Json<Profile>>> {
        let result = db.get_account(account_id)?
            .map(|acc| Json(Profile::from_account(&acc)));
        
        Ok(result)
    }

    #[post("/players/v1/objectives")]
    pub fn complete_objectives() {
        // TODO
    }
}

pub mod v2 {
    use super::*;
    
    const MSG_UPDATE_PROFILE: i32 = 11;

    #[derive(FromForm)]
    pub struct DisplayNameRequest {
        Name: String,
    }

    #[post("/players/v2/displayname", format = "form", data = "<data>")]
    pub fn set_display_name(player: Player, state: State<Global>, db: TencerData, data: Form<DisplayNameRequest>) -> QueryResult<Json<ResultResponse>> {
        use crate::api::platformlogin::v1::CachedLogin;

        db.set_display_name(player.account_id, &data.Name)?;
        let acc = CachedLogin::from_account(&db.get_account(player.account_id)?.unwrap());


        let msg = format!("{{\"Id\":{},\"Msg\":{}}}", MSG_UPDATE_PROFILE, serde_json::to_string(&acc).unwrap());
        let mut players = state.player_map.lock().unwrap();
        let targets = players.get_mut(&player.account_id).unwrap().subbed_by.clone();
        for p in targets {
            match players.get_mut(&p).unwrap().ws_tx.start_send(Message::Text(msg.to_owned())) {
                Ok(_) => (),
                Err(_) => return Ok(Json(ResultResponse { Success: false, Message: Some("Failed to send profile update".to_owned()) })),
            }
        }

        Ok(Json(ResultResponse { Success: true, Message: None }))
    }
}