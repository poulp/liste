use super::listview::TraitListViewItem;

use std::clone::Clone;

pub struct Subscription {
    pub id: i32,
    pub name: String,
    pub url: String
}

impl Subscription {

    pub fn new(id: i32, name: String, url: String) -> Subscription {
        Subscription {
            id: id,
            name: name,
            url: url
        }
    }
}

impl TraitListViewItem for Subscription {
    fn get_name(&self) -> &str {
        self.name.as_ref()
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
