use super::listview::TraitListViewItem;

use std::clone::Clone;

pub struct Subscription {
    pub name: String,
    pub url: String
}

impl Subscription {

    pub fn new(name: String, url: String) -> Subscription {
        Subscription {
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

impl Clone for Subscription {
    fn clone(&self) -> Self {
        Subscription{
            name: self.name.to_owned(),
            url: self.url.to_owned(),
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

    pub fn add_subscription(&mut self, name: String, url: String) {
        let sub = Subscription::new(name, url);
        self.subscriptions.push(sub);
    }

    pub fn has_subscription(&self, sub_name: &str) -> bool {
        self.subscriptions.iter().any(|x| x.url == sub_name)
    }
}

impl Clone for ListSubscriptions {
    fn clone(&self) -> Self {
        ListSubscriptions{
            subscriptions: self.subscriptions.clone()
        }
    }
}