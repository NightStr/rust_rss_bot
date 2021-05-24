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
    async fn write_error(&self, user_id: i64, error_text: String) {
        println!("Was error {}: {}", user_id, error_text);
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

impl<'a> TelegramWriter<'a> {
    async fn write_request(&self, request: SendMessage<'_>) {
        if let Err(e) = self.api.send(request).await {
            println!("Failed to write a message in telegram due error {}", e);
        }
    }
}

#[async_trait]
impl<'a> RssWriter for TelegramWriter<'a> {
    async fn write(&self, user_id: i64, rss_items: Vec<RssItem>) {
        for chunk in rss_items.chunks(10) {
            for item in chunk {
                let message_title = format!("[{}]({})", item.title, item.url);
                let message = match &item.description {
                    Some(description) => format!("{}\n\n{}", message_title, description),
                    None => message_title
                };
                let mut request = SendMessage::new(ChatRef::Id(user_id.into()), message);
                request.parse_mode(ParseMode::Markdown);
                self.write_request(request).await;
            }
        }
    }
    async fn write_error(&self, user_id: i64, error_text: String) {
        let mut request = SendMessage::new(ChatRef::Id(user_id.into()), error_text);
        request.disable_preview();
        request.parse_mode(ParseMode::Markdown);
        self.write_request(request);
    }
}
