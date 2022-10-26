#[path = "scene_manager/mod/mod/tile.rs"] mod tile;
use tile::{ GroundType, CoverType };

use serde::{ Deserialize };
use std::collections::HashMap;
use bevy::prelude::*;

use std::fs::File;
use std::io::copy;
use std::io::stdout;

#[derive( Deserialize )]
pub struct BiomeDeployConfig{

}
#[derive( Deserialize )]
pub struct TileDeployConfig{
    walkable: bool,
    ground: GroundType,
    cover: CoverType,
    movement_ratio: u16,
    place_cover: bool,
    place_object: bool,
    remove_cover: bool,
    remove_object: bool,
}
#[derive( Deserialize )]
pub struct SceneDeployConfig{

}
#[derive( Deserialize )]
pub struct EntityDeployConfig{

}

#[derive( Default, Reflect )]
pub struct Deploy{
    pub deploy_biome: HashMap<String, BiomeDeployConfig>,
    pub deploy_tile:HashMap<String, TileDeployConfig>,
    pub deploy_scene: HashMap<String, SceneDeployConfig>,
}

impl Deploy{
    pub fn init( &self ){
        
    }

    pub fn get_biome_config( &self, biome:u32 ) -> &BiomeDeployConfig{
        match biome {
            102 => return &self.deploy_biome[ "102" ],
            100 => return &self.deploy_biome[ "100" ],
            101 => return &self.deploy_biome[ "101" ],
            106 => return &self.deploy_biome[ "106" ],
            105 => return &self.deploy_biome[ "105" ],
            103 => return &self.deploy_biome[ "103" ],
            104 => return &self.deploy_biome[ "104" ],
            _ => panic!(),
        }
    }

    pub fn get_tile_ground_config( &self, ground_type: GroundType )-> &TileDeployConfig {
        match ground_type {
            Earth => return &self.deploy_tile[ "200" ],
            Rock => return &self.deploy_tile[ "203" ],
            DryEarth => return &self.deploy_tile[ "201" ],
            Dirt => return &self.deploy_tile[ "202" ],
            Sandrock => return &self.deploy_tile[ "204" ],
            RockEnvirounment => return &self.deploy_tile[ "205" ],
            SandrockEnvirounment =>  return &self.deploy_tile[ "206" ],
            Nothing => panic!(),
        };
    }

    pub fn get_tile_cover_config( &self, cover_type: CoverType ) ->&TileDeployConfig {
        match cover_type {
            Nothing => return &self.deploy_tile[ "220" ],
            Grass => return &self.deploy_tile[ "221" ],
            Sand => return &self.deploy_tile[ "222" ],
            Snow => return &self.deploy_tile[ "223" ],
            Water => return &self.deploy_tile[ "224" ],
            Ice => return &self.deploy_tile[ "227" ],
            Shallow => return &self.deploy_tile[ "225" ],
            WoodenFloor => return &self.deploy_tile[ "226" ],

        };
    }

    pub fn get_scene_config( &self, scene_type:u32 ) ->&SceneDeployConfig {
        match scene_type {
            400 => return &self.deploy_scene[ "400" ],
            401 => return &self.deploy_scene[ "401" ],
            402 => return &self.deploy_scene[ "402" ],
            403 => return &self.deploy_scene[ "403" ],
            _ => panic!(),
        };
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
    let deploy: HashMap< String, TileDeployConfig> = serde_json::from_str(string).unwrap();
    return deploy;
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
