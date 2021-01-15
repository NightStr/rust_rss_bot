use telegram_bot::*;
use regex::Regex;
use futures::StreamExt;
use crate::rss::UserRssRepository;


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

pub struct TelegramBot<'a> {
    api: Api,
    rss_rep: &'a dyn UserRssRepository,
}

impl<'a> TelegramBot<'a> {
    pub fn new<T: Into<String> >(token: T, rss_rep: &'a dyn UserRssRepository) -> Self {
        let mut bot = TelegramBot {
            api: Api::new(token.into()),
            rss_rep
        };
        bot.load_feeds();
        bot
    }

    fn load_feeds(&mut self) {
    }

    async fn add_feed<T: Into<String>>(&mut self, user_id: i64, url: T) {
        self.rss_rep.add_subscribe(user_id, url.into()).unwrap();
    }

    fn parse_message(data: &String) -> MessageType {
        let re = Regex::new(r"/(?P<command>\w*) (?P<params>.*)").unwrap();
        match re.captures(data) {
            Some(cap) => {
                match cap {
                    cap if cap["command"] == "add".to_string() && cap["params"].len() > 0 => {
                        dbg!("{:?}", &cap);
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
            if let UpdateKind::Message(message) = update.expect(
                "Failed to get an update"
            ).kind {
                if let MessageKind::Text { ref data, .. } = message.kind {
                    dbg!(&message, data);
                    match Self::parse_message(data) {
                        MessageType::Command{command: c, params: p} => {
                            match c {
                                CommandType::Add => dbg!(self.add_feed(message.from.id.into(), p).await)
                            }
                        },
                        _ => {
                            self.api.send(message.text_reply(format!(
                                "Hi, {}! You just wrote '{}'. Your ID: {}",
                                &message.from.first_name, data, message.from.id
                            ))).await.expect("Failed to send message");
                        }
                    };
                }
            }
        };
        Ok(())
    }
}