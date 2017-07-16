extern crate rusqlite;

use self::rusqlite::Connection;

use database::get_total_unread_item;

pub struct Channel {
    pub id: i32,
    pub title: Option<String>,
    pub link: String,
}

impl Channel {

    pub fn new(id: i32, link: String, title: Option<String>) -> Channel {
        Channel {
            id: id,
            link: link,
            title: title
        }
    }

    pub fn title(&self) -> &str {
        /* If no title return the link */
        match self.title.as_ref() {
            Some(title) => title,
            None => self.link.as_ref()
        }
    }

    pub fn get_total_item_unread(&self, db_connection: &Connection) -> i32 {
        get_total_unread_item(db_connection, self.id)
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
