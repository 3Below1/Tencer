use rocket_contrib::databases::diesel::{prelude::*, insert_into};
use diesel::Queryable;

pub mod accounts;
pub mod schema;
pub mod config;

#[database("tencer_data")]
pub struct TencerData(SqliteConnection);
