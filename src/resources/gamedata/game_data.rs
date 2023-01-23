use serde::{ Serialize, Deserialize };

use crate::resources::profile::Profile;
use crate::resources::scene_manager::SceneManager;


#[derive( Serialize, Deserialize )]
pub struct GameData{
    pub scene_manager: SceneManager,
    pub profile: Profile,
}

impl GameData{
    pub fn new() -> Self {
        return GameData{
            scene_manager: SceneManager::new(),
            profile: Profile::new(),
        };
    }

    pub fn save( &self ){}
    pub fn load( &mut self ){}
}