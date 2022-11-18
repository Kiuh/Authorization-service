use serde::{Deserialize, Serialize};

pub mod cells;
pub mod change_name_description;
pub mod create;
pub mod get;
pub mod get_time;
pub mod remove;

#[derive(Serialize, Deserialize)]
pub struct MapJson {
    pub prefab_name: String,
    pub json: String,
}

#[derive(Serialize, Deserialize)]
pub struct FeedTypeJson {
    pub prefab_name: String,
    pub json: String,
}

#[derive(Serialize, Deserialize)]
pub struct SetupTypeJson {
    pub prefab_name: String,
    pub json: String,
}

#[derive(Serialize, Deserialize)]
pub struct LifeTypeJson {
    pub prefab_name: String,
    pub json: String,
}
