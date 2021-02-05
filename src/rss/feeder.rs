use super::{RssRep, RssWriter, UserRssRepository};
use crate::rss::UserRssItemsFilter;
use std::cell::{RefCell, Ref};

pub struct RssGetter<'a> {
    rss_rep: &'a dyn RssRep,
    rss_writer: &'a dyn RssWriter,
    user_rss_getter: &'a dyn UserRssRepository,
    filter: &'a dyn UserRssItemsFilter
}

impl<'a> RssGetter<'a> {
    pub fn new(
        rss_rep: &'a dyn RssRep,
        rss_writer: &'a dyn RssWriter,
        rss_reader: &'a dyn UserRssRepository,
        filter: &'a dyn UserRssItemsFilter
    ) -> Self {
        RssGetter {rss_rep, rss_writer, user_rss_getter: rss_reader, filter }
    }

    pub async fn work(&self) {
        loop {
            for user in self.user_rss_getter.get_user_list() {
                for url in user.get_subsribes().iter() {
                    match self.rss_rep.get_rss(url.as_str()) {
                        Ok(rss_list) => {
                            self.rss_writer.write(
                                user.user_id,
                                self.filter.filter(
                                    user.user_id, url, rss_list
                                )
                            ).await
                        },
                        Err(e) => {
                            self.rss_writer.write_error(
                                user.user_id,
                                format!("При обработке {} произошла ошибка {}. Ссылка была удалена из подписок.", url, e)
                            ).await;
                            self.user_rss_getter.rm_subscribe(user.user_id, url);
                        }
                    };
                }
            }
            tokio::time::delay_for(tokio::time::Duration::from_millis(10000)).await;
        }
    }
}
