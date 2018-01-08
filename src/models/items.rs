extern crate rusqlite;

use self::rusqlite::Connection;

use::database::set_iteam_as_read;

pub struct Item {
    pub id: i32,
    pub link: String,
    pub title: String,
    pub description: String,

    pub is_read: bool
}

impl Item {

    pub fn set_as_read(&mut self, db_connection: &Connection, is_read: bool) {
        let is_read_int = match is_read {
            true => 1,
            false => 0
        };

        match set_iteam_as_read(db_connection, self.id, is_read_int) {
            Ok(_) => {
                self.is_read = is_read;
            },
            Err(_) => {}
        }
    }
}
