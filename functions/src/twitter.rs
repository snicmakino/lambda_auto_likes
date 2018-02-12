pub extern crate chrono;
pub extern crate tokio_core;
pub extern crate futures;
pub extern crate egg_mode;

use self::tokio_core::reactor;

pub struct Twitter {
    core: reactor::Core,
    handle: reactor::Handle,
    token: egg_mode::Token,
}

impl Twitter {
    pub fn new(
        consumer_key: String,
        consumer_secret: String,
        access_key: String,
        access_secret: String,
    ) -> Self {
        let con_token = egg_mode::KeyPair::new(consumer_key, consumer_secret);
        let access_token = egg_mode::KeyPair::new(access_key, access_secret);

        let core = reactor::Core::new().unwrap();
        let handle = core.handle();

        Twitter {
            core: core,
            handle: handle,
            token: egg_mode::Token::Access {
                consumer: con_token,
                access: access_token,
            },
        }
    }

    pub fn auto_likes(&mut self, keyword: &str) {

        let search_result = self.core
            .run(
                egg_mode::search::search(keyword)
                    .result_type(egg_mode::search::ResultType::Mixed)
                    .count(100)
                    .call(&self.token, &self.handle),
            )
            .unwrap();

        for tweet in &search_result.statuses {
            if let Some(_) = tweet.retweeted_status {
                continue;
            }
            print_tweet(tweet);
            let result = self.core
                .run(egg_mode::tweet::like(tweet.id, &self.token, &self.handle))
                .unwrap();
            print_tweet(&result);
        }
    }
}

pub fn print_tweet(tweet: &egg_mode::tweet::Tweet) {
    if let Some(ref user) = tweet.user {
        println!(
            "{} (@{}) posted at {}",
            user.name,
            user.screen_name,
            tweet.created_at.with_timezone(&chrono::Local)
        );
    }

    if let Some(ref screen_name) = tweet.in_reply_to_screen_name {
        println!("--> in reply to @{}", screen_name);
    }

    if let Some(ref status) = tweet.retweeted_status {
        if let Some(ref user) = status.user {
            println!("Retweeted from {}:", user.name);
        }
        print_tweet(status);
        return;
    } else {
        println!("{}", tweet.text);
    }

    println!("--via {} ({})", tweet.source.name, tweet.source.url);

    if let Some(ref place) = tweet.place {
        println!("--from {}", place.full_name);
    }

    if let Some(ref status) = tweet.quoted_status {
        println!("--Quoting the following status:");
        print_tweet(status);
    }

    if !tweet.entities.hashtags.is_empty() {
        println!("Hashtags contained in the tweet:");
        for tag in &tweet.entities.hashtags {
            println!("{}", tag.text);
        }
    }

    if !tweet.entities.symbols.is_empty() {
        println!("Symbols contained in the tweet:");
        for tag in &tweet.entities.symbols {
            println!("{}", tag.text);
        }
    }

    if !tweet.entities.urls.is_empty() {
        println!("URLs contained in the tweet:");
        for url in &tweet.entities.urls {
            println!("{}", url.expanded_url);
        }
    }

    if !tweet.entities.user_mentions.is_empty() {
        println!("Users mentioned in the tweet:");
        for user in &tweet.entities.user_mentions {
            println!("{}", user.screen_name);
        }
    }

    if let Some(ref media) = tweet.extended_entities {
        println!("Media attached to the tweet:");
        for info in &media.media {
            println!("A {:?}", info.media_type);
        }
    }
}
