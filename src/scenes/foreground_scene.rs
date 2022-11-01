#[path = "mod/tilemap.rs"] pub mod tilemap;
#[path = "../entity_manager/character.rs"] mod character;
#[path = "../entity_manager/effect.rs"] mod effect;
#[path = "../entity_manager/object.rs"] mod object;
#[path = "../entity_manager/stuff.rs"] mod stuff;

use tilemap::*;
use character::Character;
use stuff::Stuff;
use object::Object;
use effect::Effect;

pub struct ForegroundScene{
    pub scene_id: u32,
    pub tilemap: Tilemap,
    pub objects: Vec<Object>,
    pub stuff: Vec<Stuff>,
    pub characters: Vec<Character>,
    pub effects: Vec<Effect>,
}

impl ForegroundScene{

}

pub fn new( id: u32 ) -> ForegroundScene{
    let new_tilemap = tilemap::new();
    let result = ForegroundScene{
        scene_id: id,
        tilemap: new_tilemap,
        objects: vec![],
        stuff: vec![],
        characters: vec![],
        effects: vec![],
    };
    return result;
}

