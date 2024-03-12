#![allow(non_snake_case)]

use crate::Player;
use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};

pub mod v1 {
    use super::*;

    #[derive(Serialize, Deserialize)]
    #[allow(dead_code)]
    pub struct BanDetails {
        ReportCategory: i32,
        Duration: i32,
        GameSessionId: i64,
        Message: String,
    }

    #[get("/PlayerReporting/v1/moderationBlockDetails")]
    pub fn ban_details(_player: Player) -> Json<BanDetails> {
        Json(BanDetails {
            ReportCategory: 0,
            Duration: 0,
            GameSessionId: 0,
            Message: "".to_owned(),
        })
    }
}