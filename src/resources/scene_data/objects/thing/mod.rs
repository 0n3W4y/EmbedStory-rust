use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::resources::scene_data::objects::body_part::BodyPart;
use crate::resources::scene_data::objects::resists::Resist;
use crate::scenes::game_scenes::tilemap::tile::Position;

use super::body_part::BodyPartType;


#[derive( Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default )]
pub enum ThingPermissions{
    CanHarvested,
    CanRepaired,
    #[default]
    CanBeDestroied,

}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Copy, Default)]
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
    pub resists: Vec<Resist>,
    pub body_struct: HashMap<BodyPartType, i16>
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Thing {
    pub id: usize,
    pub index: usize, // in Scene Vec<Things>,
    pub thing_type: ThingType,
    pub position: Position<i32>,
    pub graphic_position: Position<f32>,
    pub graphic_index: u8,

    pub permissions: Vec<ThingPermissions>,
    pub resists: Vec<Resist>,
    pub resists_cache: Vec<Resist>,
    pub body_structure: Vec<BodyPart>,

    current_health_points: i16,
    total_health_points: i16
}

impl Thing {
    pub fn get_current_health_points(&self) -> i16 {
        return self.current_health_points;
    }

    pub fn get_total_health_points(&self) -> i16 {
        return self.total_health_points;
    }
}
