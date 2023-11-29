pub mod tilemap_tile_deploy;
pub mod scene_miscellaneous_deploy;
pub mod game_scene_biome_deploy;
pub mod game_scene_deploy;
pub mod game_objects_deploy;
pub mod charactor_deploy;
pub mod projectile_deploy;
pub mod monster_deploy;

use bevy::prelude::*;
use serde::Deserialize;

use self::monster_deploy::MonsterDeploy;
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
    pub monster_deploy: MonsterDeploy,
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
        let monster_deploy: MonsterDeploy = MonsterDeploy::new();
        return Deploy{
            tile: tile_deploy,
            game_scene_biome: biome_deploy,
            game_scene: game_scene_deploy,
            objects_deploy,
            charactor_deploy,
            projectile_deploy,
            monster_deploy,
        };
    }
}