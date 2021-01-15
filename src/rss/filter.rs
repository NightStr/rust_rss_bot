use std::collections::HashMap;
use chrono::{DateTime, Utc};
use crate::rss::{UserRssItemsFilter, RssItem};
use std::cell::RefCell;

pub struct FilterByLastRequestData {
    last_request_cache: RefCell<HashMap<String, DateTime<Utc>>>
}

impl FilterByLastRequestData {
    pub fn new() -> Self {
        FilterByLastRequestData { last_request_cache: RefCell::new(HashMap::new()) }
    }
}

impl UserRssItemsFilter for FilterByLastRequestData {
    fn filter(&self, user: i64, rep: &String, items: Vec<RssItem>) -> Vec<RssItem> {
        let key = format!("{} {}", user, rep);
        let mut cache = self.last_request_cache.borrow_mut();
        let last_request = cache.insert(key, Utc::now());
        if let Some(last_request) = last_request {
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
    }
}
