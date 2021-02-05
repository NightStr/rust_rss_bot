use rss;
use chrono::{DateTime, Utc};
use super::{RssRep, RssItem};
use async_trait::async_trait;
use rss::Error;


pub struct RssItemsGetter {
}

impl RssItemsGetter {
    pub fn new() -> RssItemsGetter {
        RssItemsGetter {}
    }
}

#[async_trait]
impl RssRep for RssItemsGetter {
    fn get_rss(&self, url: &str) -> Result<Vec<RssItem>, Error> {
        dbg!(url);
        let channel = rss::Channel::from_url(url)?;
        let mut r: Vec<RssItem> = Vec::new();
        for item in channel.items().iter() {
            r.push(RssItem{
                url: String::from(item.link().unwrap_or_default()),
                title: String::from(item.title().unwrap_or_default()),
                created_date: DateTime::parse_from_rfc2822(
                    item.pub_date().unwrap_or_default()
                ).unwrap().with_timezone(&Utc),
            });
        }
        Ok(r.into())
    }
}
