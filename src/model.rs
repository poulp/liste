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

pub struct ListModel<'a> {
    pub subscriptions: Vec<&'a Subscription>,
    //observers: Vec<&'a Window>
}

impl<'a> ListModel<'a> {

    pub fn new() -> ListModel<'a> {
        ListModel {
            //observers: vec![],
            subscriptions:vec![],
        }
    }

    pub fn add_feed(&mut self, feed: &'a Subscription) {
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