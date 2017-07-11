extern crate rusqlite;

use self::rusqlite::Connection;

use database::get_total_unread_feed;

pub struct Subscription {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub title: Option<String>,
}

impl Subscription {

    pub fn new(id: i32, name: String, url: String, title: Option<String>) -> Subscription {
        Subscription {
            id: id,
            name: name,
            url: url,
            title: title
        }
    }

    pub fn title(&self) -> &str {
        match self.title.as_ref() {
            Some(title) => title,
            None => self.name.as_ref()
        }
    }

    pub fn get_total_feed_unread(&self, db_connection: &Connection) -> i32 {
        get_total_unread_feed(db_connection, self.id)
    }
}

pub struct ListSubscriptions {
    pub subscriptions: Vec<Subscription>,
}

impl ListSubscriptions {

    pub fn new() -> ListSubscriptions {
        ListSubscriptions {
            subscriptions:vec![],
        }
    }

    pub fn add_subscription(&mut self, subscription: Subscription) {
        self.subscriptions.push(subscription);
    }
}
