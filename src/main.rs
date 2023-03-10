use reqwest;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct ExternalUrls {
    spotify: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Artist {
    name: String,
    external_urls: ExternalUrls
}

#[derive(Serialize, Deserialize, Debug)]
struct Album {
    name: String,
    artists: Vec<Artist>,
    external_urls: ExternalUrls
}

#[derive(Serialize, Deserialize, Debug)]
struct Track {
    name: String,
    href: String,
    popularity: u32,
    album: Album,
    external_urls: ExternalUrls
}

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    tracts: Items<Track>
}

#[derive(Serialize, Deserialize, Debug)]
struct Items<T> {
    items: Vec<T>
}

fn print_tracts(tracks: vec<&Track>) {

}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let search_query = &args[1];
    let auth_token = &args[2];
    let url = format!(
        "https://api.spotify.com/v1/search?q={query}&type=track,artist",
        query = search_query
    );
}
