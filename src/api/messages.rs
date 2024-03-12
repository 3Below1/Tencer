#![allow(non_snake_case)]

use crate::Player;
use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};

pub mod v2 {
    use super::*;

    #[derive(Serialize, Deserialize)]
    pub struct Message {
        Id: i64,
        FromPlayerId: i64,
        /// seconds since 1970 (utc)
        SentTime: i64,
        Type: i32,
        Data: String,
    }

    #[get("/messages/v2/get")]
    pub fn get(_player: Player) -> Json<Vec<Message>> {
        Json(vec![])
    }
}