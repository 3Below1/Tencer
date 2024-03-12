#![allow(non_snake_case)]

use rocket::response::content;
use serde::{Serialize, Deserialize};

pub mod config;
pub mod platformlogin;
pub mod playerreporting;
pub mod relationships;
pub mod players;
pub mod messages;
pub mod avatar;
pub mod settings;
pub mod equipment;
pub mod events;
pub mod activities;
pub mod challenge;
pub mod gamesessions;
pub mod images;

#[get("/versioncheck/v3?<v>")]
pub fn versioncheck_v3(v: String) -> content::Json<String> {
    const VALID_VERSIONS: [&'static str; 3] = [
        "20171027_EA",
        "20171103_EA",
        "20171117_EA",
    ];

    let is_valid = VALID_VERSIONS.iter().any(|valid| &v == *valid);
    
    content::Json(format!("{{\"ValidVersion\":{}}}", is_valid))
}

#[derive(Serialize, Deserialize)]
pub struct ResultResponse {
    pub Success: bool,
    pub Message: Option<String>,
}
