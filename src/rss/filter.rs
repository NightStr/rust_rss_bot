use std::collections::HashMap;
use chrono::{DateTime, Utc, FixedOffset};
use crate::rss::{UserRssItemsFilter, RssItem};
use std::cell::RefCell;
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
    fn filter(&self, user: i64, rep: &String, items: Vec<RssItem>) -> Vec<RssItem> {
        let key = format!("{} {}", user, rep);
        let r = self.last_request_cache.write(|db| {
            let last_request_str = db.insert(key, Utc::now().to_rfc2822());
            if let Some(last_request_str) = last_request_str {
                let last_request: DateTime<Utc> = DateTime::parse_from_rfc2822(&last_request_str).unwrap().into();
                let mut r = vec![];
                for item in items {
                    if item.created_date > last_request {
                        r.push(item);
                    }
                }
                r
            } else {
                items
            }
        }).unwrap();
        self.last_request_cache.save();
        r
    }
}
