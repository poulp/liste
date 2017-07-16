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
    pub channels: Vec<String>
}

impl Settings {
    pub fn new(matches: ArgMatches) -> Result<Settings, String> {
        /* Settings file path */
        let settings_file = matches.value_of("settings")
            .unwrap_or(SETTINGS_FILE);
        let path = Path::new(settings_file);
        let mut channels = vec![];
        /* Open settings file and extract everything */
        match File::open(&path) {
            Ok(file) => {
                /* Load here the list of channels */
                let buffer = BufReader::new(file);
                /* Extract channels links */
                for line in buffer.lines() {
                    let link = line.unwrap();
                    /* Add channel to the model */
                    channels.push(link.to_string());
                }
                /* Return settings */
                Ok(Settings {
                    settings_file: String::from(settings_file),
                    channels: channels
                })
            },
            Err(why) => {
                match path.to_str() {
                    Some(s) => {
                        Err(format!("There is a problem with the links file at {} :\n {}", s, why))
                    },
                    None => {
                        Err(format!("There is a problem with the links file :\n {}", why))
                    }
                }
            },
        }
    }
}
