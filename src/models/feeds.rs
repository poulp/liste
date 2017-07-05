pub struct Feed {
    pub title: String,
    pub description: String,

    pub is_read: bool
}

impl Feed {
    pub fn new(title: String, description: String, is_read: bool) -> Feed {
        Feed {
            title: title,
            description: description,
            is_read: is_read
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