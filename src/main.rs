// Copyright ©️  Zach Nielsen 2022

use std::collections::{HashMap, BTreeMap};
use std::io::BufRead;

mod spotify;
mod lastfm;


fn main() {
    // Set up API keys and the like
    let mut secrets_map = HashMap::<String, String>::new();
    let file = std::fs::File::open("secrets").unwrap();
    let reader = std::io::BufReader::new(file);
    for line in reader.lines().map(|line| line.unwrap()) {
        let parts: Vec<&str> = line.split(':').collect();
        secrets_map.insert(parts[0].to_owned(), parts[1].to_owned());
    }

    let spotify = spotify::ClientKeys {
        id:     secrets_map.get("spotify_client_id").unwrap().to_owned(),
        secret: secrets_map.get("spotify_client_secret").unwrap().to_owned(),
    };
    let lastfm = lastfm::ApiKeys {
        api_key: secrets_map.get("lastfm_api_key").unwrap().to_owned(),
        secret:  secrets_map.get("lastfm_shared_secret").unwrap().to_owned(),
    };
    println!("spotify: id: {}, secret: {}", spotify.id, spotify.secret);

    // Query for any new liked songs
    let token = spotify::get_token(&spotify);
    let liked_songs = spotify::get_new_liked_tracks(&spotify, &token);

    //
    // Post new liked songs to Last.FM
    // https://www.last.fm/api/show/track.love
    //
    let artist = String::new();
    let track  = String::new();
    let mut map = BTreeMap::new();
    map.insert("method".to_owned(), "track.love".to_owned());
    map.insert("artist".to_owned(), artist);
    map.insert("track".to_owned(), track);
    map.insert("sk".to_owned(), lastfm::get_session_key(&lastfm));

    let _response = lastfm::get_response(&lastfm::construct_uri(&lastfm, &mut map));
}

