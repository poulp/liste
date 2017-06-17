extern crate clap;

use clap::ArgMatches;

const SETTINGS_FILE: &str = "~/.config/liste/settings.yml";

pub struct Settings {
    pub settings_file: String
}

impl Settings {
    pub fn new(matches: ArgMatches) -> Result<Settings, &'static str> {

        let settings_file = String::from(matches.value_of("settings")
            .unwrap_or(SETTINGS_FILE));

        Ok(Settings {
            settings_file: settings_file
        })
    }
}
