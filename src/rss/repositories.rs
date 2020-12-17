use super::UserRss;
use std::collections::HashMap;
use crate::rss::UserRssRepository;
use async_trait::async_trait;


struct InMemoryUserRepository {
    users: HashMap<i64, UserRss>
}

#[async_trait]
impl UserRssRepository for InMemoryUserRepository {
    async fn add_subscribe(&mut self, user_id: i64, subscribe: String) -> Result<(), String>{
        let user_rss = match self.users.get_mut(&user_id) {
            Some(user) => user,
            None => {
                self.users.insert(user_id, UserRss { user_id, subscribes: Vec::new() });
                self.users.get_mut(&user_id).unwrap()
            }
        };
        user_rss.add_subscribe(subscribe);
        Ok(())
    }

    async fn get_user_list(&self) -> Vec<&UserRss> {
        return self.users.values().collect()
    }
}