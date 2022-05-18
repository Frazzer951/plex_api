use std::env;

use dotenv::dotenv;
use plex_api::{self, PlexAPI};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let plex_token = env::var("PLEX_TOKEN")?;
    let plex_ip = env::var("PLEX_IP")?;

    let plex = PlexAPI::new(plex_ip, plex_token);

    let libraries = plex.libraries()?;

    for library in libraries.directory {
        println!("{}: {}", library.title, library.uuid);
    }

    let info = plex.info()?;

    println!("{:#?}", info);

    Ok(())
}
