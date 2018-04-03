
def init_db(connection, urls):
    connection.execute(
        """CREATE TABLE IF NOT EXISTS channel (
        channel_id      INTEGER PRIMARY KEY,
        url             TEXT UNIQUE ON CONFLICT IGNORE,
        title           TEXT,
        description     TEXT) """)
    connection.commit()

    # Create channels
    if urls:
        urls = map(lambda x: (x,), urls)
        connection.executemany("INSERT INTO channel (url) VALUES (?)", urls)
        connection.commit()

    # TODO remove old channels


def get_channels(connection):
    result = connection.execute("SELECT url, title, description FROM channel")
    return result
