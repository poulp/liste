extern crate ncurses;
extern crate clap;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;

use clap::ArgMatches;

/* Default settings file location */
const SETTINGS_FILE: &str = "~/.config/liste/settings.yml";

pub struct Settings {
    pub settings_file: String,
    pub subscriptions: Vec<String>
}

impl Settings {
    pub fn new(matches: ArgMatches) -> Result<Settings, String> {
        /* Settings file path */
        let settings_file = matches.value_of("settings")
            .unwrap_or(SETTINGS_FILE);
        let path = Path::new(settings_file);
        let mut subscriptions = vec![];
        /* Open settings file and extract everything */
        match File::open(&path) {
            Ok(file) => {
                /* Load here the list of subscriptions */
                let buffer = BufReader::new(file);
                /* Extract feeds urls */
                for line in buffer.lines() {
                    let url = line.unwrap();
                    /* Add subscription to the model */
                    subscriptions.push(url.to_string());
                }
                /* Return settings */
                Ok(Settings {
                    settings_file: String::from(settings_file),
                    subscriptions: subscriptions
                })
            },
            Err(why) => {
                match path.to_str() {
                    Some(s) => {
                        Err(format!("There is a problem with the urls file at {} :\n {}", s, why))
                    },
                    None => {
                        Err(format!("There is a problem with the urls file :\n {}", why))
                    }
                }
            },
        }
    }
}
