#![allow(non_snake_case)]

use crate::{
    Player,
    data::TencerData,
};
use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};
use diesel::QueryResult;

pub mod charades {
    // use super::*;

    pub mod v1 {
        use super::super::*;
        
        #[derive(Serialize, Deserialize)]
        pub struct Word {
            EN_US: String,
            Difficulty: i32,
        }

        #[get("/activities/charades/v1/words")]
        pub fn words(_player: Player, db: TencerData) -> QueryResult<Json<Vec<Word>>> {
            let values = db.get_json_data("charades_words")?.unwrap_or_else(|| "".to_owned());
            match serde_json::from_str::<Vec<Word>>(&values) {
                Ok(words) => Ok(Json(words)),
                Err(_) => Ok(Json(vec![Word { EN_US: r#"database table json_data row charades_words: [{EN_US: "<WORD>", Difficulty: 0}]"#.to_owned(), Difficulty: 0 }]))
            }
        }
    }
}