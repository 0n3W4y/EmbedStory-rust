
use serde::{ Deserialize };
use crate::tilemap::tile::TileDeployConfig;
use crate::scene::BiomeDeployConfig;
use std::collections::HashMap;

use std::fs::File;
use std::io::copy;
use std::io::stdout;


pub struct Deploy{
    pub deploy_biome: HashMap< String, BiomeDeployConfig >,
    pub deploy_tile:HashMap< String, TileDeployConfig >,
}

impl Deploy{
    pub fn init(){

    }
}



pub fn new() -> Deploy{
    return Deploy{
        deploy_tile: create_tile_deploy(),
        deploy_biome: create_biome_deploy(),
    }
}

fn create_tile_deploy() -> HashMap< String, TileDeployConfig >{
    let mut file = File::open( "deploy/tile_config.json" ).unwrap();
    let mut stdout = stdout();
    let mut string = &copy(&mut file, &mut stdout).unwrap().to_string();
    let deploy_biome: HashMap< String, BiomeDeployConfig > = serde_json::from_str(string).unwrap();
    return deploy_biome;
}

fn create_biome_deploy() -> HashMap< u16, BiomeDeployConfig >{

}
