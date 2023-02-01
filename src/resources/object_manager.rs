use serde::{Deserialize, Serialize};

use crate::scenes::game_scenes::game_ground_scene::GameGroundScene;

use super::scene_data::thing::Thing;

#[derive( Deserialize, Serialize, Clone, Debug )]
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

pub struct ObjectManager{
    id: usize,
}

impl ObjectManager{
    pub fn new() -> Self{
        return ObjectManager{
            id: 0,
        };
    }

    pub fn generate_things_for_scene( scene: &mut GameGroundScene ){
        
    }

    pub fn generate_thing( &self, thing_type: ThingType ) -> Thing{
        let id = self.create_id();
    }

    pub fn set_id( &mut self, id:usize ){
        self.id = id;
    }

    fn create_id( &mut self ) -> usize{
        let id = self.id;
        self.id += 1;
        return id;
    }
}