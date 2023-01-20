use serde::{ Serialize, Deserialize };

use crate::resources::profile::Profile;
use crate::resources::scene_manager::SceneManager;


#[derive( Serialize, Deserialize )]
pub struct GameData{
    scene_manager: SceneManager,
    profile: Profile,
}