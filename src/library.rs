
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
    #[serde(rename = "MediaContainer")]
    pub media_container: MediaContainer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MediaContainer {
    pub size: i32,
    #[serde(rename = "allowSync")]
    pub allow_sync: bool,
    pub title1: String,
    #[serde(rename = "Directory")]
    pub directory: Vec<Library>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Library {
    #[serde(rename = "allowSync")]
    pub allow_sync: bool,
    pub art: String,
    pub composite: String,
    pub filters: bool,
    pub refreshing: bool,
    pub thumb: String,
    pub key: String,
    pub r#type: String,
    pub title: String,
    pub agent: String,
    pub scanner: String,
    pub language: String,
    pub uuid: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: u64,
    #[serde(rename = "createdAt")]
    pub created_at: u64,
    #[serde(rename = "scannedAt")]
    pub scanned_at: u64,
    pub content: bool,
    pub directory: bool,
    #[serde(rename = "contentChangedAt")]
    pub content_changed_at: u64,
    pub hidden: i32,
    #[serde(rename = "Location")]
    pub location: Vec<Location>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub id: i32,
    pub path: String,
}
