use serde::{Deserialize, Serialize};

use crate::scenes::game_scenes::game_scene::GameScene;

#[derive(Serialize, Deserialize, Clone, Default)]
pub enum SceneType {
    #[default]
    GroundScene,
    UndergroundScene,
}

#[derive(Serialize, Deserialize, Default)]
pub struct SceneManager {
    pub next_game_scene: Option<usize>,
    pub current_game_scene: Option<usize>,
    //globalmap_scene: GameGlobalmapScene,
    game_scene: Vec<GameScene>,
    //underground_scene: Vec<GameUndergroundScene>,
    scene_id: usize,
}

impl SceneManager {
    pub fn create_game_scene(&mut self, scene_type: &SceneType) -> GameScene {
        let scene_id = self.create_scene_id();
        let scene: GameScene = GameScene {
            scene_id,
            scene_type: scene_type.clone(),
            ..Default::default()
        };
        return scene;
    }

    pub fn store_game_scene(&mut self, mut scene: GameScene) -> usize {
        let index: usize = self.game_scene.len();
        scene.index = index;
        self.game_scene.push(scene);
        return index;
    }

    pub fn get_current_game_scene_mut(&mut self) -> &mut GameScene{
        let index: usize = match self.current_game_scene {
            Option::Some(v) => v,
            Option::None => {
                panic!("scene_manager.get_current_game_scene_mut. Can't get current scene.");
            }
        };

        return &mut self.game_scene[index];
    }

    pub fn get_current_game_scene(& self) -> & GameScene{
        let index: usize = match self.current_game_scene {
            Option::Some(v) => v,
            Option::None => {
                panic!("scene_manager.get_current_game_scene. Can't get current scene.");
            }
        };

        return &self.game_scene[index];
    }

    pub fn get_game_scene_by_id_mut(&mut self, scene_id: usize) -> &mut GameScene {
        for i in 0..self.game_scene.len() {
            if scene_id == self.game_scene[i].scene_id {
                return &mut self.game_scene[i];
            }
        }

        panic!(
            "scene_mnager.get_ground_scene_by_id. There is no id: {} in vector with Ground Scenes",
            scene_id
        );
    }

    pub fn clear_current_game_scene(&mut self){
        self.current_game_scene = None;
    }

    pub fn set_current_game_scene(&mut self, id: usize) {
        match self.current_game_scene {
            Option::Some(v) => {
                println!(
                    "scene_manager.set_current_game_scene. Can not assigned next ground scene, because it already assigned or not deassigned. Current Scene id: {}",
                    v
                );
                self.next_game_scene = Some(id);
            }
            Option::None => self.current_game_scene = Some(id),
        };
    }

    fn create_scene_id(&mut self) -> usize {
        let scene_id = self.scene_id;
        self.scene_id += 1;
        return scene_id;
    }
}
