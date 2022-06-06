pub mod tile;
use bevy::{
    prelude::*,
};
use std::collections::HashMap;
use tile::*;




pub struct TilemapConfig{
    width:u16,
    height:u16,
    tile_size:u16,
    tile_deploy:HashMap< u16, TileConfig >,
}


pub struct Tilemap{
    pub tiles:Vec<Tile>,
    pub width:u16,
    pub height:u16,
    pub tile_size:u16,
    pub tile_deploy:HashMap<u16, TileConfig>,
}

impl Tilemap{
    pub fn generate_tilemap(){

    }

    pub fn change_tile_ground( pos:Position, ground:GroundType ){

    }

    pub fn change_tile_cover( pos:Position, cover:CoverType ){

    }

    fn create_tile() -> Tile{
        let tile = tile::new();
        return tile;
    }
}

pub fn new( config: TilemapConfig ) -> Tilemap{
    let vec = vec![];
    return Tilemap{
        tiles: vec,
        width: config.width,
        height: config.height,
        tile_size: config.tile_size,
        tile_deploy: config.tile_deploy,
    }
}