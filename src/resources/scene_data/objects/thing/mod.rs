use serde::{Deserialize, Serialize};

use crate::resources::scene_data::objects::body_part::BodyPart;
use crate::resources::scene_data::objects::resists::Resist;
use crate::scenes::game_scenes::tilemap::tile::Position;

use super::body_part::{BodyPartType, HealthPoints};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Copy)]
pub enum ThingType {
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
    pub can_harvested: bool,
    pub can_repaired: bool,
    pub can_be_destroied: bool,
    pub electric_resist: i16,
    pub fire_resist: i16,
    pub kinetic_resist: i16,
    pub laser_resist: i16,
    pub plasma_resist: i16,
    pub health_points: i16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Thing {
    pub id: usize,
    pub index: usize, // in Scene Vec<Things>,
    pub thing_type: ThingType,
    pub position: Position,
    pub graphic_position: Position,

    pub graphic_index: u8,
    pub can_harvested: bool,
    pub can_repaired: bool,
    pub can_be_destroied: bool,

    pub resists: Vec<Resist>,
    pub resists_cache: Vec<Resist>,
    pub body_structure: Vec<BodyPart>,

    pub health_points: HealthPoints,
}

impl Thing {
    pub fn new(id: usize, thing_type: ThingType) -> Self {
        let resists = vec![
            Resist::Kinetic(0),
            Resist::Fire(0),
            Resist::Electric(0),
            Resist::Laser(0),
            Resist::Plasma(0),
        ];

        let resists_cache = vec![
            Resist::Kinetic(0),
            Resist::Fire(0),
            Resist::Electric(0),
            Resist::Laser(0),
            Resist::Plasma(0),
        ];

        let body_structure = vec![BodyPart::new(BodyPartType::Torso)];

        return Thing {
            id: id,
            index: 0,
            thing_type: thing_type,
            position: Position(0, 0),
            graphic_position: Position(0, 0),
            graphic_index: 0,
            can_harvested: false,
            can_repaired: false,
            can_be_destroied: false,
            resists,
            resists_cache,
            body_structure,
            health_points: HealthPoints(0, 0, 0),
        };
    }
}
