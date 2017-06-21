use super::listview::TraitListViewItem;

use std::clone::Clone;

pub struct Subscription {
    pub name: String,
}

impl Subscription {

    pub fn new(name: String) -> Subscription {
        Subscription {
            name: name
        }
    }
}

impl TraitListViewItem for Subscription {
    fn get_name(&self) -> &str {
        self.name.as_ref()
    }
}

impl Clone for Subscription {
    fn clone(&self) -> Self {
        Subscription{
            name: self.name.to_owned()
        }
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

    pub fn add_subscription(&mut self, name: String) {
        let sub = Subscription::new(name);
        self.subscriptions.push(sub);
    }
}

impl Clone for ListSubscriptions {
    fn clone(&self) -> Self {
        ListSubscriptions{
            subscriptions: self.subscriptions.clone()
        }
    }
}