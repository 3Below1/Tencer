use crate::{
    Player,
};
use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};

pub mod v1 {
    use super::*;

    #[post("/relationships/v1/bulkignoreplatformusers")]
    pub fn bulk_ignore_platform_users(_player: Player) {
        
    }
}

pub mod v2 {
    use super::*;

    #[derive(Serialize, Deserialize)]
    pub struct Relationship {

    }

    #[get("/relationships/v2/get")]
    pub fn get(_player: Player) -> Json<Vec<Relationship>> {
        Json(vec![])
    }
}
