pub mod rss {
    use rss;
    use chrono::{DateTime, Utc};
    use telegram_bot::*;
    use futures::StreamExt;
    use String;
    use std::collections::HashMap;
    use regex::Regex;

    pub type Item = rss::Item;

    pub struct RssItemsGetter {
        channel: rss::Channel
    }

    impl RssItemsGetter {
        pub fn new(url: &str) -> RssItemsGetter {
            RssItemsGetter { channel: rss::Channel::from_url(url).unwrap() }
        }
        pub fn get_rss_items(&self, filtered_date: DateTime<Utc>) -> impl Iterator<Item=&rss::Item> {
            self.channel.items().iter().filter(move |i| {
                match i.pub_date() {
                    Some(pub_date) => match DateTime::parse_from_rfc2822(pub_date) {
                        Ok(d) if d > filtered_date => true,
                        _ => false
                    },
                    _ => false
                }
            })
        }
    }

    enum CommandType {
        Add
    }
    enum MessageType {
        Command {
            command: CommandType,
            params: String
        },
        WrongCommand {
            message: String
        },
        Message {
            message: String
        }
    }

    pub struct TelegramBot {
        api: Api,
        feeds: HashMap<i64, Vec<String>>
    }

    impl TelegramBot {
        pub fn new<T: Into<String> >(token: T) -> Self {
            let mut bot = TelegramBot {
                api: Api::new(token.into()),
                feeds: HashMap::new()
            };
            bot.fill_feeds();
            bot
        }

        fn fill_feeds(&mut self) {
        }

        fn add_feed<T: Into<String>>(&mut self, user_id: i64, url: T) {
            self.feeds.entry(user_id).or_insert(Vec::new()).push(url.into());
            println!("{:?}", self.feeds);
        }

        fn parse_message(data: &String) -> MessageType {
            let re = Regex::new(r"/(?P<command>\w*) (?P<params>\w*)").unwrap();
            match re.captures(data) {
                Some(cap) => {
                    match cap {
                        cap if cap["command"] == "add".to_string() && cap["params"].len() > 0 => {
                            MessageType::Command{
                                command: CommandType::Add,
                                params: cap["params"].to_string()}
                        },
                        _ => {
                            MessageType::WrongCommand {message: data.clone()}
                        }
                    }
                },
                None => {
                    MessageType::Message {message: data.clone()}
                }
            }
        }

        pub async fn run(&mut self) -> Result<(), &'static str> {
            let mut stream = self.api.stream();
            while let Some(update) = stream.next().await {
                // If the received update contains a new message...
                let update = update.expect("Failed to update");
                if let UpdateKind::Message(message) = update.kind {
                    if let MessageKind::Text { ref data, .. } = message.kind {
                        println!("<{}>: {}", &message.from.first_name, data);
                        match Self::parse_message(data) {
                            MessageType::Command{command: c, params: p} => {
                                match c {
                                    CommandType::Add => dbg!(self.add_feed(message.from.id.into(), p))
                                }
                            },
                            _ => {
                                self.api.send(message.text_reply(format!(
                                    "Hi, {}! You just wrote '{}'. Your ID: {}",
                                    &message.from.first_name, data, message.from.id
                                ))).await.unwrap();
                            }
                        };
                    }
                }
            };
            Ok(())
        }
    }
}