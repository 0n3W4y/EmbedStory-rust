use serde::{ Serialize, Deserialize };

use crate::scenes::game_scenes::game_ground_scene::GameGroundScene;
use crate::scenes::game_scenes::game_globalmap_scene::GameGlobalmapScene;

#[derive( Serialize, Deserialize )]
pub struct SceneManager{
    //globalmap_scene: GameGlobalmapScene,
    ground_scene: Vec<GameGroundScene>,
    //underground_scene: Vec<GameUndergroundScene>,
    scene_id: usize,
}