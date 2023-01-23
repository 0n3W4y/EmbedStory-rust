use serde::{ Serialize, Deserialize };

use crate::scenes::game_scenes::game_ground_scene::GameGroundScene;
use crate::scenes::game_scenes::game_globalmap_scene::GameGlobalmapScene;


#[derive( Serialize, Deserialize )]
pub struct SceneManager{
    pub next_ground_scene: isize,
    pub next_undeground_scene: isize,
    //globalmap_scene: GameGlobalmapScene,
    ground_scene: Vec<GameGroundScene>,
    //underground_scene: Vec<GameUndergroundScene>,
    scene_id: usize,
}

impl SceneManager{
    pub fn new() -> Self {
        return SceneManager { 
            next_ground_scene: -1, // empty
            next_undeground_scene: -1, // empty
            ground_scene: vec![], 
            scene_id: 0, 
        };
    }

    pub fn create_ground_scene( &mut self ) -> &mut GameGroundScene {
        let scene_id = self.create_scene_id();
        let index = self.ground_scene.len();
        let scene: GameGroundScene = GameGroundScene::new( scene_id, index );
        self.ground_scene.push( scene );
        return &mut self.ground_scene[ index ];
    }

    pub fn get_next_ground_scene( &mut self ) -> &mut GameGroundScene {
        if self.next_ground_scene < 0 || self.next_ground_scene as usize >= self.ground_scene.len() {
            panic!( "scene_manager.get_next_ground_scene. Can't get next scene. Next scene id: {}.", self.next_ground_scene );
        }

        return &mut self.ground_scene[ self.next_ground_scene as usize ];
    }

    pub fn get_ground_scene_by_id( &mut self, scene_id: usize ) -> &mut GameGroundScene{
        for i in 0..self.ground_scene.len(){
            if scene_id == self.ground_scene[ i ].scene_id {
                return &mut self.ground_scene[ i ];
            }
        }

        panic!( "scene_mnager.get_ground_scene_by_id. There is no id: {} in vector with Ground Scene", scene_id );
    }

    fn create_scene_id( &mut self ) -> usize{
        let scene_id = self.scene_id;
        self.scene_id += 1;
        return scene_id;
    }
}