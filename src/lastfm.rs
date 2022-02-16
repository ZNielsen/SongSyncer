// Copyright ©️  Zach Nielsen 2022

use std::collections::BTreeMap;
use reqwest::StatusCode;

use crate::responses;

const API_ROOT: &'static str = "http://ws.audioscrobbler.com/2.0";

pub struct ApiKeys {
    pub api_key: String,
    pub secret:  String,
}

/// https://www.last.fm/api/desktopauth#_6-sign-your-calls
pub fn construct_api_signature(lastfm: &ApiKeys, middle_map: &mut BTreeMap<String, String>) -> String {
    let middle_bit = construct_middle_bit(middle_map);
    let key_string = format!("api_key{}{}{}",
                        lastfm.api_key, middle_bit, lastfm.secret);
    format!("{:X}", md5::compute(key_string))
}

fn construct_middle_bit(map: &BTreeMap<String, String>) -> String {
    let mut middle_bit = String::new();
    for (key, val) in map {
        middle_bit += &format!("{}{}", key, val);
    }
    middle_bit
}

pub fn get_token(lastfm: &ApiKeys) -> String {
    let mut map = BTreeMap::new();
    map.insert("method".to_owned(), "auth.getToken".to_owned());
    match reqwest::blocking::get(construct_uri(lastfm, &mut map)) {
        Err(e) => {},
        Ok(s) => {},
    }

    format!("TODO")
}

pub fn construct_uri(lastfm: &ApiKeys, map: &mut BTreeMap<String, String>) -> String {
    let api_sig = construct_api_signature(lastfm, map);
    map.insert("api_sig".to_owned(), api_sig);
    map.insert("format".to_owned(), "json".to_owned());

    let mut uri = format!("{}/?", API_ROOT);
    let mut first = true;
    for (key, val) in map {
        if !first {
            uri += "&";
        }
        uri += &format!("{}={}", key, val);
        first = false;
    }

    uri
}

pub fn get_session_key(lastfm: &ApiKeys) -> String {
    // TODO - check storage

    // TODO - only get this if it's missing or invalid
    // Authentication dance - get token, then session. Token is consumed by creating session.
    let token = get_token(lastfm);
    let mut map = BTreeMap::new();
    map.insert("method".to_owned(), "auth.getSession".to_owned());
    map.insert("token".to_owned(), token);

    let response = get_response(&construct_uri(lastfm, &mut map));
    let resp: responses::GetSession = response.json().expect("response to be deserializable");
    println!("Got session for user {}", resp.name);
    resp.key
}

pub fn get_response(uri: &str) -> reqwest::blocking::Response {
    match reqwest::blocking::get(uri) {
        Err(e) => {
            panic!("Error with reqwest on uri [{}]: {}", uri, e);
        },
        Ok(response) => {
            println!("uri response: [{}]: {}", uri, response.status());
            if response.status() != StatusCode::OK {
                let err: responses::Error = response.json().expect("response to be deserializable");
                println!("Error from lastfm: {}: {}", err.error, err.message);
                panic!("Error from lastfm: {}: {}", err.error, err.message);
            }
            response
        }
    }
}
