use std::env;
use feed_bot::rss::TelegramBot;

use dotenv::dotenv;


#[tokio::main]
async fn main() {
    dotenv().ok();
    let mut telegram_bot = TelegramBot::new(
        env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set")
    );
    telegram_bot.run().await.unwrap();
}
