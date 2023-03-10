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
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(AUTHORIZATION, format!("Bearer {}", auth_token))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<APIResponse>().await {
                Ok(parsed) => print_tracts(parsed.tracks.items.iter().collect()),
                Err(_) => println!("the response didn't match the struct / shape")
            }
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Need to grab a new token")
        }
    }

}
