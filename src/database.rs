extern crate rusqlite;

use self::rusqlite::Connection;

use models::subscriptions::Subscription;
use models::subscriptions::ListSubscriptions;

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