#![allow(non_snake_case)]

use crate::Player;
use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};

pub mod v3 {
    use super::*;

    #[derive(Serialize, Deserialize)]
    pub struct Event {
        EventId: i64,
        Name: String,
        Description: String,
        StartTime: i64,
        EndTime: i64,
        PosterImageName: String,
        CreatorPlayerId: i64,
    }

    #[get("/events/v3/list")]
    pub fn list(_player: Player) -> Json<Vec<Event>> {
        Json(vec![])
    }
}