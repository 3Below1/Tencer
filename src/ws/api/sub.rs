use crate::ws::{
    ApiRequest, 
    PlayerMap,
    api::ApiError,
};

use serde::{
    Serialize, 
    Deserialize,
};

pub mod v1 {
    use super::*;

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize)]
    struct UpdateParams {
        PlayerIds: Vec<i32>,
    }

    pub fn update(req: ApiRequest, caller: i32, players: &PlayerMap) -> Result<(), ApiError> {
        let params = match req.param {
            Some(params) => match serde_json::from_value::<UpdateParams>(params) {
                Ok(params) => params,
                Err(_) => return Err(ApiError::InvalidParams),
            },
            None => return Err(ApiError::NoParams),
        };
        
        let mut lock = players.lock().unwrap();
        if !lock.contains_key(&caller) {
            return Err(ApiError::CallerNotFound);
        }

        for p in lock.iter_mut() {
            // remove subbed_by entry in all players
            p.1.subbed_by.retain(|s| *s != caller);
            // add subbed_by entry for the players in params
            if params.PlayerIds.iter().any(|id| *id == *p.0) {
                p.1.subbed_by.push(caller);
            }
        }

        Ok(())
    }
}