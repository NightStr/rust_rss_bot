use std::env;
use feed_bot::rss::telegram_bot::TelegramBot;
use feed_bot::rss::feeder::RssGetter;
use feed_bot::rss::repositories::InMemoryUserRepository;
use feed_bot::rss::writer::ConsoleWriter;
use feed_bot::rss::reader::RssItemsGetter;
use futures::join;

use dotenv::dotenv;


#[tokio::main]
async fn main() {
    dotenv().ok();
    let rss_item_getter = RssItemsGetter::new();
    let rss_writer = ConsoleWriter::new();
    let user_rss_rep = InMemoryUserRepository::new();
    let mut telegram_bot = TelegramBot::new(
        env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set"),
        &user_rss_rep,
    );
    let rss_getter = RssGetter::new(
        &rss_item_getter,
        &rss_writer,
        &user_rss_rep,
    );
    join!(telegram_bot.run(), rss_getter.work());
}
