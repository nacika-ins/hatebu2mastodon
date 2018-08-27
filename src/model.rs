use iron::typemap::Key;


#[derive(Debug)]
pub struct Link {
    pub url: String,
    pub tags: Vec<String>,
    pub apikey: String,
    pub comment: String,
    pub status: String,
    pub title: String,
    pub username: String,
}


pub struct ApiKey;
impl Key for ApiKey {
    type Value = String;
}

#[derive(Debug, Deserialize)]
pub struct Hatebu {
    pub apikey: String
}