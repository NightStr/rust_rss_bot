use super::{RssRep, RssWriter, UserRssRepository};
use std::rc::Rc;

pub struct RssGetter {
    rss_rep: Box<dyn RssRep>,
    rss_writer: Box<dyn RssWriter>,
    user_rss_getter: Rc<dyn UserRssRepository>
}

impl RssGetter {
    pub fn new(
        rss_rep: Box<dyn RssRep>,
        writer: Box<dyn RssWriter>,
        rss_reader: Rc<dyn UserRssRepository>
    ) -> Self {
        RssGetter {rss_rep: rss_rep, rss_writer: writer, user_rss_getter: rss_reader }
    }

    pub async fn work(&self) {
        loop {
            for user in self.user_rss_getter.get_user_list() {
                for url in &user.subscribes {
                    for rss in self.rss_rep.get_rss(url.as_str()) {
                        self.rss_writer.write(user.user_id, rss.title.as_str()).await;
                    }
                }
            }
            tokio::time::delay_for(tokio::time::Duration::from_millis(10000)).await;
        }
    }
}
