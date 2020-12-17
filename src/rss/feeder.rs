use super::{RssItem, RssRep, RssWriter, UserRssRepository};
use core::pin::Pin;

struct RssGetter {
    rss_rep: Box<dyn RssRep>,
    rss_writer: Box<dyn RssWriter>,
    user_rss_getter: Pin<Box<dyn UserRssRepository>>
}

impl RssGetter {
    pub fn new(
        rss_rep: Box<dyn RssRep>,
        writer: Box<dyn RssWriter>,
        rss_reader: Pin<Box<dyn UserRssRepository>>
    ) -> Self {
        RssGetter {rss_rep: rss_rep, rss_writer: writer, user_rss_getter: rss_reader }
    }

    pub async fn work(&self) {
        while true {
            for user in self.user_rss_getter.get_user_list().await {
                for url in &user.subscribes {
                    for rss in self.rss_rep.get_rss(url.as_str()).await {
                        self.rss_writer.write(user.user_id, rss.title.as_str());
                    }
                }
            }
            tokio::time::delay_for(tokio::time::Duration::from_millis(10000)).await;
        }
    }
}
