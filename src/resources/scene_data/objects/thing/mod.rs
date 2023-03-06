use std::collections::HashMap;

pub mod draw;
pub mod cleanup;
pub mod spawn;
pub mod despawn;
pub mod destroeyd_thing_handler;

use serde::{Deserialize, Serialize};

use crate::resources::scene_data::objects::body_part::BodyPart;
use crate::resources::scene_data::objects::resists::Resist;
use crate::scenes::game_scenes::tilemap::tile::{Position, TilePermissions};

use super::body_part::BodyPartType;


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

impl ThingType{
    pub fn to_string(&self) -> &str {
        match *self {
            Self::Tree => "Tree",
            Self::FertileTree => "Fertile Tree",
            Self::Bush => "Bush",
            Self::FertileBush => "Fertile Bush",
            _ => "Other Thing",
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct ThingConfig {
    pub permissions: Vec<ThingPermissions>,
    pub resists: Vec<Resist>,
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
    pub graphic_position: Position<f32>,
    pub graphic_index: u8,

    pub permissions: Vec<ThingPermissions>,
    pub resists: Vec<Resist>,
    pub resists_cache: Vec<Resist>,
    pub body_structure: Vec<BodyPart>,

    pub current_health_points: i16,
    pub total_health_points: i16
}
