// Copyright ©️  Zach Nielsen 2022

#![feature(decl_macro)]
#[macro_use] extern crate rocket;

use std::collections::{HashMap, BTreeMap};
use std::io::BufRead;

mod spotify;
mod lastfm;

#[get("/callbacks")]
// fn token_callback(shutdown: rocket::Shutdown) {
fn token_callback() {
    // this will have the token?
    // shutdown.notify();
}

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
    // println!("spotify: id: {}, secret: {}", spotify.id, spotify.secret);
    // println!("lastfm: id: {}, secret: {}", lastfm.id, lastfm.secret);

    //
    // Authenticate through spotify
    //
    // Fire up a rocket server to field the callback
    let config = rocket::config::Config::build(rocket::config::Environment::Development)
        .port(7777)
        .workers(1)
        .unwrap();
    let rocket = rocket::custom(config).mount("/", routes![token_callback]);
    std::thread::spawn(move || {
        let launched = rocket.launch();
    });

    // Get a token
    spotify::get_authorize_url(&spotify);
    std::thread::sleep(core::time::Duration::from_secs(3));
    println!("Press enter when you have logged in...");
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).expect("Failed to read from stdin");

    let token = spotify::responses::ApiToken {
        access_token: secrets_map.get("token").unwrap().to_owned(),
        token_type: "Bearer".to_owned(),
        expires_in: 3600
    };

    // let token = spotify::get_token(&spotify);
    // TODO - Kill rocket once we have the token? Do we need it for other requests?

    // Query for any new liked songs
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

