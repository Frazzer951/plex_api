use std::env;

use dotenv::dotenv;
use plex_api::{self, PlexAPI};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let plex_token = env::var("PLEX_TOKEN")?;
    let plex_ip = env::var("PLEX_IP")?;

    let plex = PlexAPI::new(plex_ip, Some(plex_token));

    let json_obj = plex.libraries()?;

    println!("{:#?}", json_obj.media_container.directory);
    Ok(())
}
