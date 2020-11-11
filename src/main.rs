use chrono::prelude::*;
use chrono::Duration;

use feed_bot::rss::{ RssItemsGetter, Item };

fn main() {
    let v: Vec<Item> = Vec::new();
    let item_getter = RssItemsGetter::new("https://readrust.net/all/feed.rss");
    for item in item_getter.get_rss_items(Utc::now() - Duration::days(10)) {
        println!("{}", item.pub_date().unwrap())
    }
}
