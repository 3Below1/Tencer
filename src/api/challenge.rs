#![allow(non_snake_case)]

use crate::{
    api::ResultResponse,
    Player,
};
use rocket_contrib::json::Json;
// use serde::{Serialize, Deserialize};


pub mod v1 {
    use super::*;

    #[get("/challenge/v1/getCurrent")]
    pub fn current(_player: Player) -> Json<ResultResponse> {
        Json(ResultResponse {
            Success: false,
            Message: None,
        })
    }
}