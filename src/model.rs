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
    //observers: Vec<&'a Window>
}

impl ListModel {

    pub fn new() -> ListModel {
        ListModel {
            //observers: vec![],
            subscriptions:vec![],
        }
    }

    pub fn add_feed(&mut self, feed_name: String) {
        let feed = Subscription::new(feed_name);
        self.subscriptions.push(feed);
        //self.notify_observers();
    }

//    fn register_observer(&mut self, window: &'a Window) {
//        self.observers.push(window);
//    }

//    fn notify_observers(&self) {
//        for obs in &self.observers {
//            obs.on_notify(self);
//        }
//    }

}