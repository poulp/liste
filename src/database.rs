extern crate rusqlite;

use self::rusqlite::Connection;

use settings::Settings;
use models::subscriptions::{Subscription, ListSubscriptions};
use models::feeds::{Feed, ListFeeds};

pub fn init_database(connection: &Connection, settings: &Settings) {
    /* Subscription table */
    connection.execute("
        CREATE TABLE IF NOT EXISTS subscription (
        subscription_id INTEGER PRIMARY KEY,
        url             TEXT UNIQUE ON CONFLICT IGNORE,
        name            TEXT,
        title           TEXT
    )", &[]).unwrap();

    /* Feed table */
    connection.execute("
        CREATE TABLE IF NOT EXISTS feed (
        feed_id         INTEGER PRIMARY KEY,
        title           TEXT,
        description     TEXT,
        is_read         BOOL,
        subscription_id INTEGER,
        FOREIGN KEY(subscription_id) REFERENCES subscription(subscription_id)
    )", &[]).unwrap();

    /* Register new subscriptions */
    for subscription in &settings.subscriptions {
        connection.execute("
            INSERT INTO subscription (url, name) VALUES (?1, ?2)",
                           &[subscription, subscription]).unwrap();
    }

    /* Purge old subscriptions */
    let mut stmt = connection.prepare("SELECT url FROM subscription").unwrap();
    let rows = stmt.query_map(&[], |row| -> String {row.get(0)}).unwrap();
    for row in rows {
        let url = row.unwrap();
        if !settings.subscriptions.iter().any(|x| x == &url) {
            connection.execute("DELETE FROM subscription WHERE url = ?", &[&url]).unwrap();
        }
    }
}

pub fn get_subscriptions(db_connection: &Connection) -> ListSubscriptions {
    let mut subscriptions = ListSubscriptions::new();

    let mut statement = db_connection.prepare("
        SELECT subscription_id, name, url, title FROM subscription").unwrap();
    let results = statement.query_map(&[], |row| {
        Subscription {
            id: row.get(0),
            name: row.get(1),
            url: row.get(2),
            title: row.get(3),
            total_feed_unread: 12
        }
    }).unwrap();
    for subscription in results {
        subscriptions.add_subscription(subscription.unwrap());
    }
    subscriptions
}

pub fn get_feeds_from_subscription(db_connection: &Connection, subscription_id: i32) -> ListFeeds {
    let mut feeds = ListFeeds::new();
    let mut statement = db_connection.prepare("
                    SELECT title, description, is_read FROM feed WHERE subscription_id = ?").unwrap();
    let rows = statement.query_map(&[&subscription_id], |row| {
        Feed {
            title: row.get(0),
            description: row.get(1),
            is_read: row.get(2)
        }
    }).unwrap();
    for row in rows {
        feeds.add_feed(row.unwrap());
    }
    feeds
}

pub fn create_feed(db_connection: &Connection, title: &str,
                   description: &str, subscription_id: i32) {
    db_connection.execute(
        "INSERT INTO feed (title, description, subscription_id, is_read) VALUES (?, ?, ?, ?)",
        &[&title, &description, &subscription_id, &false]
    ).unwrap();
}