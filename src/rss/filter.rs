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
        let r = self.last_request_cache.write(|db| {
            let last_request: DateTime<Utc> = if let Some(last_request_str) = db.get(&key) {
                DateTime::parse_from_rfc2822(&last_request_str).unwrap().into()
            } else {
                Utc::now() - Duration::days(2)
            };
            if last_request < item.created_date {
                db.insert(key, item.created_date.to_rfc2822());
                true
            } else {
                false
            }
        }).unwrap();
        self.last_request_cache.save().unwrap();
        r
    }
}
