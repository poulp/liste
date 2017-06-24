extern crate rusqlite;

use self::rusqlite::Connection;

use models::subscriptions::Subscription;
use models::subscriptions::ListSubscriptions;
use models::feeds::Feed;
use models::feeds::ListFeeds;

pub fn get_subscriptions(db_connection: &Connection) -> ListSubscriptions {
    let mut subscriptions = ListSubscriptions::new();

    let mut statement = db_connection.prepare("
        SELECT subscription_id, name, url FROM subscription").unwrap();
    let results = statement.query_map(&[], |row| {
        Subscription {
            id: row.get(0),
            name: row.get(1),
            url: row.get(2)
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
                    SELECT content FROM feed WHERE subscription_id = ?").unwrap();
    let rows = statement.query_map(&[&subscription_id], |row| -> String {row.get(0)}).unwrap();
    for row in rows {
        let feed_name = row.unwrap();
        feeds.add_feed(feed_name);
    }
    feeds
}
