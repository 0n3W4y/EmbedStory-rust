use std::collections::HashMap;
use serde::{Deserialize, Serialize};

pub mod draw;
pub mod cleanup;
pub mod destroeyd_thing_handler;

use crate::scenes::game_scenes::tilemap::tile::{Position, TilePermissions};
use super::{Attribute, Resist};

pub const WEAK_STRUCTURE_DEFENSE_TYPE: u8 = 15;
pub const NORMAL_STRUCTURE_DEFENSE_TYPE: u8 = 30;
pub const STRONG_STRUCTURE_DEFENSE_TYPE: u8 = 45;


#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ThingDefenseType {
    #[default]
    Weak,
    Normal,
    Strong,
}


#[derive( Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default )]
pub enum ThingPermissions{
    CanHarvested,
    #[default]
    CanBeDestroyed,
    CanWalk,
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
    DungeonEnter(usize),
    DungeonExit(usize),
}

#[derive(Deserialize, Clone, Debug)]
pub struct ThingConfig {
    pub permissions: Vec<ThingPermissions>,
    pub attributes: HashMap<Attribute, i16>,
    pub resists: HashMap<Resist, i16>,
    pub tile_allow_permissions: Vec<TilePermissions>,
    pub tile_deny_permissions: Vec<TilePermissions>,
    pub tile_movement_ratio: u16,
    pub thing_defense_type: ThingDefenseType,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Thing {
    pub id: usize,
    pub thing_type: ThingType,
    pub position: Position<i32>,
    pub graphic_index: u8, // get image from material_manager;

    pub permissions: Vec<ThingPermissions>,
    pub resists: HashMap<Resist, i16>,
    pub attributes: HashMap<Attribute, i16>,
    pub attributes_cache: HashMap<Attribute, i16>,
    pub thing_defense_type: ThingDefenseType,
}
