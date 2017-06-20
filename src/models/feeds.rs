use super::listview::TraitListViewItem;

pub struct Feed {
    pub name: String
}

impl Feed {
    pub fn new(name: String) -> Feed {
        Feed {
            name: name
        }
    }
}

impl TraitListViewItem for Feed {
    fn get_name(&self) -> &str {
        self.name.as_ref()
    }
}

pub struct ListFeeds {
    pub feeds: Vec<Feed>
}

impl ListFeeds {
    pub fn new() -> ListFeeds {
        ListFeeds {
            feeds: vec![]
        }
    }

    pub fn add_feed(&mut self, name: String) {
        let feed = Feed::new(name);
        self.feeds.push(feed);
    }
}