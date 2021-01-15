use std::env;
use feed_bot::rss::telegram_bot::TelegramBot;
use feed_bot::rss::feeder::RssGetter;
use feed_bot::rss::repositories::InMemoryUserRepository;
use feed_bot::rss::writer::ConsoleWriter;
use feed_bot::rss::reader::RssItemsGetter;
use futures::join;

use dotenv::dotenv;
use std::rc::Rc;


#[tokio::main]
async fn main() {
    dotenv().ok();
    let rep = Rc::new(InMemoryUserRepository::new());
    let mut telegram_bot = TelegramBot::new(
        env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set"),
        rep.clone(),
    );
    let rss_getter = RssGetter::new(
        Box::new(RssItemsGetter::new()),
        Box::new(ConsoleWriter::new()),
        rep.clone(),
    );
    join!(telegram_bot.run(), rss_getter.work());
}
