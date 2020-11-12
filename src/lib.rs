pub mod rss {
    use rss;
    use chrono::{DateTime, Utc};
    use telegram_bot::*;
    use futures::StreamExt;
    use String;

    pub type Item = rss::Item;

    pub struct RssItemsGetter {
        channel: rss::Channel
    }

    impl RssItemsGetter {
        pub fn new(url: &str) -> RssItemsGetter {
            RssItemsGetter { channel: rss::Channel::from_url(url).unwrap() }
        }
        pub fn get_rss_items(&self, filtered_date: DateTime<Utc>) -> impl Iterator<Item=&rss::Item> {
            self.channel.items().iter().filter(move |i| {
                match i.pub_date() {
                    Some(pub_date) => match DateTime::parse_from_rfc2822(pub_date) {
                        Ok(d) if d > filtered_date => true,
                        _ => false
                    },
                    _ => false
                }
            })
        }
    }

    pub struct TelegramBot {
        api: Api,
    }

    impl TelegramBot {
        pub fn new<T: Into<String> >(token: T) -> Self
        {
            return TelegramBot {
                api: Api::new(token.into()),
            }
        }

        pub async fn run(self) -> Result<(), &'static str> {
            let mut stream = self.api.stream();
            while let Some(update) = stream.next().await {
                // If the received update contains a new message...
                let update = update.expect("Failed to update");
                if let UpdateKind::Message(message) = update.kind {
                    if let MessageKind::Text { ref data, .. } = message.kind {
                        println!("<{}>: {}", &message.from.first_name, data);
                        self.api.send(message.text_reply(format!(
                            "Hi, {}! You just wrote '{}'",
                            &message.from.first_name, data
                        ))).await;
                    }
                }
            };
            Ok(())
        }
    }
}