extern crate rusqlite;

use self::rusqlite::Connection;

use database::get_total_unread_item;

pub struct Channel {
    pub id: i32,
    pub link: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub unread_items: i32
}

impl Channel {

    pub fn new(db_connection: &Connection, id: i32, link: String, title: Option<String>, description: Option<String>) -> Channel {
        let unread_items = get_total_unread_item(db_connection, id);
        Channel {
            id,
            link,
            title,
            description,
            unread_items
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

impl Clone for Channel {
    fn clone(&self) -> Channel {
        Channel {
            id: self.id,
            link: self.link.clone(),
            title: self.title.clone(),
            description: self.description.clone(),
            unread_items: self.unread_items
        }
    }
}
