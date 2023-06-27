use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

use crate::resources::scene_data::charactor::Charactor;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Profile {
    pub last_save: String,
    pub playtime: i64,
    pub profile_name: String,
    pub start_time: String,
    pub end_time: String,
    pub charactor: Option<Charactor>,
    pub companion: Option<Charactor>,
}

impl Profile {
    fn default() -> Self {
        let start_time: DateTime<Local> = Local::now();

        return Profile {
            profile_name: String::new(),
            last_save: String::new(),
            playtime: 0,
            start_time: start_time.to_rfc3339(),
            end_time: String::new(),
            charactor: None,
            companion: None,
        };
    }

    pub fn set_name(&mut self, name: String) {
        self.profile_name = name;
    }

    pub fn save_profile(&mut self) {
        self.end_time = Local::now().to_rfc3339();
        let end_time = DateTime::parse_from_rfc3339(self.end_time.clone().as_str())
            .expect("Can not parse time");
        let start_time = DateTime::parse_from_rfc3339(self.start_time.clone().as_str())
            .expect("Can not parse time");
        let diff_time = end_time - start_time;
        let playtime = diff_time.num_seconds();
        self.playtime = playtime;
        self.last_save = Local::now().to_rfc3339();
        let mut profile_file = File::create("profile.json").expect("Can not create profile file");
        let profile_file_str: String = serde_json::to_string(&self).unwrap();
        profile_file
            .write(profile_file_str.as_bytes())
            .expect("Can not to write profile file");
    }

    pub fn load_profile(&mut self) {
        let profile: Profile = match File::open("profile.json") {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                serde_json::from_str(&contents).expect("JSON was not well-formatted")
            }
            Err(_err) => {
                let mut setting_file =
                    File::create("profile.json").expect("Can not create profile file");
                let setting_str: String = serde_json::to_string(&Profile::default()).unwrap();
                setting_file
                    .write(setting_str.as_bytes())
                    .expect("Can not write file");
                Profile::default()
            }
        };
        self.last_save = profile.last_save;
        self.playtime = profile.playtime;
        self.profile_name = profile.profile_name;
    }
}
