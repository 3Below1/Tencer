
use std::{
    sync::{Arc, Mutex},
    collections::HashMap,
};

pub type InstanceMap = Arc<Mutex<HashMap<i32, Instance>>>;

#[derive(Clone, Debug)]
pub struct Instance {
    pub region: String,
    pub photon_id: String,
    pub activity_id: String,
    pub in_progress: bool,
    pub game_version: String,
    pub private: bool,
    pub player_count: u32,
    pub capacity: u32,
}