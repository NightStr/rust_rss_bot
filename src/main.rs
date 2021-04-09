use std::env;
use feed_bot::rss::telegram_bot::TelegramBot;
use feed_bot::rss::feeder::RssGetter;
use feed_bot::rss::repositories::LocalFileDatabase;
use feed_bot::rss::writer::TelegramWriter;
use feed_bot::rss::reader::RssItemsGetter;
use feed_bot::rss::filter::FilterByLastRequestData;
use futures::join;

use dotenv::dotenv;
use telegram_bot::Api;


#[tokio::main]
async fn main() {
    dotenv().ok();
    let rss_item_getter = RssItemsGetter::new();
    let user_rss_rep = LocalFileDatabase::new();
    let rss_filter = FilterByLastRequestData::new();
    let telegram_api = Api::new(env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set"));
    let rss_writer = TelegramWriter::new(&telegram_api);

    let telegram_bot = TelegramBot::new(
        &telegram_api,
        &user_rss_rep,
    );
    let rss_getter = RssGetter::new(
        &rss_item_getter,
        &rss_writer,
        &user_rss_rep,
        &rss_filter
    );
    join!(telegram_bot.run(), rss_getter.work());
}
