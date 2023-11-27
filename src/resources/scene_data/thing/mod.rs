use std::collections::HashMap;

pub mod draw;
pub mod cleanup;
pub mod destroeyd_thing_handler;

use serde::{Deserialize, Serialize};

use crate::scenes::game_scenes::tilemap::tile::{Position, TilePermissions};

use super::{Attribute, ResistType};


#[derive( Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default )]
pub enum ThingPermissions{
    CanHarvested,
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
    pub attributes: HashMap<Attribute, i16>,
    pub resists: HashMap<ResistType, i16>,
    pub tile_allow_permissions: Vec<TilePermissions>,
    pub tile_deny_permissions: Vec<TilePermissions>,
    pub tile_movement_ratio: u16
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Thing {
    pub id: usize,
    pub thing_type: ThingType,
    pub position: Position<i32>,
    pub graphic_index: u8, // get image from material_manager;

    pub permissions: Vec<ThingPermissions>,
    pub resists: HashMap<ResistType, i16>,
    pub attributes: HashMap<Attribute, i16>,
    pub attributes_cache: HashMap<Attribute, i16>,
}
