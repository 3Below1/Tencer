#![allow(non_snake_case)]

use crate::Player;
use rocket::response::content;
use serde::{Serialize, Deserialize};

pub mod v1 {
    use super::*;
    #[derive(Serialize, Deserialize)]
    pub struct Equipment {
        PrefabName: String,
        ModificationGuid: String,
        UnlockedLevel: i32,
        Favorited: bool,
    }

    #[get("/equipment/v1/getUnlocked")]
    pub fn unlocked(_player: Player) -> content::Json<&'static str> {
        content::Json(include_str!("../../ue.json"))
    }
}