//! Plex API Bindings

mod info;
mod library;

use reqwest::header::{HeaderMap, ACCEPT};
use serde_json::from_str;

pub struct PlexAPI {
    ip: String,
    token: String,
}

impl PlexAPI {
    pub fn new(ip: String, token: String) -> PlexAPI {
        PlexAPI { ip, token }
    }

    pub fn libraries(&self) -> Result<library::MediaContainer, Box<dyn std::error::Error>> {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, "application/json".parse().unwrap());

        let client = reqwest::blocking::Client::new();
        let resp = client.get(format!("http://{}/library/sections", self.ip))
                         .headers(headers)
                         .send()?
                         .text()?;

        let json_obj: library::Container = from_str(&resp)?;

        Ok(json_obj.media_container)
    }

    pub fn info(&self) -> Result<info::MediaContainer, Box<dyn std::error::Error>> {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, "application/json".parse().unwrap());

        let client = reqwest::blocking::Client::new();
        let resp = client.get(format!("http://{}/", self.ip)).headers(headers).send()?.text()?;

        let json_obj: info::Container = from_str(&resp)?;

        Ok(json_obj.media_container)
    }
}
