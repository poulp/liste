import curses
import time
import sqlite3

import toml
import feedparser

from db import init_db, get_channels


class Window:

    def __init__(self, height, width, starty, startx):
        self.height = height
        self.width = width
        self.starty = starty
        self.startx = startx

        self.window = curses.newwin(height, width, starty, startx)

    def draw(self, app):
        self.window.refresh()


class TopBar:

    def __init__(self, height, width, starty, startx):
        self.height = height
        self.width = width
        self.starty = starty
        self.startx = startx

        self.window = curses.newwin(height, width, starty, startx)

    def draw(self, app):
        self.window.addstr(0, 0, "topbar")
        self.window.refresh()


class BottomBar:

    def __init__(self, height, width, starty, startx):
        self.height = height
        self.width = width
        self.starty = starty
        self.startx = startx

        self.window = curses.newwin(height, width, starty, startx)

    def draw(self, app, txt):
        self.window.addstr(0, 0, txt)
        self.window.refresh()


class WindowChannels:

    def __init__(self, height, width, starty, startx):
        self.height = height
        self.width = width
        self.starty = starty
        self.startx = startx
        self.windows = []
        self.index = 0
        self.init_windows()

    def init_windows(self):
        for i in range(self.height):
            self.windows.append(curses.newwin(1, self.width, i + self.starty, self.startx))

    def draw(self, app):
        for index, window in enumerate(self.windows):
            try:
                channel = app.channels[index]
                window.addstr(0, 0, channel.url)
                window.refresh()
            except IndexError:
                pass
        self.draw_active_item()

    def draw_active_item(self):
        self.windows[self.index].bkgd(curses.color_pair(1))
        self.windows[self.index].refresh()

    def draw_next_item(self, app):
        if len(app.channels) > self.index:
            self.windows[self.index].bkgd(curses.color_pair(0))
            self.windows[self.index].refresh()
            self.index += 1
            self.draw(app)

    def draw_previous_item(self, app):
        if self.index > 0:
            self.windows[self.index].bkgd(curses.color_pair(0))
            self.windows[self.index].refresh()
            self.index -= 1
            self.draw(app)


class Channel:

    def __init__(self, url, title, description):
        self.url = url
        self.title = title
        self.description = description


class Application:

    def __init__(self, stdscr, connection):
        lines, cols = stdscr.getmaxyx()
        self.connection = connection
        self.channels = []
        self.load_channels()

        self.bottom_bar = BottomBar(1, cols, lines - 1, 0)
        self.bottom_bar.window.nodelay(1)
        self.bottom_bar.window.keypad(1)

        self.window_channels = WindowChannels(lines - 2, cols, 1, 0)

        self.top_bar = TopBar(1, cols, 0, 0)

    def load_channels(self):
        raw_channels = get_channels(self.connection)
        for raw_channel in raw_channels:
            self.channels.append(Channel(raw_channel[0], raw_channel[1], raw_channel[2]))

    def synchronize(self):
        self.bottom_bar.draw(self, "synchronize")
        for channel in self.channels:
            channel_parsed = feedparser.parse(channel.url)
            if 'title' in channel_parsed.feed:
                print(channel_parsed.feed.title)

    def loop(self, stdscr):
        self.top_bar.draw(self)
        self.window_channels.draw(self)
        self.bottom_bar.draw(self, "bottom")

        while True:
            cmd = self.bottom_bar.window.getch()
            if cmd == ord('q'):
                break
            if cmd == ord('s'):
                self.synchronize()
            elif cmd == curses.KEY_DOWN:
                self.window_channels.draw_next_item(self)
            elif cmd == curses.KEY_UP:
                self.window_channels.draw_previous_item(self)
            time.sleep(0.01)


def main(stdscr):
    connection = sqlite3.connect('base.db')
    settings_toml = toml.load('/home/alex/.config/liste/settings.toml')
    # TODO handle exception
    urls = settings_toml['channels'].get('urls')
    init_db(connection, urls)

    curses.noecho()  # Don't echo while getch
    curses.start_color()
    curses.use_default_colors()
    curses.curs_set(0)
    curses.init_pair(1, curses.COLOR_RED, curses.COLOR_WHITE)

    app = Application(stdscr, connection)
    app.loop(stdscr)

    connection.close()
    curses.endwin()


if __name__ == '__main__':
    curses.wrapper(main)
