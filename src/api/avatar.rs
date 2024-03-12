#![allow(non_snake_case)]

use crate::{
    data::TencerData,
    Player,
};
use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};
use rocket::{
    response::content,
};
use diesel::QueryResult;

pub mod v2 {
    use super::*;

    #[derive(Serialize, Deserialize, Default)]
    pub struct Avatar {
        OutfitSelections: String,
        SkinColor: String,
        HairColor: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Gift {
        Id: i64,
        AvatarItemDesc: Option<String>,
        Xp: i32,
        GiftRarity: i32,
        Message: String,
        EquipmentPrefabName: Option<String>,
        EquipmentModificationGuid: Option<String>,
        GiftContext: i32,
    }

    #[get("/avatar/v2")]
    pub fn get(player: Player, db: TencerData) -> QueryResult<Json<Avatar>> {
        let raw = db.get_current_avatar(player.account_id)?;

        match raw {
            Some(raw) => Ok(Json(serde_json::from_str(&raw).unwrap_or_default())),
            None => Ok(Json(Avatar::default())),
        }
    }

    #[post("/avatar/v2/set", format = "json", data = "<avatar>")]
    pub fn set(player: Player, db: TencerData, avatar: Json<Avatar>) -> QueryResult<()> {
        let value = serde_json::to_string(&*avatar).unwrap();
        db.set_current_avatar(player.account_id, &value)
    }

    #[get("/avatar/v2/gifts")]
    pub fn gifts(_player: Player) -> Json<Vec<Gift>> {
        Json(vec![])
    }
}

pub mod v3 {
    use super::*;

    #[derive(Serialize, Deserialize)]
    pub struct UnlockedItem {
        AvatarItemDesc: String,
        UnlockedLevel: i32,
    }

    #[get("/avatar/v3/items")]
    pub fn unlocked(_player: Player) -> content::Json<&'static str> {
        content::Json(include_str!("../../uo.json"))
    }
}