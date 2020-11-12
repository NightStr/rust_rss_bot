use std::env;
use feed_bot::rss::TelegramBot;


#[tokio::main]
async fn main() {
    let telegram_bot = TelegramBot::new(
        env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set")
    );
    telegram_bot.run().await.unwrap();
}
