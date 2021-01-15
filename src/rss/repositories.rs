use super::UserRss;
use std::collections::HashMap;
use crate::rss::UserRssRepository;
use async_trait::async_trait;
use std::cell::RefCell;
use std::borrow::Borrow;
use std::rc::Rc;


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
                users.insert(user_id, Rc::new(UserRss { user_id, subscribes: Vec::new() }));
                users.get_mut(&user_id).unwrap()
            }
        };
        Rc::get_mut(user_rss).unwrap().add_subscribe(subscribe);
        dbg!(users.borrow());
        Ok(())
    }

    fn get_user_list(&self) -> Vec<Rc<UserRss>> {
        let mut v = vec![];
        for user_rss in self.users.borrow().values() {
            v.push(Rc::clone(user_rss));
        }
        v
    }
}