use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

use crate::resources::deploy::game_scene_biome_deploy::BiomeType;
use crate::resources::scene_data::charactor::RaceType;

use super::DEPLOY_GROUND_SCENE_PATH;

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub enum LocationType {
    Friendly,
    #[default]
    Neutral,
    Agressive,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub enum Location {
    #[default]
    ElvenPlains,
    ElvenForest,
    PlainCavse,
    ForestCaves,
}

#[derive(Deserialize, Debug)]
pub struct GameSceneConfig {
    pub location_type: LocationType,
    pub biome_type: BiomeType,
    pub width: u16,
    pub height: u16,
    pub dungeon: DungeonConfig,
    pub races: Vec<RaceType>,
}

#[derive(Deserialize, Debug)]
pub struct DungeonConfig {
    pub chance: u8,
    pub dungeon_type: Location,
    pub dungeon_floors_max: u8,
}

#[derive(Deserialize, Debug)]
pub struct GameSceneDeploy {
    pub elven_plains: GameSceneConfig,
    pub elven_forest: GameSceneConfig,
    pub plain_caves: GameSceneConfig,
    pub forest_caves: GameSceneConfig,
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
            Location::PlainCavse => &self.plain_caves,
            Location::ForestCaves => &self.forest_caves,
        }
    }
}
