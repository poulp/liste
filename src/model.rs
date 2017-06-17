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

pub struct ListModel {
    pub subscriptions: Vec<Subscription>,
}

impl ListModel {

    pub fn new() -> ListModel {
        ListModel {
            subscriptions:vec![],
        }
    }

    pub fn add_feed(&mut self, feed_name: String) {
        let feed = Subscription::new(feed_name);
        self.subscriptions.push(feed);
    }

}