use serde::{ Serialize, Deserialize };

use crate::resources::scene_data::objects::main_resists::MainResists;

use super::body_structure::{ BodyStructure, BodyStructureType };

#[derive( Serialize, Deserialize, Debug, Clone, PartialEq, Eq )]
pub enum ThingType{
    Tree,
    FertileTree,
    Bush,
    FertileBush,
    Rock,
    Boulder,
    Log,
    WoodenWall,
    StoneWall,
    IronWall,
    SteelWall,
    ClayWall,
    WoodenDoor,
    ReinforcedWoodenDoor,
    IronDoor,
    ReinforcedIronDoor,
    SteelDoor,
    ReinforcedSteelDoor,
}

#[derive( Deserialize, Clone )]
pub struct ThingConfig{
    pub can_harvested: bool,
    pub can_repaired: bool,
    pub can_be_destroied: bool,
}


#[derive( Serialize, Deserialize, Debug, Clone )]
pub struct Thing{
    pub id: usize,
    pub index: usize, // in Scene Vec<Things>,
    pub thing_type: ThingType,
    pub x: u16,
    pub y: u16,
    pub graphic_x: u32,
    pub graphic_y: u32,
    pub can_harvested: bool,
    pub can_repaired: bool,
    pub can_be_destroied: bool,
    pub resists: MainResists,
    pub body_structure: BodyStructure,
}

impl Thing {
    pub fn new( id: usize, thing_type: ThingType ) -> Self{
        return Thing { 
            id: id, 
            index: 0, 
            thing_type: thing_type, 
            x: 0, 
            y: 0, 
            graphic_x: 0, 
            graphic_y: 0,
            can_harvested: false,
            can_repaired: false,
            can_be_destroied: false,
            resists: MainResists::new(),
            body_structure: BodyStructure::new( BodyStructureType::Thing ),
        }
    }
}