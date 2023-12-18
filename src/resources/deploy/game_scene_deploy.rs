use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

use crate::resources::deploy::game_scene_biome_deploy::BiomeType;
use crate::resources::scene_data::charactor::RaceType;

use super::DEPLOY_GROUND_SCENE_PATH;


#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub enum Location {
    #[default]
    ElvenPlains,
    ElvenForest,
    PlainDungeon,
    ForestDungeon,
}

#[derive(Deserialize, Debug)]
pub struct GameSceneConfig {
    pub location: Location,
    pub biome_type: BiomeType,
    pub width: u16,
    pub height: u16,
    pub dungeon_chance: u8,
    pub dungeon_type: Location,
    pub dungeon_floors_max: u8,
    pub races: Vec<RaceType>,
    pub monsters_min: u8,
    pub monsters_max: u8
}

#[derive(Deserialize, Debug)]
pub struct GameSceneDeploy {
    pub elven_plains: GameSceneConfig,
    pub elven_forest: GameSceneConfig,
    pub plain_dungeon: GameSceneConfig,
    pub forest_dungeon: GameSceneConfig,
}

impl GameSceneDeploy {
    pub fn new() -> Self {
        let result: GameSceneDeploy = match File::open(DEPLOY_GROUND_SCENE_PATH) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                serde_json::from_str(&contents).expect("JSON was not well-formatted")
            }
            Err(err) => panic!("Can not open ground scene data file: {}, {}", err, DEPLOY_GROUND_SCENE_PATH),
        };

        return result;
    }

    pub fn get_scene_setting(&self, location: &Location) -> &GameSceneConfig {
        match *location {
            Location::ElvenPlains => &self.elven_plains,
            Location::ElvenForest => &self.elven_forest,
            Location::PlainDungeon => &self.plain_dungeon,
            Location::ForestDungeon => &self.forest_dungeon,
        }
    }
}
