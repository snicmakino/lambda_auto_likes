extern crate lambda_auto_likes;
extern crate rust_apex;
#[macro_use]
extern crate serde_json;
extern crate failure;

use std::env;
use lambda_auto_likes::twitter;
use failure::{Compat, Error};
use serde_json::Value;
use rust_apex::Context;

fn main() {
    rust_apex::run::<_, _, Compat<Error>, _>(|_: Value, _: Context| {
        let mut tw = twitter::Twitter::new(
            get_env("TWITTER_CONSUMER_KEY"),
            get_env("TWITTER_CONSUMER_SECRET"),
            get_env("TWITTER_ACCESS_KEY"),
            get_env("TWITTER_ACCESS_SECRET"),
        );

        let mut count = 0;
        for keyword in get_env("SEARCH_KEYWORDS").split(",") {
            count += tw.auto_likes(keyword);
        }

        Ok(json!({
            "like_count": count
        }))
    });
}

fn get_env(key: &str) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(_e) => panic!("environment variable not found : {:?}", key),
    }
}