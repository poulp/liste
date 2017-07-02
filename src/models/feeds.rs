pub struct Feed {
    pub title: String,
    pub description: String
}

impl Feed {
    pub fn new(title: String, description: String) -> Feed {
        Feed {
            title: title,
            description: description
        }
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

    pub fn add_feed(&mut self, feed: Feed) {
        self.feeds.push(feed);
    }

    pub fn clear(&mut self) {
        self.feeds.clear();
    }
}