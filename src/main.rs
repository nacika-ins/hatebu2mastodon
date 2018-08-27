#![deny(warnings)]

// std
use std::env;
use std::fs::File;
use std::io::prelude::*;
// use std::thread::sleep;
// use std::time::Duration;

#[macro_use]
extern crate serde_derive;

// iron
extern crate iron;
use iron::prelude::*;
use iron::status;
// use iron::typemap::Key;
extern crate bodyparser;
extern crate persistent;

// router
extern crate router;
use router::Router;

// persistent
use persistent::Read as perRead;
// use persistent::Write as perWrite;

// regex
extern crate regex;
use regex::Regex;

// model
mod model;
use model::Link;
use model::ApiKey;
use model::Hatebu;

extern crate toml;
extern crate queryst;
use queryst::parse;

// url
extern crate url;
// use url::Url;
// use url::percent_encoding::utf8_percent_encode;

const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;



fn main() {


    let args: Vec<String> = env::args().collect();
    let port = args.get(1).expect("get port");
    println!("port {}", port);
    let port = option_env!("PORT").unwrap_or(port);


    // Load Config
    let mut f = File::open("config.toml").unwrap();
    let mut toml = String::new();
    let _ = f.read_to_string(&mut toml);
    let value: Hatebu = toml::from_str(&toml).unwrap();
    let apikey = value.apikey;

    // Start Server
    let mut router = Router::new();
    router.post("/_/hatena/", callback, "hatena");
    let mut chain = Chain::new(router);
    chain.link_before(perRead::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
    chain.link_before(perRead::<ApiKey>::one(apikey));

    Iron::new(chain).http(format!("localhost:{}", port)).unwrap();

}


/// Hatena Bookmark Callback
fn callback(request: &mut Request) -> IronResult<Response> {
    let apikey = request.get::<perRead<ApiKey>>().unwrap();
    println!("apikey --> {}", apikey);
    let link = parse_link(request).unwrap();

    if apikey.to_string() == link.apikey {

        println!("--> APIキーが一致しています");
        println!("--> {:?}", link);


    }

    Ok(Response::with(status::Ok))
}



// Hatena Bookmark parse Web hook body
fn parse_link(request: &mut Request) -> Option<Link> {
    // println!("{:?}", request);

    let body = request.get::<bodyparser::Raw>();
    // println!("{:?}", body);

    match body {
        Ok(v) => {
            match v {
                Some(v) => {
                    let object = parse(&*v);
                    match object {
                        Ok(v) => {
                            println!("{:?}", v);
                            if v.is_object() {

                                let obj = v.as_object().unwrap();

                                // Api key
                                let key = obj.get("key")
                                    .unwrap()
                                    .as_str()
                                    .unwrap()
                                    .clone()
                                    .to_string();
                                println!("apikey --> {}", key);

                                // Hatena bookmarked url
                                let url = obj.get("url")
                                    .unwrap()
                                    .as_str()
                                    .unwrap()
                                    .clone()
                                    .to_string();
                                println!("url --> {}", url);

                                // title
                                let title = obj.get("title")
                                    .unwrap()
                                    .as_str()
                                    .unwrap()
                                    .clone()
                                    .to_string();
                                println!("title --> {}", title);

                                // username
                                let username = obj.get("username")
                                    .unwrap()
                                    .as_str()
                                    .unwrap()
                                    .clone()
                                    .to_string();
                                println!("username --> {}", username);

                                // status
                                let status = obj.get("status")
                                    .unwrap()
                                    .as_str()
                                    .unwrap()
                                    .clone()
                                    .to_string();
                                println!("status --> {}", status);

                                // comment
                                let comment = obj.get("comment")
                                    .unwrap()
                                    .as_str()
                                    .unwrap()
                                    .clone()
                                    .to_string();
                                println!("comment --> {}", comment);

                                // tags
                                let mut tags: Vec<String> = Vec::new();
                                let re = Regex::new(r"\[([^\]]+)\]").unwrap();
                                for cap in re.captures_iter(&*comment) {
                                    println!("cap --> {:?}", cap.get(1).map_or(None, |m| Some(m.as_str())));
                                    match cap.get(1).map_or(None, |m| Some(m.as_str())) {
                                        Some(v) => {
                                            tags.push(v.to_string());
                                        }
                                        None => (),
                                    }
                                }

                                Some(Link {
                                    url: url,
                                    tags: tags,
                                    apikey: key,
                                    title: title,
                                    status: status,
                                    comment: comment,
                                    username: username,
                                })

                            } else {
                                None
                            }
                        }
                        Err(_) => None,
                    }
                }
                None => None,
            }
        }
        Err(_) => None,
    }

}
