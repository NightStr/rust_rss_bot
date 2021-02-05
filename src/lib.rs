pub mod rss {
    use String;
    use chrono::{DateTime, Utc};
    use async_trait::async_trait;
    use std::rc::Rc;
    use rss::Error;
    use std::cell::{RefCell, Ref};
    use std::slice::Iter;

    pub mod feeder;
    pub mod telegram_bot;
    pub mod reader;
    pub mod repositories;
    pub mod writer;
    pub mod filter;

    pub struct RssItem {
        pub url: String,
        pub title: String,
        pub created_date: DateTime<Utc>
    }

    #[derive(Debug)]
    pub struct UserRss {
        pub user_id: i64,
        pub subscribes: RefCell<Vec<String>>
    }

    impl UserRss {
        fn add_subscribe(&self, subscribe: String) {
            let mut subscribes = self.subscribes.borrow_mut();
            subscribes.push(subscribe);
        }
        fn rm_subscribe(&self, subscribe: &String) {
            let mut subscribes = self.subscribes.borrow_mut();
            if let Some(index) =  subscribes.iter().position(|x| x == subscribe) {
                subscribes.remove(index);
            }
        }
        fn get_subsribes(&self) -> Vec<String> {
            let mut r: Vec<String> = Vec::new();
            for s in self.subscribes.borrow().iter() {
                r.push(s.into());
            }
            return r
        }
    }

    #[async_trait]
    pub trait RssRep {
        fn get_rss(&self, url: &str) -> Result<Vec<RssItem>, Error>;
    }
    
    #[async_trait]
    pub trait RssWriter {
        async fn write(&self, user_id: i64, rss_list: Vec<RssItem>);
        async fn write_error(&self, user_id: i64, error_text: String);
    }
    
    #[async_trait]
    pub trait UserRssRepository {
        fn add_subscribe(&self, user_id: i64, subscribe: String) -> Result<(), String>;
        fn rm_subscribe(&self, user_id: i64, subscribe: &String) -> Result<(), String>;
        fn get_user_list(&self) -> Vec<Rc<UserRss>>;
    }

    pub trait UserRssItemsFilter {
        fn filter(&self, user: i64, rep: &String, items: Vec<RssItem>) -> Vec<RssItem>;
    }
}