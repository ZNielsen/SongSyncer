// Copyright ©️  Zach Nielsen 2022
use serde::Deserialize;

const API_ROOT: &'static str = "https://api.spotify.com/v1";

pub mod responses {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct ApiToken {
        pub access_token: String,
        pub token_type:   String,
        pub expires_in:   u64,
    }

    #[derive(Deserialize, Debug)]
    pub struct ErrorWrapper {
        pub error: Error,
    }

    #[derive(Deserialize, Debug)]
    pub struct Error {
        pub status: i64,
        pub message: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct AuthError {
        pub error: String,
        pub error_description: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct Tracks {
        pub href: String,
        pub items: Vec<super::Track>,
        pub limit: i64,
        pub next: Option<String>,
        pub offset: i64,
        pub previous: Option<String>,
        pub total: i64,
    }
}

pub struct ClientKeys {
    pub id:     String,
    pub secret: String,
}

#[derive(Deserialize, Debug)]
pub struct Track {

}


/// https://developer.spotify.com/documentation/general/guides/authorization/use-access-token/
/// https://developer.spotify.com/documentation/web-api/reference/#/operations/get-users-saved-tracks
pub fn get_new_liked_tracks(spotify: &ClientKeys, token: &responses::ApiToken) -> Vec<Track> {
    let mut ret_vec = Vec::<Track>::new();

    let client = reqwest::blocking::Client::new();
    let mut all_new = true;
    let mut offset = 0;
    let limit = 20;
    while all_new {
        // Get newest tracks
        let uri = format!("{}/{}?limit={}&offset={}&json=true", API_ROOT, "me/tracks", limit, offset);
        let response = client
            .get(&uri)
            .header("Authorization", format!("{} {}", token.token_type, token.access_token))
            .send();
        println!("raw response: {:?}", response);
        let response = parse_response(&uri, response);
        println!("raw response: {:?}", response);
        let tracks = response.json::<responses::Tracks>().expect("response to be deserializable");

        println!("DEBUG: Tracks: {:?}", tracks);
        panic!("checking output");

        // Compare against saved list
        // Add new tracks to return list
        // if all new, do another query
    }

    ret_vec
}

pub fn get_authorize_url(spotify: &ClientKeys) {
    println!("Authorize at: {}?client_id={}&response_type=token&redirect_uri={}&scope={}&json=true",
                "https://accounts.spotify.com/authorize", &spotify.id, "http://localhost:7777/callback", "user-library-read" );
}

/// https://developer.spotify.com/documentation/general/guides/authorization/client-credentials/
pub fn get_token(spotify: &ClientKeys) -> responses::ApiToken {
    // let uri = format!("{}/{}?{}&json=true", API_ROOT, "api/token", "grant_type=client_credentials");
    let uri = format!("{}/{}", API_ROOT, "api/token");
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(&uri)
        .body("grant_type=authorization_code")
        .header("Authorization", format!("Basic {}:{}", base64_url::encode(&spotify.id), base64_url::encode(&spotify.secret)))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send();
    let response = parse_response(&uri, response);
    response.json::<responses::ApiToken>().expect("response to be deserializable")
}

fn parse_response(uri: &str, resp: Result<reqwest::blocking::Response, reqwest::Error>) -> reqwest::blocking::Response {
    match resp {
        Err(e) => {
            panic!("Error with reqwest on uri [{}]: {}", uri, e);
        },
        Ok(response) => {
            println!("uri response: [{}]: {}", uri, response.status());
            if response.status() != reqwest::StatusCode::OK {
                println!("raw response: {:?}", response);
                let err: responses::ErrorWrapper = response.json().expect("response to be deserializable");
                panic!("Error from spotify: {}: {}", err.error.status, err.error.message);
            }

            response
        }
    }
}

