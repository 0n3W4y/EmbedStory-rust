use bevy::prelude::*;
use serde::Deserialize;

use crate::resources::deploy_addiction::ground_tilemap_tile_deploy::GroundSceneTileDeploy;
//use crate::resources::deploy_addiction::scene_miscellaneous_deploy::SceneMiscellaneousDeploy;



#[derive( Deserialize, Debug )]
pub struct Deploy{
    pub ground_tilemap_tile: GroundSceneTileDeploy,
    //pub global_tilemap_tile: GlobalTilemapTiledeploy,
    //pub scene_miscellaneous: SceneMiscellaneousDeploy,
    //pub ground_scene_biome: BiomeDeploy,
}

impl Deploy{
    
}

impl FromWorld for Deploy{
    fn from_world( _world: &mut World ) -> Self {
        let cover_path: &str = "deploy/ground_tilemap_tile_cover_config.json";
        let ground_path: &str = "deploy/ground_tilemap_tile_ground_config.json";
        let scene_path: &str = "deploy/scene_config.json";
        let biome_path: &str = "deploy/biome_config.json";
        

        let tile_deploy = GroundSceneTileDeploy::new( ground_path, cover_path );
        //let scene_deploy: SceneMiscellaneousDeploy = SceneMiscellaneousDeploy::new( scene_path );
        return Deploy{
            ground_tilemap_tile: tile_deploy,
        };
    }
}