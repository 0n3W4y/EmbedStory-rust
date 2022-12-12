use serde::{ Serialize, Deserialize };
use std::vec;

use crate::resources::{tilemap::tile::ground_tilemap_tile::{ GroundType, CoverType, GroundTilemapTile }, deploy::Deploy};


#[derive( Serialize, Deserialize )]
pub struct GroundTilemap{
    tile_size: u16,
    tilemap_width: u16,
    tilemap_height: u16,
    tilemap_tile_storage: Vec<GroundTilemapTile>,
    total_tiles: usize,
}

impl GroundTilemap{
    pub fn new() -> GroundTilemap{
        return GroundTilemap{
            tile_size: 0,
            tilemap_width: 0,
            tilemap_height: 0,
            tilemap_tile_storage: vec![],
            total_tiles: 0
        }
    }
    pub fn set( &mut self, tile_size: u16, width: u16, height: u16 ){
        self.tile_size = tile_size;
        self.tilemap_height = height;
        self.tilemap_width = width;
        self.total_tiles = width as usize * height as usize;
    }

    pub fn get_tile_size( &self ) -> u16{
        return self.tile_size;
    }

    pub fn get_tilemap_width( &self ) -> u16{
        return self.tilemap_width;
    }

    pub fn get_tilemap_height( &self ) -> u16{
        return self.tilemap_height;
    }

    pub fn get_total_tiles( &self ) -> usize{
        return self.total_tiles;
    }

    pub fn get_tile_by_index( &mut self, value: u32 ) -> &mut GroundTilemapTile{
        let vector_length = self.tilemap_tile_storage.len();
        if value as usize > vector_length{
            panic!( "ground_tilemap::get_tile_by_index. Value > vec.len(); Value:{}, vec.len():{}", value, vector_length );
        }

        return &mut self.tilemap_tile_storage[ value as usize ];
    }

    pub fn generate_tilemap( &self, deploy: &Deploy ){
        if self.tile_size == 0 || self.total_tiles == 0{
            panic!( "ground_tilemap::generate_tilemap. Tilemap not setted yet!" );
        }
        let ground_type = GroundType::Earth;
        let additional_ground_type: Vec<GroundType> = vec![ GroundType::Dirt, GroundType::RockEnvirounment ];
        let additional_ground_type_value: Vec<u8> = vec![ 5, 1 ];

        self.prepare_tilemap( &ground_type );
        self.generate_additional_ground( &additional_ground_type, &additional_ground_type_value );

        let cover_type: CoverType = CoverType::Grass;
        let additional_cover_type: Vec<CoverType> = vec![ CoverType::RockyRoad ];
        let additional_cover_type_value: Vec<u8> = vec![ 1 ];

        self.generate_cover( &cover_type );
        self.generate_additional_cover( &additional_cover_type, &additional_cover_type_value );
    }

    fn prepare_tilemap( &self, ground_type: &GroundType ){
        //Generate main grid with default value @Earth;
        for i in 0..self.tilemap_height{
            for j in 0..self.tilemap_width{
                let mut tile = GroundTilemapTile::new();
                tile.x = j;
                tile.y = i;
                tile.graphic_x = j as u32 * self.tile_size as u32;
                tile.graphic_y = i as u32 * self.tile_size as u32;
                tile.index = i as u32 * self.tilemap_height as u32 + j as u32;
                tile.ground_type = ground_type.clone();
            }
        }
    }

    fn generate_additional_ground( &self, additional_ground_type: &Vec<GroundType>, additional_ground_type_value: &Vec<u8> ){
        let additional_ground_num: usize = additional_ground_type.len();
        for i in 0..additional_ground_num {
            let percent = additional_ground_type_value[ i ];
            let ground_type = additional_ground_type[ i ].clone();
        }
    }

    fn generate_cover( &self, cover_type: &CoverType ){

    }

    fn generate_additional_cover( &self, additional_cover_type: &Vec<CoverType>, additional_cover_type_value: &Vec<u8> ){

    }
}