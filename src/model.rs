struct Subscription {
    name: String,
}

impl Subscription {

    fn new(name: String) -> Subscription {
        Subscription {
            name: name
        }
    }
}

struct ListModel<'a> {
    subscriptions: Vec<&'a Subscription>,
    //observers: Vec<&'a Window>
}

impl<'a> ListModel<'a> {

    fn new() -> ListModel<'a> {
        ListModel {
            //observers: vec![],
            subscriptions:vec![],
        }
    }

    fn add_feed(&mut self, feed: &'a Subscription) {
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