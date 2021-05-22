use telegram_bot::*;
use regex::Regex;
use futures::StreamExt;
use crate::rss::UserRssRepository;

use strum::IntoEnumIterator;
use strum_macros::{EnumIter, ToString};


#[derive(EnumIter, ToString)]
enum CommandType {
    Add,
    Del,
    List,
    Help,
}

trait Command<'a> {
    fn new(rss_rep: &'a dyn UserRssRepository) -> Self where Self: Sized;
    fn run(&self, user_id: i64, params: String) -> String;
    fn help(&self) -> String;
}

struct DummyCommand;

impl<'a> Command<'a> for DummyCommand {
    fn new(_rss_rep: &'a dyn UserRssRepository) -> Self {
        Self{}
    }

    fn run(&self, _user_id: i64, _params: String) -> String {
        "Я просто кукла, чего вы от меня хотите?".to_string()
    }

    fn help(&self) -> String {
        "Я просто кукла, я ничем не могу помочь".to_string()
    }
}

struct AddCommand<'a>{
    rss_rep: &'a dyn UserRssRepository
}

impl<'a> Command<'a> for AddCommand<'a> {
    fn new(rss_rep: &'a dyn UserRssRepository) -> Self {
        Self{rss_rep}
    }

    fn run(&self, user_id: i64, params: String) -> String {
        match self.rss_rep.add_subscribe(user_id, params.into()) {
            Ok(_) => "Успешно добавлен".to_string(),
            Err(_) => format!("Не удалось добавить. {}", self.help()),
        }
    }

    fn help(&self) -> String {
        "Чтобы добавить подписку на рассылку введите: /add ссылка на rss".to_string()
    }
}

struct DelCommand<'a>{
    rss_rep: &'a dyn UserRssRepository
}

impl<'a> Command<'a> for DelCommand<'a> {
    fn new(rss_rep: &'a dyn UserRssRepository) -> Self {
        Self{rss_rep}
    }

    fn run(&self, user_id: i64, params: String) -> String {
        match self.rss_rep.rm_subscribe(user_id, &params.into()) {
            Ok(_) => "Успешно удалено".to_string(),
            Err(_) => "Удалить не удалось".to_string(),
        }
    }

    fn help(&self) -> String {
        "Чтобы удалить подписку на рассылку введите: /del ссылка на rss".to_string()
    }
}

struct ListCommand<'a>{
    rss_rep: &'a dyn UserRssRepository
}

impl<'a> Command<'a> for ListCommand<'a> {
    fn new(rss_rep: &'a dyn UserRssRepository) -> Self {
        Self{rss_rep}
    }

    fn run(&self, user_id: i64, _params: String) -> String {
        if let Some(subscribes) = self.rss_rep.get_user_subscribes(user_id) {
            subscribes.join("\n")
        } else {
            "Я не нашел подписок :(".to_string()
        }
    }

    fn help(&self) -> String {
        "Чтобы вывести список активных подписок введите: /list".to_string()
    }
}

struct HelpCommand<'a>{
    rss_rep: &'a dyn UserRssRepository
}

impl<'a> Command<'a> for HelpCommand<'a> {
    fn new(rss_rep: &'a dyn UserRssRepository) -> Self {
        Self{rss_rep}
    }

    fn run(&self, _user_id: i64, _params: String) -> String {
        CommandType::iter()
        .map(|command_type| format!(
            "{} -- {}",
            command_type.to_string(),
            CommandFactory::new(command_type).produce(self.rss_rep).help()
        ))
        .collect::<Vec<String>>()
        .join("\n")
    }

    fn help(&self) -> String {
        "Чтобы получить помощь по командам введите: /help".to_string()
    }
}

struct CommandFactory {
    command_type: CommandType,
}

impl CommandFactory {
    pub fn new(command_type: CommandType) -> Self {
        Self{command_type}
    }
    pub fn produce<'a>(&'a self, rss_rep: &'a dyn UserRssRepository) -> Box<dyn Command + 'a> {
        match self.command_type {
            CommandType::Add => Box::new(AddCommand::new(rss_rep)),
            CommandType::Del => Box::new(DelCommand::new(rss_rep)),
            CommandType::List => Box::new(ListCommand::new(rss_rep)),
            CommandType::Help => Box::new(HelpCommand::new(rss_rep)),
        }
    }
}

enum MessageType {
    Command {
        command: CommandType,
        params: String
    },
    WrongCommand,
    Message
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

    fn parse_message(data: &String) -> MessageType {
        let re = Regex::new(r"/(?P<command>\w*) ?(?P<params>.*)").unwrap();
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
                    }, cap if cap["command"] == "list".to_string() && cap["params"].len() == 0 => {
                        dbg!("{:?}", &cap);
                        MessageType::Command{
                            command: CommandType::List,
                            params: "".to_string()}
                    }, cap if cap["command"] == "help".to_string() && cap["params"].len() == 0 => {
                        dbg!("{:?}", &cap);
                        MessageType::Command{
                            command: CommandType::Help,
                            params: "".to_string()}
                    },
                    _ => {
                        MessageType::WrongCommand
                    }
                }
            },
            None => {
                MessageType::Message
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
                     let reply_message = message.text_reply(match Self::parse_message(data) {
                        MessageType::Command{command: c, params: p} =>
                            CommandFactory::new(c)
                            .produce(self.rss_rep)
                            .run(message.from.id.into(),p),
                        _ => "Привет, я тебя не понимаю, попробуй /help".to_string()
                    });
                    self.api.send(reply_message).await.expect("Failed to send message");
                }
            }
        };
        Ok(())
    }
}