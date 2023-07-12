use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use crate::{
    resources::scene_data::{
        charactor::RaceType,
        thing::ThingType,
    },
    scenes::game_scenes::tilemap::tile::{CoverType, GroundType}
};

#[derive(Deserialize, Clone, Debug)]
pub enum BiomeType {
    Plain,
    Desert,
    Forest,
    Rocks,
    Tropic,
    Snow,
    Swamp,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Biome {
    pub main_ground: GroundType,
    pub main_cover: CoverType,
    pub main_cover_filling: u8,
    pub additional_ground: HashMap<GroundType, f32>,
    pub additional_cover: HashMap<CoverType, f32>,
    pub rivers: Rivers,
    pub spots: Spots,
    pub objects: BiomeObjects,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum RiverType {
    Horizontal,
    Vertical,
    Random,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Rivers {
    pub liquid_river: Vec<RiverSetting>,
    pub solid_river: Vec<RiverSetting>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Spots {
    pub liquid_spot: Vec<SpotSetting>,
    pub solid_spot: Vec<SpotSetting>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SpotSetting {
    pub amount: u8,
    pub emerging: u8,
    pub ground_type: GroundType,
    pub cover_type: CoverType,
    pub max_width: u16,
    pub max_height: u16,
    pub min_width: u16,
    pub min_height: u16,
    pub x_offset: i8,
    pub y_offset: i8,
    pub height_offset: i8,
    pub width_offset: i8,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RiverSetting {
    pub emerging: u8,
    pub ground_type: GroundType,
    pub cover_type: CoverType,
    pub max_width: u16,
    pub min_width: u16,
    pub offset: i8,
    pub offset_width: i8,
    pub river_type: RiverType,
}

#[derive(Deserialize, Clone, Debug)]
pub struct BiomeCharacters {
    monster_race: Vec<RaceType>,
    npc_race: Vec<RaceType>,
    
}

#[derive(Deserialize, Clone, Debug)]
pub struct BiomeObjects {
    pub things: BiomeThings,
    pub charactors: BiomeCharacters,
}

#[derive(Deserialize, Clone, Debug)]
pub struct BiomeThings {
    pub natural_things: HashMap<ThingType, f32>,
}

#[derive(Deserialize, Debug)]
pub struct GameSceneBiomeDeploy {
    plain: Biome,
}

impl GameSceneBiomeDeploy {
    pub fn new(path: &str) -> Self {
        let result: GameSceneBiomeDeploy = match File::open(path) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                serde_json::from_str(&contents).expect("JSON was not well-formatted")
            }
            Err(err) => panic!("Can not open biome data file: {}, {}", err, path),
        };

        return result;
    }

    pub fn get_biome_setting(&self, biome_type: &BiomeType) -> &Biome {
        match biome_type {
            BiomeType::Plain => return &self.plain,
            _ => panic!(
                "Ground_scene_biome_deploy.get_biome_setting. Biome: '{:?}' Not created yet!",
                biome_type
            ),
            //BiomeType::Desert => { return &self.desert },
            //BiomeType::Forest => { return &self.forest },
            //BiomeType::Rocks => { return &self.rocks },
            //BiomeType::Snow => { return &self.snow },
            //BiomeType::Swamp => { return &self.swamp },
            //BiomeType::Tropic => { return &self.tropic },
        }
    }
}
