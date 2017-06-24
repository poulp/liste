use super::listview::TraitListViewItem;

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

impl TraitListViewItem for Feed {
    fn get_name(&self) -> &str {
        self.title.as_ref()
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