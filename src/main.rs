extern crate lambda_auto_likes;

use std::env;
use lambda_auto_likes::twitter;

fn main() {
    let mut tw = twitter::Twitter::new(
        get_env("TWITTER_CONSUMER_KEY"),
        get_env("TWITTER_CONSUMER_SECRET"),
        get_env("TWITTER_ACCESS_KEY"),
        get_env("TWITTER_ACCESS_SECRET"),
    );

    for keyword in get_env("SEARCH_KEYWORDS").split(",") {
        tw.auto_likes(keyword);
    }
}

fn get_env(key: &str) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(_e) => panic!("environment variable not found : {:?}", key),
    }
}