pub mod rss {
    use rss;
    use chrono::{DateTime, Utc};
    use telegram_bot;

    pub type Item = rss::Item;

    pub struct RssItemsGetter{
        channel: rss::Channel
    }

    impl RssItemsGetter {
        pub fn new(url: &str) -> RssItemsGetter {
            RssItemsGetter{ channel: rss::Channel::from_url(url).unwrap() }
        }
        pub fn get_rss_items(&self, filtered_date: DateTime<Utc>) -> impl Iterator<Item=&rss::Item> {
            self.channel.items().iter().filter(move |i| {
                match i.pub_date() {
                    Some(pub_date) => match DateTime::parse_from_rfc2822(pub_date) {
                        Ok(d) if d > filtered_date => true,
                        _ => false
                    },
                    _ => false
                }
            })
        }
    }

    pub struct TelegramBot {
        api: telegram_bot::Api
    }
}