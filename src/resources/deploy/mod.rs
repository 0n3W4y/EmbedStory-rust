pub mod tilemap_tile_deploy;
pub mod scene_miscellaneous_deploy;
pub mod game_scene_biome_deploy;
pub mod game_scene_deploy;
pub mod game_objects_deploy;
pub mod projectile_deploy;
pub mod charactor_deploy;

use bevy::prelude::*;
use serde::Deserialize;

use self::charactor_deploy::CharactorDeploy;
use self::projectile_deploy::ProjectileDeploy;

//use crate::resources::deploy_addiction::scene_miscellaneous_deploy::SceneMiscellaneousDeploy;
use super::deploy::game_scene_deploy::GameSceneDeploy;
use super::deploy::tilemap_tile_deploy::TilemapTileDeploy;
use super::deploy::game_scene_biome_deploy::GameSceneBiomeDeploy;
use super::deploy::game_objects_deploy::GameObjectsDeploy;


pub const DEPLOY_BIOME_PATH: &str = "deploy/biome_config.json";
pub const DEPLOY_COVER_PATH: &str = "deploy/tilemap_tile_cover_config.json";
pub const DEPLOY_GROUND_PATH: &str = "deploy/tilemap_tile_ground_config.json";
//pub const DEPLOY_SCENE_PATH: &str = "deploy/scene_config.json";
pub const DEPLOY_GROUND_SCENE_PATH: &str = "deploy/game_scene_config.json";
pub const DEPLOY_OBJECTS_PATH: &str = "deploy/game_objects_config.json";
pub const DEPLOY_RACE_PATH: &str = "deploy/race_config.json";
pub const DEPLOY_EFFECTS_PATH: &str = "deploy/battle_effects_config.json";
pub const DEPLOY_SKILLS_PATH: &str = "deploy/skills_config.json";
pub const DEPLOY_PROJECTILE_PATH: &str = "deploy/projectiles.json";
pub const DEPLOY_MONSTER_STRENGTH_PATH: &str = "deploy/mosnter_config.json";
pub const DEPLOY_LOCATION_PATH: &str = "deploy/location_config.json";
pub const DEPLOY_NPC_PATH: &str = "deploy/npc_config.json";
pub const DEPLOY_COMPANION_PATH: &str = "deploy/companion_config";


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
        let tile_deploy = TilemapTileDeploy::new();
        //let scene_deploy: SceneMiscellaneousDeploy = SceneMiscellaneousDeploy::new( scene_path );
        let biome_deploy: GameSceneBiomeDeploy = GameSceneBiomeDeploy::new();
        let game_scene_deploy: GameSceneDeploy = GameSceneDeploy::new();
        let objects_deploy: GameObjectsDeploy = GameObjectsDeploy::new();
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