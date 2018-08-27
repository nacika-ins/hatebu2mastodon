use iron::typemap::Key;

use mammut::Mastodon;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Link {
    pub url: String,
    pub tags: Vec<String>,
    pub apikey: String,
    pub comment: String,
    pub status: String,
    pub title: String,
    pub username: String,
    pub is_private: bool
}


pub struct ApiKey;
impl Key for ApiKey {
    type Value = String;
}

pub struct MastodonKey;
impl Key for MastodonKey {
    type Value = Arc<Mutex<Mastodon>>;
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub hatena: Hatena
}

#[derive(Debug, Deserialize)]
pub struct Hatena {
    pub apikey: String
}
