pub mod rss {
    use String;
    use chrono::{DateTime, Utc};
    use async_trait::async_trait;
    use rss::Error;

    pub mod feeder;
    pub mod telegram_bot;
    pub mod reader;
    pub mod repositories;
    pub mod writer;
    pub mod filter;

    pub struct RssItem {
        pub url: String,
        pub title: String,
        pub created_date: DateTime<Utc>,
        pub description: Option<String>
    }

    #[derive(Debug)]
    pub struct UserRss {
        pub user_id: i64,
        pub subscribes: Vec<String>
    }

    impl UserRss  {
        fn new(user_id: i64, subscribes: Vec<String>) -> Self {
            Self{user_id, subscribes }
        }
    }

    #[async_trait]
    pub trait RssRep {
        fn get_rss(&self, url: &str) -> Result<Box<dyn Iterator<Item=RssItem>>, Error>;
    }
    
    #[async_trait]
    pub trait RssWriter {
        async fn write(&self, user_id: i64, item: RssItem);
        async fn write_error(&self, user_id: i64, error_text: String);
    }
    
    #[async_trait]
    pub trait UserRssRepository {
        fn add_subscribe(&self, user_id: i64, subscribe: String) -> Result<(), String>;
        fn rm_subscribe(&self, user_id: i64, subscribe: &String) -> Result<(), String>;
        fn get_user_list(&self) -> Vec<UserRss>;
        fn get_user_subscribes(&self, user_id: i64) -> Option<Vec<String>>;
    }

    pub trait UserRssItemsFilter {
        fn filter(&self, user: i64, rep: &String, item: &RssItem) -> bool;
    }
}