use std::{fs::File, io::Read};

use serde::Deserialize;

use crate::resources::deploy::DEPLOY_COMPANION_PATH;

#[derive(Deserialize, Debug)]
pub struct CompanionDeploy {

}

impl CompanionDeploy {
    pub fn new() -> Self {
        let companion_deploy: CompanionDeploy = match File::open(DEPLOY_COMPANION_PATH) {
            Ok(mut file) => {
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                serde_json::from_str(&content).expect("JSON was not well-formatted")
            }
            Err(e) => panic!("Can not open objects data file: {}, {}", e, DEPLOY_COMPANION_PATH),
        };
        return companion_deploy;
    }
}