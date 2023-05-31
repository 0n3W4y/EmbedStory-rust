use std::collections::HashMap;

pub mod draw;
pub mod cleanup;
pub mod spawn;
pub mod destroeyd_thing_handler;

use serde::{Deserialize, Serialize};

use crate::resources::scene_data::objects::resists::Resist;
use crate::scenes::game_scenes::tilemap::tile::{Position, TilePermissions};

use super::body_part::{BodyPart, BodyPartType};


#[derive( Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default )]
pub enum ThingPermissions{
    CanHarvested,
    CanRepaired,
    #[default]
    CanBeDestroyed,

}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Copy, Default, Hash)]
pub enum ThingType {
    #[default]
    Tree,
    FertileTree,
    Bush,
    FertileBush,
    Rock,
    Boulder,
    Log,
    CopperOre,
    IronOre,
    WoodenWall,
    StoneWall,
    IronWall,
    SteelWall,
    WoodenDoor,
    ReinforcedWoodenDoor,
    IronDoor,
    ReinforcedIronDoor,
    SteelDoor,
    ReinforcedSteelDoor,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ThingConfig {
    pub permissions: Vec<ThingPermissions>,
    pub resists: HashMap<Resist, i16>,
    pub body_structure: HashMap<BodyPartType, i16>,
    pub tile_allow_permissions: Vec<TilePermissions>,
    pub tile_deny_permissions: Vec<TilePermissions>,
    pub tile_movement_ratio: u16
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Thing {
    pub id: usize,
    pub tile_index: usize, // index of tile in vec on tilemap for fast get; because all tiles r static in tilemap vec;
    pub thing_type: ThingType,
    pub position: Position<i32>,
    pub graphic_index: u8, // get image from material_manager;

    pub permissions: Vec<ThingPermissions>,
    pub resists: HashMap<Resist, i16>,
    pub resists_cache: HashMap<Resist, i16>,
    pub body_structure: HashMap<BodyPartType, BodyPart>,

    pub current_health_points: i16,
    pub total_health_points: i16
}
