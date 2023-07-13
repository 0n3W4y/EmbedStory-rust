use std::collections::HashMap;

pub mod draw;
pub mod cleanup;
pub mod destroeyd_thing_handler;

use serde::{Deserialize, Serialize};

use crate::scenes::game_scenes::tilemap::tile::{Position, TilePermissions};

use super::{stuff::damage_type::DamageType, charactor::stats::ExtraStat};


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
    pub resists: HashMap<DamageType, i16>,
    pub extra_stats: HashMap<ExtraStat, i16>,
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
    pub tile_index: usize, // tile id;

    pub permissions: Vec<ThingPermissions>,
    pub resists: HashMap<DamageType, i16>,
    pub extra_stats: HashMap<ExtraStat, i16>,
}
