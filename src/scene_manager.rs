pub mod foreground_scene;
pub mod loader_scene;
pub mod main_menu_scene;
pub mod create_char_scene;
pub mod underground_scene;

use bevy::{
    prelude::*,
};

use foreground_scene::*;

pub enum SceneType{
    ForegroundScene,
    GlobalMapScene,
    MainMenuScene,
    UndergroundScene,
    CreateCharScene,
    LoaderScene,
}

#[derive( Default, Reflect )]
pub struct SceneManager {
    scenes_id: u32,
    foreground_scenes: Vec<foreground_scene::ForegroundScene>,
    //undeground_scenes: Vec<uderground_scene::UndergroundScene>,
    //global_map_scene: GlobalMapScene,
    //loader_scene: LoaderScene,
    //main_menu_scene: MainMenuScene,
}

impl SceneManager {

    pub fn init( &self ){
    }

    pub fn create_foreground_scene( &self ) -> foreground_scene::ForegroundScene{
        let id = self.create_scene_id();
        let result = ForegroundScene{
            scene_id: id,
            tilemap: tilemap::new(),
            objects: vec![],
            stuff: vec![],
            effects: vec![],
            characters: vec![],
        };

        return result;
    }

    pub fn save(){

    }

    pub fn load(){

    }

    fn create_scene_id( &mut self ) -> u32{
        let result = self.scenes_id;
        self.scenes_id += 1;
        return result;
    }

}