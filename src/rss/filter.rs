use std::collections::HashMap;
use chrono::{DateTime, Utc, FixedOffset, Duration};
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
            let mut r = vec![];
            let last_request: DateTime<Utc> = if let Some(last_request_str) = db.get(&key) {
                DateTime::parse_from_rfc2822(&last_request_str).unwrap().into()
            } else {
                Utc::now() - Duration::days(2)
            };
            let mut max_created_date: Option<DateTime<Utc>> = None;
            for item in items {
                max_created_date = match max_created_date {
                    None => Some(item.created_date.clone()),
                    Some(mcd) if mcd < item.created_date => Some(item.created_date.clone()),
                    Some(mcd) => Some(mcd),
                };
                if last_request < item.created_date {
                    r.push(item);
                }
            }
            if let Some(mcd) = max_created_date {
                db.insert(key, mcd.to_rfc2822());
            }
            r
        }).unwrap();
        self.last_request_cache.save();
        r
    }
}
