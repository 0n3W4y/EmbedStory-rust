
use serde::Deserialize;
use crate::tilemap::tile::TileDeployConfig;
use crate::scene::BiomeConfig;
use std::collections::HashMap;

pub struct Deploy{
    pub deploy_biome:HashMap< u16, BiomeConfig >,
    pub deploy_tile:HashMap< u16, TileDeployConfig >,
}

impl Deploy{
    pub fn init(){

    }
}



pub fn new() -> Deploy{
    let mut tile_deploy = HashMap::new();
    return Deploy{

    }
}

fn create_tile_deploy() -> HashMap< u16, TileDeployConfig >{

}
