use std::env;

use dotenv::dotenv;
use reqwest::header::{HeaderMap, ACCEPT};
use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Serialize, Deserialize, Debug)]
struct LibrarySections {
    #[serde(rename = "MediaContainer")]
    media_container: MediaContainer,
}

#[derive(Serialize, Deserialize, Debug)]
struct MediaContainer {
    size: i32,
    #[serde(rename = "allowSync")]
    allow_sync: bool,
    title1: String,
    #[serde(rename = "Directory")]
    directory: Vec<Library>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Library {
    #[serde(rename = "allowSync")]
    allow_sync: bool,
    art: String,
    composite: String,
    filters: bool,
    refreshing: bool,
    thumb: String,
    key: String,
    r#type: String,
    title: String,
    agent: String,
    scanner: String,
    language: String,
    uuid: String,
    #[serde(rename = "updatedAt")]
    updated_at: u64,
    #[serde(rename = "createdAt")]
    created_at: u64,
    #[serde(rename = "scannedAt")]
    scanned_at: u64,
    content: bool,
    directory: bool,
    #[serde(rename = "contentChangedAt")]
    content_changed_at: u64,
    hidden: i32,
    #[serde(rename = "Location")]
    location: Vec<Location>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Location {
    id: i32,
    path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let plex_token = env::var("PLEX_TOKEN")?;
    let plex_ip = env::var("PLEX_IP")?;

    println!("IP: {}", plex_ip);
    println!("Token: {}", plex_token);

    // let resp = reqwest::blocking::get(format!("http://{}/library/sections", plex_ip))?.text()?;

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse().unwrap());

    let client = reqwest::blocking::Client::new();
    let resp = client.get(format!("http://{}/library/sections", plex_ip))
                     .headers(headers)
                     .send()?
                     .text()?;

    let json_obj: LibrarySections = from_str(&resp)?;

    // println!("{:#?}", resp);
    println!("{:#?}", json_obj);
    Ok(())
}
