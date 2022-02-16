use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetSession {
    pub name: String,
    pub key: String,
    pub subscriber: u64,
}

#[derive(Deserialize)]
pub struct Error {
    pub error: u64,
    pub message: String,
}
