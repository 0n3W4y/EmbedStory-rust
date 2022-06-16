
//use serde::{ Deserialize };
use crate::tilemap::tile::{ TileDeployConfig, GroundType, CoverType };
use crate::scene::{ BiomeDeployConfig, Biome };
use std::collections::HashMap;

use std::fs::File;
use std::io::copy;
use std::io::stdout;


pub struct Deploy{
    pub deploy_biome: HashMap< String, BiomeDeployConfig >,
    pub deploy_tile:HashMap< String, TileDeployConfig >,
    pub deploy_scene: HashMap< String, SceneDeployConfig >,
}

impl Deploy{
    pub fn init(){

    }

    pub fn get_biome_config( &self, biome: &Biome ) -> &BiomeDeployConfig{
        match biome {
            Plain => return &self.deploy_biome[ "102" ],
            Desert => return &self.deploy_biome[ "100" ],
            Forest => return &self.deploy_biome[ "101" ],
            Swamp => return &self.deploy_biome[ "106" ],
            Winter => return &self.deploy_biome[ "105" ],
            Rocks => return &self.deploy_biome[ "103" ],
            Tropics => return &self.deploy_biome[ "104" ],
        }
    }

    pub fn get_tile_ground_config( &self, ground_type: &GroundType )-> &TileDeployConfig {
        match ground_type {
            Earth => return &self.deploy_tile[ "200" ],
            Rock => return &self.deploy_tile[ "203" ],
            DryEarth => return &self.deploy_tile[ "201" ],
            Dirt => return &self.deploy_tile[ "202" ],
            Sandrock => return &self.deploy_tile[ "204" ],
            RockEnvirounment => return &self.deploy_tile[ "205" ],
            SandrockEnvironment =>  return &self.deploy_tile[ "206" ],
        };
    }

    pub fn get_tile_cover_config( &self, cover_type: &CoverType ) ->&TileDeployConfig {
        match cover_type {
            Nothing => return &self.deploy_tile[ "220" ],
            Grass => return &self.deploy_tile[ "221" ],
            Snow => return &self.deploy_tile[ "223" ],
            Water => return &self.deploy_tile[ "224" ],
            Sand => return &self.deploy_tile[ "222" ],
            WoodenFloor => return &self.deploy_tile[ "227" ],
            Ice => return &self.deploy_tile[ "225" ],
            Shallow => return &self.deploy_tile[ "226" ],
        };
    }

    pub fn get_scene_config( &self, scene_type: &SceneType ) ->&SceneDeployConfig {
        match scene_type {

        }
    }
}



pub fn new() -> Deploy{
    return Deploy{
        deploy_tile: create_tile_deploy(),
        deploy_biome: create_biome_deploy(),
        deploy_scene: create_scene_deploy(),
    }
}

fn create_tile_deploy() -> HashMap< String, TileDeployConfig >{
    let mut file = File::open( "deploy/tile_config.json" ).unwrap();
    let mut stdout = stdout();
    let mut string = &copy(&mut file, &mut stdout).unwrap().to_string();
    let deploy: HashMap< String, TileDeployConfig > = serde_json::from_str(string).unwrap();
    return deploy
}

fn create_biome_deploy() -> HashMap< String, BiomeDeployConfig >{
    let mut file = File::open( "deploy/tile_config.json" ).unwrap();
    let mut stdout = stdout();
    let mut string = &copy(&mut file, &mut stdout).unwrap().to_string();
    let deploy: HashMap< String, BiomeDeployConfig > = serde_json::from_str(string).unwrap();
    return deploy;
}

fn create_scene_deploy() -> HashMap< String, SceneDeployConfig >{
    let mut file = File::open( "deploy/tile_config.json" ).unwrap();
    let mut stdout = stdout();
    let mut string = &copy(&mut file, &mut stdout).unwrap().to_string();
    let deploy: HashMap< String, SceneDeployConfig > = serde_json::from_str(string).unwrap();
    return deploy;
}
