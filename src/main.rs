use std::env;

use chrono::prelude::*;
use chrono::Duration;
use tokio::prelude::*;

use feed_bot::rss::{ RssItemsGetter, TelegramBot };


#[tokio::main]
async fn main() {
    let telegram_bot = TelegramBot::new(
        env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set")
    );
    telegram_bot.run().await.unwrap();
}
