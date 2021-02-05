use super::UserRss;
use std::collections::HashMap;
use crate::rss::UserRssRepository;
use async_trait::async_trait;
use std::cell::RefCell;
use std::borrow::Borrow;
use std::rc::Rc;
use std::fs::{read_to_string, copy};


#[derive(Debug)]
pub struct InMemoryUserRepository {
    users_subscribes: RefCell<HashMap<i64, Vec<String>>>
}

impl InMemoryUserRepository {
    pub fn new() -> InMemoryUserRepository {
        InMemoryUserRepository { users_subscribes: RefCell::new(HashMap::new()) }
    }
}

#[async_trait]
impl UserRssRepository for InMemoryUserRepository {
    fn add_subscribe(&self, user_id: i64, subscribe: String) -> Result<(), String> {
        let mut users = self.users_subscribes.borrow_mut();
        println!("Add subscribe {} {}", user_id, subscribe);
        let user_rss = match users.get_mut(&user_id) {
            Some(user) => user,
            None => {
                users.insert(user_id, Vec::new());
                users.get_mut(&user_id).unwrap()
            }
        };
        user_rss.push(subscribe);
        Ok(())
    }

    fn get_user_list(&self) -> Vec<UserRss> {
        return self.users_subscribes.borrow().iter().map(
            |(user, subscribes)| UserRss::new(
                *user,
                subscribes.iter().map(String::from).collect()
            )
        ).collect();
    }

    fn rm_subscribe(&self, user_id: i64, subscribe: &String) -> Result<(), String> {
        println!("Remove subscribe {} {}", user_id, subscribe);
        match self.users_subscribes.borrow_mut().get_mut(&user_id) {
            Some(subscribes) => {
                if let Some(index) =  subscribes.iter().position(|x| x == subscribe) {
                   subscribes.remove(index);
                }
            },
            None => return Err(format!("User {} not found", user_id))
        };
        return Ok(())
    }
}