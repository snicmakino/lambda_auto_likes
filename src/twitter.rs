extern crate twitter_api as api;

use oauth::Token;
use std::convert::AsRef;
use std::io;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub access_key: String,
    pub access_secret: String,
}

pub struct Twitter<'a> {
    pub consumer: Token<'a>,
    pub access: Token<'a>,
}

impl<'a> Twitter<'a> {
    pub fn new(
        consumer_key: String,
        consumer_secret: String,
        access_key: String,
        access_secret: String,
    ) -> Twitter<'a> {
        Twitter {
            consumer: Token::new(consumer_key, consumer_secret),
            access: Token::new(access_key, access_secret),
        }
    }

    fn console_input(&mut self, prompt: &str) -> String {
        println!("{} : ", prompt);
        let mut line = String::new();
        let _ = io::stdin().read_line(&mut line).unwrap();
        line.trim().to_string()
    }

    pub fn search(&mut self) {}

    pub fn twitter(&mut self) {
        loop {
            let make_your_choice = self.console_input("What do you want to do?");

            match make_your_choice.as_ref() {
                "update status" => {
                    let status = self.console_input("What's happening?");
                    api::update_status(&self.consumer, &self.access, &status).unwrap();
                }
                "get timeline" => {
                    let ts = api::get_last_tweets(&self.consumer, &self.access).unwrap();
                    if ts.is_empty() {
                        println!("No tweet in your timeline...");
                    } else {
                        for t in ts {
                            println!("{} - {}", t.created_at, t.text)
                        }
                    }
                }
                _ => {
                    println!("Bye!");
                    break;
                }
            }
        }
    }
}
