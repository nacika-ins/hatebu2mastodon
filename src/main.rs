#![deny(warnings)]

// std
use std::env;
use std::fs::File;
use std::io::prelude::*;
// use std::thread::sleep;
// use std::time::Duration;

#[macro_use]
extern crate serde_derive;
use mammut::Mastodon;

// mammut
extern crate mammut;
use mammut::status_builder::StatusBuilder;
use mammut::status_builder::Visibility::Public;
mod send_mstdn;


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
use model::MastodonKey;
use model::Config;

extern crate toml;
extern crate queryst;
use queryst::parse;
use std::sync::{Arc, Mutex};

// url
extern crate url;
// use url::Url;
// use url::percent_encoding::utf8_percent_encode;

use std::process::exit;

const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;


///
/// main
///
fn main() {


    let mastodon_opt: Option<Mastodon> = send_mstdn::try().expect("main");
    if mastodon_opt.is_none() {
        exit(0);
    }
    let mastodon: Mastodon = mastodon_opt.expect("get mastodon");

    let args: Vec<String> = env::args().collect();
    let port = args.get(1).expect("get port");
    println!("port {}", port);
    let port = option_env!("PORT").unwrap_or(port);


    // Load Config
    let mut f = File::open("config.toml").unwrap();
    let mut toml = String::new();
    let _ = f.read_to_string(&mut toml);
    let config: Config = toml::from_str(&toml).expect("open config");
    let apikey = config.hatena.apikey;

    // Start Server
    let mut router = Router::new();
    router.post("/_/hatebu/", callback, "hatebu");
    let mut chain = Chain::new(router);
    chain.link_before(perRead::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
    chain.link_before(perRead::<ApiKey>::one(apikey));
    chain.link_before(perRead::<MastodonKey>::one(Arc::new(Mutex::new(mastodon))));

    Iron::new(chain).http(format!("localhost:{}", port)).unwrap();

}

///
/// Hatena Bookmark Callback
///
fn callback(request: &mut Request) -> IronResult<Response> {
    let apikey = request.get::<perRead<ApiKey>>().unwrap();
    let mastodon = request.get::<perRead<MastodonKey>>().unwrap();
    let mastodon = match mastodon.lock() {
        Ok(mastodon) => mastodon,
        Err(err) => err.into_inner(),
    };

    println!("apikey --> {}", apikey);
    let link = parse_link(request).unwrap();

    if apikey.to_string() == link.apikey {

        println!("--> APIキーが一致しています");
        println!("--> {:?}", link);

        if !link.is_private {

            let comment: String = if link.comment == "" {
                "".to_owned()
            } else {
                let re = Regex::new(r"\[.+?\]").unwrap();
                let comment = link.comment.replace("+", " ");
                let result = re.replace_all(&comment, "");
                let comment = result.to_string();
                if comment == "" {
                    "".to_owned()
                } else {
                    format!("{} / ", comment.trim())
                }
            };
            let tags: Vec<String> = link.tags.iter().map(|x| format!(" #{}", x) ).collect();
            let message = format!("{}{} {}{}", comment, link.title.replace("+", " "), link.url, tags.join(""));
            println!("message: {}", message);
            let status = StatusBuilder {
                              status: message.into(),
                              in_reply_to_id: None,
                              media_ids: None,
                              sensitive: None,
                              spoiler_text: None,
                              visibility: Some(Public),
                         };
            let result = mastodon.new_status(status);
            println!("{:?}", result);

        }


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

                                // is_private
                                let is_private = obj.get("is_private")
                                    .unwrap()
                                    .as_str()
                                    .unwrap()
                                    .clone()
                                    .to_string();
                                let is_private = is_private == "1";
                                println!("is_private --> {:?}", is_private);

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
                                    is_private: is_private
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
