use async_trait::async_trait;
use crate::rss::{RssWriter, RssItem};
use telegram_bot::{Api, SendMessage, ChatRef, ParseMode};

pub struct ConsoleWriter{}

impl ConsoleWriter {
    pub fn new() -> ConsoleWriter {
        ConsoleWriter{}
    }
}

#[async_trait]
impl RssWriter for ConsoleWriter {
    async fn write(&self, user_id: i64, rss_list: Vec<RssItem>) {
        for rss in rss_list {
            println!("{}: {}", user_id, rss.title);
        }
    }
}

pub struct TelegramWriter<'a> {
    api: &'a Api
}

impl<'a> TelegramWriter<'a> {
    pub fn new(api: &'a Api) -> Self {
        Self{api}
    }
}

#[async_trait]
impl<'a> RssWriter for TelegramWriter<'a> {
    async fn write(&self, user_id: i64, rss_list: Vec<RssItem>) {
        for chunk in rss_list.chunks(10) {
            let message = chunk.iter().map(
                |i| format!("[{}]({})", i.title, i.url)
            ).fold(String::new(), |r, a| format!("{}\n{}", r, a));
            let mut request = SendMessage::new(ChatRef::Id(user_id.into()), message);
            request.disable_preview();
            request.parse_mode(ParseMode::Markdown);
            dbg!(self.api.send(request).await);
        }
    }
}
