use super::UserRss;
use std::collections::HashMap;
use crate::rss::UserRssRepository;
use async_trait::async_trait;
use std::cell::RefCell;
use std::borrow::Borrow;
use std::rc::Rc;
use std::fs::read_to_string;


#[derive(Debug)]
pub struct InMemoryUserRepository {
    users: RefCell<HashMap<i64, Rc<UserRss>>>
}

impl InMemoryUserRepository {
    pub fn new() -> InMemoryUserRepository {
        InMemoryUserRepository { users: RefCell::new(HashMap::new()) }
    }
}

#[async_trait]
impl UserRssRepository for InMemoryUserRepository {
    fn add_subscribe(&self, user_id: i64, subscribe: String) -> Result<(), String>{
        let mut users = self.users.borrow_mut();
        println!("Add subscribe {} {}", user_id, subscribe);
        let user_rss = match users.get_mut(&user_id) {
            Some(user) => user,
            None => {
                users.insert(user_id, Rc::new(UserRss { user_id, subscribes: RefCell::new(Vec::new()) }));
                users.get_mut(&user_id).unwrap()
            }
        };
        user_rss.add_subscribe(subscribe);
        dbg!(users.borrow());
        Ok(())
    }

    fn get_user_list(&self) -> Vec<Rc<UserRss>> {
        self.users.borrow().values().map(
            |user_rss| Rc::clone(user_rss)
        ).collect()
    }
    fn rm_subscribe(&self, user_id: i64, subscribe: &String) -> Result<(), String> {
        println!("Remove subscribe {} {}", user_id, subscribe);
        match self.users.borrow_mut().get_mut(&user_id) {
            Some(users) => users.rm_subscribe(subscribe),
            None => return Err(format!("User {} not found", user_id))
        };
        return Ok(())
    }
}