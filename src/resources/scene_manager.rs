use serde::{ Serialize, Deserialize };

use crate::scenes::game_scenes::game_scene::GameScene;

#[derive( Serialize, Deserialize, Clone )]
pub enum SceneType{
    GroundScene,
    UndergroundScene,
}

#[derive( Serialize, Deserialize )]
pub struct SceneManager{
    pub next_ground_scene: isize,
    //pub next_undeground_scene: isize,
    //globalmap_scene: GameGlobalmapScene,
    game_scene: Vec<GameScene>,
    //underground_scene: Vec<GameUndergroundScene>,
    scene_id: usize,
}

impl SceneManager{
    pub fn new() -> Self {
        return SceneManager { 
            next_ground_scene: -1, // empty
            //next_undeground_scene: -1, // empty
            game_scene: vec![], 
            scene_id: 0, 
        };
    }

    pub fn create_ground_scene( &mut self ) -> GameScene {
        let scene_id = self.create_scene_id();
        let scene: GameScene = GameScene::new( scene_id );
        return scene;
    }

    pub fn store_ground_scene( &mut self, mut scene: GameScene ) -> usize {
        let index: usize = self.game_scene.len();
        scene.index = index;
        self.game_scene.push( scene );
        return index;
    }

    pub fn get_next_ground_scene( &mut self ) -> &mut GameScene {
        if self.next_ground_scene < 0 || self.next_ground_scene as usize >= self.game_scene.len() {
            panic!( "scene_manager.get_next_ground_scene. Can't get next scene. Next scene id: {}.", self.next_ground_scene );
        }

        return &mut self.game_scene[ self.next_ground_scene as usize ];
    }

    pub fn get_ground_scene_by_id( &mut self, scene_id: usize ) -> &mut GameScene{
        for i in 0..self.game_scene.len(){
            if scene_id == self.game_scene[ i ].scene_id {
                return &mut self.game_scene[ i ];
            }
        }

        panic!( "scene_mnager.get_ground_scene_by_id. There is no id: {} in vector with Ground Scenes", scene_id );
    }

    pub fn set_next_ground_scene( &mut self, index: usize ){
        if self.next_ground_scene == -1 {
            self.next_ground_scene = index as isize;
        }else{
            panic!( "scene_manager.set_next_ground_scene. Can not assigned next ground scene, because it already assigned or not deassigned. Next Scene: {}", self.next_ground_scene );
        }
        
    }

    fn create_scene_id( &mut self ) -> usize{
        let scene_id = self.scene_id;
        self.scene_id += 1;
        return scene_id;
    }
}