use async_trait::async_trait;
use crate::rss::RssWriter;

pub struct ConsoleWriter{}

impl ConsoleWriter {
    pub fn new() -> ConsoleWriter {
        ConsoleWriter{}
    }
}

#[async_trait]
impl RssWriter for ConsoleWriter {
    async fn write(&self, user_id: i64, text: &str) {
        println!("{}: {}", user_id, text);
    }
}