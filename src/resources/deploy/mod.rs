pub mod tilemap_tile_deploy;
pub mod scene_miscellaneous_deploy;
pub mod game_scene_biome_deploy;
pub mod game_scene_deploy;
pub mod game_objects_deploy;
pub mod charactor_deploy;
pub mod projectile_deploy;

use bevy::prelude::*;
use serde::Deserialize;

use self::projectile_deploy::ProjectileDeploy;

//use crate::resources::deploy_addiction::scene_miscellaneous_deploy::SceneMiscellaneousDeploy;
use super::deploy::game_scene_deploy::GameSceneDeploy;
use super::deploy::tilemap_tile_deploy::TilemapTileDeploy;
use super::deploy::game_scene_biome_deploy::GameSceneBiomeDeploy;
use super::deploy::game_objects_deploy::GameObjectsDeploy;
use super::deploy::charactor_deploy::CharactorDeploy;



#[derive( Deserialize, Debug )]
pub struct Deploy{
    pub tile: TilemapTileDeploy,
    //pub scene_miscellaneous: SceneMiscellaneousDeploy,
    pub objects_deploy: GameObjectsDeploy, 
    pub game_scene_biome: GameSceneBiomeDeploy,
    pub game_scene: GameSceneDeploy, 
    pub charactor_deploy: CharactorDeploy,
    pub projectile_deploy: ProjectileDeploy,
}

impl Deploy{
    
}

impl FromWorld for Deploy{
    fn from_world( _world: &mut World ) -> Self {
        let cover_path: &str = "deploy/tilemap_tile_cover_config.json";
        let ground_path: &str = "deploy/tilemap_tile_ground_config.json";
        //let scene_path: &str = "deploy/scene_config.json";
        let biome_path: &str = "deploy/biome_config.json";
        let ground_scene_path: &str = "deploy/game_scene_config.json";
        let objects_path: &str = "deploy/game_objects_config.json";
        

        let tile_deploy = TilemapTileDeploy::new(ground_path, cover_path);
        //let scene_deploy: SceneMiscellaneousDeploy = SceneMiscellaneousDeploy::new( scene_path );
        let biome_deploy: GameSceneBiomeDeploy = GameSceneBiomeDeploy::new(biome_path);
        let game_scene_deploy: GameSceneDeploy = GameSceneDeploy::new(ground_scene_path);
        let objects_deploy: GameObjectsDeploy = GameObjectsDeploy::new(objects_path);
        let charactor_deploy: CharactorDeploy = CharactorDeploy::new();
        let projectile_deploy: ProjectileDeploy = ProjectileDeploy::new();
        return Deploy{
            tile: tile_deploy,
            game_scene_biome: biome_deploy,
            game_scene: game_scene_deploy,
            objects_deploy,
            charactor_deploy,
            projectile_deploy,
        };
    }
}