use rss;
use chrono::{DateTime, Utc};
use super::{RssRep, RssItem};
use async_trait::async_trait;
use rss::Error;
use html2text::from_read;


pub struct RssItemsGetter {
}

pub struct RssGetterResult {
    channel: rss::Channel,
    curr: usize
}

impl RssItemsGetter {
    pub fn new() -> RssItemsGetter {
        RssItemsGetter {}
    }
}

impl RssGetterResult {
    pub fn new(url: &str) -> Self {
        let channel = rss::Channel::from_url(url).unwrap();
        let curr = channel.items().len() - 1;
        Self {channel, curr}
    }
}


impl Iterator for RssGetterResult {
    type Item = RssItem;

    fn next(&mut self) -> Option<Self::Item> {
        match self.channel.items().get(self.curr) {
            Some(item) => {
                self.curr -= 1;
                Some(RssItem {
                    url: String::from(item.link().unwrap_or_default()),
                    title: String::from(item.title().unwrap_or_default()),
                    created_date: DateTime::parse_from_rfc2822(
                        item.pub_date().unwrap_or_default()
                    ).unwrap().with_timezone(&Utc),
                    description: item.description().map(|description| from_read(description.as_bytes(), description.len()))
                })
            },
            None => None
        }
    }
}

#[async_trait]
impl RssRep for RssItemsGetter {
    fn get_rss(&self, url: &str) -> Result<Box<dyn Iterator<Item=RssItem>>, Error> {
        dbg!(url);
        Ok(Box::new(RssGetterResult::new(url)))
    }
}
