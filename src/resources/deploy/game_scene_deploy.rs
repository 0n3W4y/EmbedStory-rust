use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;

use crate::resources::deploy::game_scene_biome_deploy::BiomeType;

#[derive(Deserialize, Debug)]
pub struct GameSceneConfig {
    pub biome_type: BiomeType,
    pub width: u16,
    pub height: u16,
    pub underground: u8,       // percent;
    pub underground_floor: u8, // min 0 - max - value;
    pub monsters: u8, //min 0 - max - value;
    pub npc: u8,      //min 0 - max - value;
}

#[derive(Deserialize, Debug)]
pub struct GameSceneDeploy {
    pub plain_event: GameSceneConfig,
}

impl GameSceneDeploy {
    pub fn new(path: &str) -> Self {
        let result: GameSceneDeploy = match File::open(path) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                serde_json::from_str(&contents).expect("JSON was not well-formatted")
            }
            Err(err) => panic!("Can not open ground scene data file: {}, {}", err, path),
        };

        return result;
    }

    pub fn get_scene_setting(&self, biome_type: BiomeType) -> &GameSceneConfig {
        match biome_type {
            BiomeType::Plain => return &self.plain_event,
            /*
            BiomeType::Desert =>{},
            BiomeType::Forest =>{},
            BiomeType::Rocks =>{},
            BiomeType::Snow =>{},
            BiomeType::Swamp =>{},
            BiomeType::Tropic =>{},
            */
            _ => return &self.plain_event,
        }
    }
}
