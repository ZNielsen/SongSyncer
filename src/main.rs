use std::collections::BTreeMap;

mod responses;
mod lastfm;


struct SpotifyClientKeys {
    id:     String,
    secret: String,
}

fn main() {

    // TODO
    // Set up API keys and the like
    let spotify = SpotifyClientKeys {
        id: "get from secrets".to_owned(),
        secret: "get from secrets".to_owned(),
    };
    let lastfm = lastfm::ApiKeys {
        api_key: "get from secrets".to_owned(),
        secret: "get from secrets".to_owned(),
    };

    // TODO
    // Query for any new liked songs


    //
    // Post new liked songs to Last.FM
    // https://www.last.fm/api/rest
    //
    let artist = String::new();
    let track  = String::new();
    let mut map = BTreeMap::new();
    map.insert("method".to_owned(), "track.love".to_owned());
    map.insert("artist".to_owned(), artist);
    map.insert("track".to_owned(), track);
    map.insert("sk".to_owned(), lastfm::get_session_key(&lastfm));

    let response = lastfm::get_response(&lastfm::construct_uri(&lastfm, &mut map));
}

