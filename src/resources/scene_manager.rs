use serde::{Deserialize, Serialize};
use rand::Rng;

use crate::{scenes::game_scenes::{game_scene::GameScene, tilemap::generate::generate_tilemap}, config::TILE_SIZE};

use super::{deploy::{Deploy, game_scene_deploy::Location}, charactor_manager::CharactorManager, thing_manager::ThingManager, profile::Profile};


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
    pub fn generate_new_scenes(
        &mut self, 
        deploy: &Deploy, 
        thing_manager: &mut ThingManager, 
        charactor_manager: &mut CharactorManager,
        profile: &Profile,
        location: &Location
    ) -> &mut GameScene {
        //TODO:: check for dungeon; create dungeon entrance like thing; add into scene id;
        // create 1-st floor, create 2 things. ID to ground scene and id to next floor;
        let mut random = rand::thread_rng();
        let location_config = deploy.game_scene.get_scene_setting(location);

        let scene = self.generate_game_scene(deploy, thing_manager, charactor_manager, profile, location);
        
        let dungeon_percent = location_config.dungeon_chance;
        if dungeon_percent < 100 {                                              //check for dungeon in current scene;
            let random_chance: u8 = random.gen_range(0..=99);
            if dungeon_percent < random_chance {
                return scene;
            }
        }

        //here we need to generate dungeon scenes and generate enter and exit to\from dungeon; 
        todo!();
        return scene;        
    }

    fn generate_game_scene(
        &mut self, 
        deploy: &Deploy, 
        thing_manager: &mut ThingManager, 
        charactor_manager: &mut CharactorManager, 
        profile: &Profile, 
        location: &Location
    ) -> &mut GameScene {
        let scene = self.create_game_scene(deploy, location);
        thing_manager.generate_things_for_scene(scene, deploy);
        let player_level = match &profile.charactor {
            Some(v) => v.level,
            None => {
                println!("Can not get player level in Game Scene Generator. Using 0");
                0
            }
        };
        charactor_manager.generate_charactors_for_scene(scene, deploy, player_level);

        return scene;
    }

    fn create_game_scene(&mut self, deploy: &Deploy, location: &Location) -> &mut GameScene {
        let scene_id = self.create_scene_id();
        let scene_config = deploy.game_scene.get_scene_setting(location);

        let mut scene: GameScene = GameScene {
            scene_id,
            location: location.clone(),
            biome_type: scene_config.biome_type.clone(),
            ..Default::default()
        };

        scene.tilemap.set(TILE_SIZE, scene_config.width, scene_config.height);
        generate_tilemap(&mut scene.tilemap, deploy, &scene.biome_type);

        return self.store_game_scene(scene);
    }

    fn store_game_scene(&mut self, scene: GameScene) -> &mut GameScene {
        let id = scene.scene_id;
        self.game_scene.push(scene);
        return self.get_game_scene_by_id_mut(id);
    }

    pub fn get_current_game_scene_mut(&mut self) -> &mut GameScene{
        let index: usize = match self.current_game_scene {
            Option::Some(v) => v,
            Option::None => {
                panic!("scene_manager.get_current_game_scene_mut. Can't get current scene.");
            }
        };
        self.get_game_scene_by_id_mut(index)
    }

    pub fn get_current_game_scene(& self) -> &GameScene {
        let index: usize = match self.current_game_scene {
            Option::Some(v) => v,
            Option::None => {
                panic!("scene_manager.get_current_game_scene. Can't get current scene.No current game scene");
            }
        };
        self.get_game_scene_by_id(index)        
    }

    pub fn get_game_scene_by_id_mut(&mut self, scene_id: usize) -> &mut GameScene {
        for scene in self.game_scene.iter_mut() {
            if scene.scene_id == scene_id {
                return scene;
            }
        }

        panic!("Can't get current scene, because ID '{:?}' not available in game_scenes vector", scene_id);
    }

    pub fn get_game_scene_by_id(&self, scene_id: usize) -> &GameScene {
        for scene in self.game_scene.iter() {
            if scene.scene_id == scene_id {
                return scene;
            }
        }

        panic!("Can't get current scene, because ID '{:?}' not available in game_scenes vector", scene_id);
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
    pub fn get_next_scene(&mut self) -> &mut GameScene {
        let id = match self.next_game_scene {
            Some(v) => v,
            None => panic!("Can't get next scene, because it's empty")
        };

        self.get_game_scene_by_id_mut(id)
    }

    fn create_scene_id(&mut self) -> usize {
        let scene_id = self.scene_id;
        self.scene_id += 1;
        return scene_id;
    }
}
