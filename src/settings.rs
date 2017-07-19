extern crate clap;
extern crate toml;

use std::io::Read;
use std::fs::File;
use std::path::Path;

use self::toml::Value;
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
            Ok(mut file) => {
                let mut settings_content = String::new();
                file.read_to_string(&mut settings_content).unwrap();
                /* Parse toml */
                let settings = settings_content.parse::<Value>().unwrap();
                // Is there a way to avoid multiple if let like that ?
                if let Some(channels_key) = settings.get("channels") {
                    if let Some(channels_table) = channels_key.as_table() {
                        if let Some(links_key) = channels_table.get("links") {
                            if let Some(links_array) = links_key.as_array() {
                                for link in links_array {
                                    /* Add channel to the model */
                                    channels.push(
                                        String::from(link.as_str().unwrap()));
                                }
                                /* Return settings */
                                Ok(Settings {
                                    settings_file: String::from(settings_file),
                                    channels: channels
                                })
                            } else {
                                Err(format!("Links must be a list"))
                            }
                        } else {
                            Err(format!("You must define a list of RSS/ATOM urls inside the settings file. Example : \n \
                                        [channels]\n \
                                        links = [\"http://mychannel.com/rss\"]"))
                        }
                    } else {
                        Err(format!("Channels must be a table"))
                    }
                } else {
                    Err(format!("You must define a list of RSS/ATOM urls inside the settings file. Example : \n \
                                [channels]\n \
                                links = [\"http://mychannel.com/rss\"]"))
                }
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
