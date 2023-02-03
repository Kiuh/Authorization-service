use serde::{Deserialize, Serialize};

pub mod get;
pub mod post;

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
