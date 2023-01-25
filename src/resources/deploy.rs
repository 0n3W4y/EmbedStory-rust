use bevy::prelude::*;
use serde::Deserialize;

use crate::resources::deploy_addiction::ground_tilemap_tile_deploy::GroundSceneTileDeploy;
//use crate::resources::deploy_addiction::scene_miscellaneous_deploy::SceneMiscellaneousDeploy;
use crate::resources::deploy_addiction::ground_scene_biome_deploy::GroundSceneBiomeDeploy;
use crate::resources::deploy_addiction::ground_scene_deploy::GroundSceneDeploy;



#[derive( Deserialize, Debug )]
pub struct Deploy{
    pub ground_tilemap_tile: GroundSceneTileDeploy,
    //pub global_tilemap_tile: GlobalTilemapTiledeploy,
    //pub scene_miscellaneous: SceneMiscellaneousDeploy,
    pub ground_scene_biome: GroundSceneBiomeDeploy,
    pub ground_scene: GroundSceneDeploy, 
}

impl Deploy{
    
}

impl FromWorld for Deploy{
    fn from_world( _world: &mut World ) -> Self {
        let cover_path: &str = "deploy/ground_tilemap_tile_cover_config.json";
        let ground_path: &str = "deploy/ground_tilemap_tile_ground_config.json";
        let scene_path: &str = "deploy/scene_config.json";
        let biome_path: &str = "deploy/biome_config.json";
        let ground_scene_path: &str = "deploy/ground_scene_config.json";
        

        let tile_deploy = GroundSceneTileDeploy::new( ground_path, cover_path );
        //let scene_deploy: SceneMiscellaneousDeploy = SceneMiscellaneousDeploy::new( scene_path );
        let biome_deploy: GroundSceneBiomeDeploy = GroundSceneBiomeDeploy::new( biome_path );
        let ground_scene_deploy: GroundSceneDeploy = GroundSceneDeploy::new( ground_scene_path );
        return Deploy{
            ground_tilemap_tile: tile_deploy,
            ground_scene_biome: biome_deploy,
            ground_scene: ground_scene_deploy,
        };
    }
}