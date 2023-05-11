use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::resources::scene_data::objects::charactor::Charactor;
use crate::resources::scene_data::objects::scene_effect::SceneEffect;
use crate::resources::scene_data::objects::stuff::Stuff;
use crate::resources::scene_data::objects::thing::Thing;
use crate::resources::scene_manager::SceneType;
use crate::scenes::game_scenes::tilemap::Tilemap;
use crate::scenes::game_scenes::tilemap;
use crate::resources::scene_data::objects::thing;
use crate::scenes::SceneState;


#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GameScene {
    pub scene_type: SceneType,
    pub scene_id: usize,
    pub index: usize, // vector index in scene_manager.ground_scene;
    pub tilemap: Tilemap,
    pub things: Vec<Thing>,
    pub stuff: Vec<Stuff>,
    pub charactors: Vec<Charactor>,
    pub effects: Vec<SceneEffect>,
    //pub fog: Vec<>,
    //pub roof: Vec<>,
}
impl GameScene {
    pub fn get_thing_by_id_mut(&mut self, id: usize) -> Option<&mut Thing> {
        for thing in self.things.iter_mut(){
            if thing.id == id {
                return Some(thing);
            }
        }
        return None;
    }
}

pub struct GameScenePlugin;

impl Plugin for GameScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(SceneState::GameScene)
            .with_system(tilemap::draw::draw)
            .with_system(thing::draw::draw)
        );

        app.add_system_set(SystemSet::on_update(SceneState::GameScene)
            .with_system(thing::destroeyd_thing_handler::destroeyd_thing_handler)
            .with_system(tilemap::change_cover_type_handler::change_cover_type_handler)
            .with_system(update)
        );

        app.add_system_set(SystemSet::on_exit(SceneState::GameScene)
            .with_system(tilemap::cleanup::cleanup)
            .with_system(thing::cleanup::cleanup)
        );
    }
}



fn update() {}
