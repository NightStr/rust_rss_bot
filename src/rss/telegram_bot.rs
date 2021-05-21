use telegram_bot::*;
use regex::Regex;
use futures::StreamExt;
use crate::rss::UserRssRepository;
use std::fmt::Display;


enum CommandType {
    Add,
    Del
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
    api: &'a Api,
    rss_rep: &'a dyn UserRssRepository,
}

impl<'a> TelegramBot<'a> {
    pub fn new(api: &'a Api, rss_rep: &'a dyn UserRssRepository) -> Self {
        let bot = TelegramBot {
            api,
            rss_rep
        };
        bot
    }

    async fn add_feed<T: Into<String>>(&self, user_id: i64, url: T) -> String{
        match self.rss_rep.add_subscribe(user_id, url.into()) {
            Ok(s) => "Успешно добавлен".to_string(),
            Err(e) => "Добавить не удалось".to_string(),
        }
    }

    async fn del_feed<T: Into<String>>(&self, user_id: i64, url: T) -> String {
        let inner_url = url.into();
        match self.rss_rep.rm_subscribe(user_id, &inner_url) {
            Ok(_) => "Успешно удалено".to_string(),
            Err(_) => "Удалить не удалось".to_string(),
        }
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
                    }, cap if cap["command"] == "del".to_string() && cap["params"].len() > 0 => {
                        dbg!("{:?}", &cap);
                        MessageType::Command{
                            command: CommandType::Del,
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

    pub async fn run(&self) -> Result<(), &'static str> {
        let mut stream = self.api.stream();
        while let Some(update) = stream.next().await {
            if let UpdateKind::Message(message) = update.expect(
                "Failed to get an update"
            ).kind {
                if let MessageKind::Text { ref data, .. } = message.kind {
                    dbg!(&message, data);
                     self.api.send(message.text_reply(match Self::parse_message(data) {
                        MessageType::Command{command: c, params: p} => {
                            match c {
                                CommandType::Add => dbg!(self.add_feed(message.from.id.into(), p).await),
                                CommandType::Del => dbg!(self.del_feed(message.from.id.into(), p).await)
                            }
                        },
                        _ => {
                            "Привет, я тебя не понимаю, попробуй еще раз".to_string()
                        }
                    })).await.expect("Failed to send message");
                }
            }
        };
        Ok(())
    }
}