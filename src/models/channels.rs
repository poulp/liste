extern crate rusqlite;

use self::rusqlite::Connection;

use database::get_total_unread_feed;

pub struct Channel {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub title: Option<String>,
}

impl Channel {

    pub fn new(id: i32, name: String, url: String, title: Option<String>) -> Channel {
        Channel {
            id: id,
            name: name,
            url: url,
            title: title
        }
    }

    pub fn title(&self) -> &str {
        match self.title.as_ref() {
            Some(title) => title,
            None => self.name.as_ref()
        }
    }

    pub fn get_total_feed_unread(&self, db_connection: &Connection) -> i32 {
        get_total_unread_feed(db_connection, self.id)
    }
}

pub struct ListChannels {
    pub channels: Vec<Channel>,
}

impl ListChannels {

    pub fn new() -> ListChannels {
        ListChannels {
            channels: vec![],
        }
    }

    pub fn add_channel(&mut self, channel: Channel) {
        self.channels.push(channel);
    }
}
