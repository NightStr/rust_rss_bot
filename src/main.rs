use std::env;
use feed_bot::rss::telegram_bot::TelegramBot;
use futures::join;

use dotenv::dotenv;


#[tokio::main]
async fn main() {
    dotenv().ok();
    let mut telegram_bot = TelegramBot::new(
        env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set")
    );
    join!(telegram_bot.run());
}
