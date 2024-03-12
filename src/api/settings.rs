#![allow(non_snake_case)]

use crate::Player;
use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};

pub mod v2 {
    use super::*;

    #[derive(Serialize, Deserialize)]
    pub struct Setting {
        Key: String,
        Value: String,
    }

    #[get("/settings/v2")]
    pub fn get(_player: Player) -> Json<Vec<Setting>> {
        // TODO settings
        Json(vec![ Setting { Key: "Recroom.OOBE".to_owned(), Value: "100".to_owned() }])
    }

    #[post("/settings/v2/set")]
    pub fn set(_player: Player) {
        // println!("got settings update: {}", data);
    }
}