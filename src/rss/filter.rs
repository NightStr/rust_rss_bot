use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};
use crate::rss::{UserRssItemsFilter, RssItem};
use rustbreak::FileDatabase;
use rustbreak::deser::Ron;

pub struct FilterByLastRequestData {
    last_request_cache: FileDatabase<HashMap<String, String>, Ron>
}

impl FilterByLastRequestData {
    pub fn new() -> Self {
        FilterByLastRequestData {
            last_request_cache: FileDatabase::load_from_path_or_default("./shown_cache.db").unwrap()
        }
    }
}

impl UserRssItemsFilter for FilterByLastRequestData {
    fn filter(&self, user: i64, rep: &String, item: &RssItem) -> bool {
        let key = format!("{} {}", user, rep);
        let last_request = match  self.last_request_cache.read(|db| {
            db.get(&key).map(|v| DateTime::parse_from_rfc2822(v).unwrap().into())
        }).unwrap() {
            Some(v) => v,
            None => Utc::now() - Duration::days(2)
        };

        if last_request < item.created_date {
            self.last_request_cache.write(|db| {
                db.insert(key, item.created_date.to_rfc2822());
            }).unwrap();
            self.last_request_cache.save().unwrap();
            true
        } else {
            false
        }
    }
}
