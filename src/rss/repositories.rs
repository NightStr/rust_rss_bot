use super::UserRss;
use std::collections::HashMap;
use crate::rss::UserRssRepository;
use async_trait::async_trait;
use std::cell::RefCell;
use rustbreak::{ FileDatabase, deser::Ron };


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

    fn get_user_subscribes(&self, user_id: i64) -> Option<Vec<String>> {
        if let Some(subscribes) = self.users_subscribes.borrow().get(&user_id) {
            let mut r = vec![];
            for sub in subscribes {
                r.push(String::from(sub));
            }
            Some(r)
        } else {
            None
        }
    }
}


pub struct LocalFileDatabase {
    db: FileDatabase<HashMap<i64, Vec<String>>, Ron>
}

impl LocalFileDatabase {
    pub fn new() -> Self {
        LocalFileDatabase { db: FileDatabase::<HashMap<i64, Vec<String>>, Ron>::load_from_path_or_default("./subscribes.db").unwrap() }
    }
}

impl UserRssRepository for LocalFileDatabase {
    fn add_subscribe(&self, user_id: i64, subscribe: String) -> Result<(), String> {
        self.db.write(|db| {
                if let Some(subscribes) = db.get_mut(&user_id) {
                    if subscribes.binary_search(&subscribe).is_err() {
                        subscribes.push(subscribe);
                    } else {
                        dbg!(format!("{} уже добавлена", subscribe));
                    }
                } else {
                    db.insert(user_id, vec![subscribe]);
                }
            }
        ).unwrap();
        self.db.save().unwrap();
        Ok(())
    }

    fn rm_subscribe(&self, user_id: i64, subscribe: &String) -> Result<(), String> {
        self.db.write(|db| {
                if let Some(subscribes) = db.get_mut(&user_id) {
                    if let Some(index) = subscribes.iter().position(|x| x == subscribe) {
                        subscribes.remove(index);
                    }
                }
            }
        ).unwrap();
        self.db.save().unwrap();
        Ok(())
    }

    fn get_user_list(&self) -> Vec<UserRss> {
        self.db.read(|db| {
            db.iter().map(|(user_id, subscribes)| {
                UserRss::new(
                    *user_id,
                    subscribes.iter().map(String::from).collect()
                )
            }).collect()
        }).unwrap()
    }

    fn get_user_subscribes(&self, user_id: i64) -> Option<Vec<String>> {
        self.db.read(|db| {
            if let Some(subscribes) = db.get(&user_id) {
                let mut r = vec![];
                for sub in subscribes {
                    r.push(String::from(sub));
                }
                Some(r)
            } else {
                None
            }
        }).unwrap()
    }
}
