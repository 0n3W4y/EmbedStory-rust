use std::{fs::File, io::Read};

use serde::Deserialize;

use crate::resources::deploy::DEPLOY_NPC_PATH;

#[derive(Deserialize, Debug)]
pub struct NPCDeploy {

}

impl NPCDeploy {
    pub fn new() -> Self {
        let npc_deploy: NPCDeploy = match File::open(DEPLOY_NPC_PATH) {
            Ok(mut file) => {
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                serde_json::from_str(&content).expect("JSON was not well-formatted")
            }
            Err(e) => panic!("Can not open objects data file: {}, {}", e, DEPLOY_NPC_PATH),
        };
        return npc_deploy;
    }
}