pub mod rss {
    use String;
    use chrono::{DateTime, Utc};
    use std::collections::HashMap;
    use async_trait::async_trait;
    use std::pin::Pin;

    pub mod feeder;
    pub mod telegram_bot;
    pub mod reader;
    pub mod repositories;

    pub struct RssItem {
        pub url: String,
        pub title: String,
        pub created_date: DateTime<Utc>
    }
    
    pub struct UserRss {
        pub user_id: i64,
        pub subscribes: Vec<String>
    }

    impl UserRss {
        fn add_subscribe(&mut self, subscribe: String) {
            self.subscribes.push(subscribe)
        }
    }

    #[async_trait]
    pub trait RssRep {
        async fn get_rss(&self, url: &str) -> Vec<RssItem>;
    }
    
    #[async_trait]
    pub trait RssWriter {
        async fn write(&self, user_id: i64, text: &str);
    }
    
    #[async_trait]
    pub trait UserRssRepository {
        async fn add_subscribe(&mut self, user_id: i64, subscribe: String) -> Result<(), String>;
        async fn get_user_list(&self) -> Vec<&UserRss>;
    }

}