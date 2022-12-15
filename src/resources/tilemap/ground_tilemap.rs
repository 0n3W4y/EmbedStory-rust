use serde::{ Serialize, Deserialize };
use std::vec;
use rand::Rng;

use crate::resources::{
    tilemap::tile::ground_tilemap_tile::{ GroundType, CoverType, GroundTilemapTile, GroundTilemapTileDeploy }, 
    deploy::{Deploy, GroundTilemapTileDeployData}
};


pub enum BiomeType{
    Plain,
    Desert,
    Forest,
    Rocks,
    Tropic,
    Snow,
    Swamp,
}

#[derive( Deserialize, Clone )]
pub struct Biome{
    pub main_ground: GroundType,
    pub main_cover: CoverType,
    pub additional_ground: Vec<GroundType>,
    pub additional_ground_value: Vec<f32>,
    pub additional_cover: Vec<CoverType>,
    pub additional_cover_value: Vec<f32>,
    //pub liquids: pub river: Vec<River> pub lake: Vec<Lake>,
    //pub solids: Vec<Solid>,
}


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

    pub fn get_tile_by_index( &mut self, value: usize ) -> &mut GroundTilemapTile{
        let vector_length = self.tilemap_tile_storage.len();
        if value as usize > vector_length{
            panic!( "ground_tilemap::get_tile_by_index. Value > vec.len(); Value:{}, vec.len():{}", value, vector_length );
        }

        return &mut self.tilemap_tile_storage[ value as usize ];
    }

    pub fn generate_tilemap( &mut self, deploy: &Deploy ){
        if self.tile_size == 0 || self.total_tiles == 0{
            panic!( "ground_tilemap::generate_tilemap. Tilemap not setted yet!" );
        }
        let biome_setting: Biome = Biome {
            main_ground: GroundType::Earth,
            main_cover: CoverType::Grass,
            additional_ground: vec![ GroundType::Dirt, GroundType::RockEnvirounment ],
            additional_ground_value: vec![ 5.3, 5.0 ],
            additional_cover: vec![ CoverType::RockyRoad, CoverType::Sand ],
            additional_cover_value: vec![ 5.0, 0.8 ],
        };

        self.generate_ground( &biome_setting.main_ground, &deploy );
        self.generate_additional_ground( &biome_setting.additional_ground, &biome_setting.additional_ground_value, &deploy ); 
        
        self.generate_cover( &biome_setting.main_cover, &deploy );
        self.generate_additional_cover( &biome_setting.additional_cover, &biome_setting.additional_cover_value, &deploy );
    }

    fn generate_ground( &mut self, ground_type: &GroundType, deploy: &Deploy ){
        let tile_setting = deploy.get_ground_tile_data( &ground_type );
        for i in 0..self.tilemap_height {
            for j in 0..self.tilemap_width {
                let mut tile = GroundTilemapTile::new();
                tile.x = j;
                tile.y = i;
                tile.graphic_x = j as u32 * self.tile_size as u32;
                tile.graphic_y = i as u32 * self.tile_size as u32;
                tile.index = i as u32 * self.tilemap_height as u32 + j as u32;
                tile.ground_type = ground_type.clone();
                //cover_type = Covertype::None by default;
                GroundTilemap::set_data_to_tile( &mut tile, &tile_setting );
                self.tilemap_tile_storage.push( tile );
            }
        }
    }

    fn generate_additional_ground( &self, additional_ground_type: &Vec<GroundType>, additional_ground_type_value: &Vec<f32>, deploy: &Deploy ){
        let additional_ground_num: usize = additional_ground_type.len();
        for i in 0..additional_ground_num {
            let percent: f32 = additional_ground_type_value[ i ];
            let ground_type = additional_ground_type[ i ].clone();
            let tile_settings: &GroundTilemapTileDeploy = deploy.get_ground_tile_data( &ground_type );
            let mut remain_tiles: usize = ( self.total_tiles as f32 * percent  / 100.0 ) as usize;

            //guard for infinity loop;
            while remain_tiles > 10 {
                //TODO:
            }
        }
    }

    fn generate_cover( &mut self, cover_type: &CoverType, deploy: &Deploy ){
        let tile_setting = deploy.get_cover_tile_data( &cover_type );
        for i in 0..self.total_tiles{
            let mut tile = self.get_tile_by_index( i );
            tile.cover_type = tile_setting.cover_type.clone();
            GroundTilemap::set_data_to_tile( &mut tile, &tile_setting );
        }
    }

    fn generate_additional_cover( &self, additional_cover_type: &Vec<CoverType>, additional_cover_type_value: &Vec<f32>, deploy: &Deploy ){
        let additional_cover_num: usize = additional_cover_type.len();
        for i in 0..additional_cover_num {
            let percent : f32 = additional_cover_type_value[ i ];
            let cover_type: CoverType = additional_cover_type[ i ].clone();
            let tile_setting: &GroundTilemapTileDeploy = deploy.get_cover_tile_data( &cover_type );
            let mut remain_tiles: usize = ( self.total_tiles as f32 * percent / 100.0 ) as usize;

            while remain_tiles > 10 {
                //TODO:
            }
        }
    }

    fn generate_spots( &mut self, deploy: &Deploy ){
        let amount: u8 = 2;
        let emegring: u8 = 30;  //% 
        let mut rng = rand::thread_rng();
        for n in 0..amount {
            let random_num = rng.gen_range( 0..99 ); //100%
            if random_num < emegring { continue; };

            let ground_type = GroundType::Rock;
            let cover_type: CoverType = CoverType::None;
            let ground_data = deploy.get_ground_tile_data( &ground_type );
            let cover_data = deploy.get_cover_tile_data( &cover_type );
            let max_width: u16 = 10;
            let max_height: u16 = 10;
            let min_width: u16 = 10;
            let min_height: u16 = 10;
            let x_offset: i8 = 1;
            let y_offset: i8 = 1;
            let height_offset: i8 = 1;
            let width_offset: i8 = 1;

            let starting_point_x = rng.gen_range( 0..( self.tilemap_width - max_width ));
            let starting_point_y = rng.gen_range( 0..( self.tilemap_height - max_height ));

            let mut current_width = rng.gen_range( min_width..max_width );
            let mut current_height = rng.gen_range( min_height..max_height );

            let average_width = (( min_width + max_width ) / 2 ) as u16;
            let average_height: u16 = (( min_height + max_height ) / 2 ) as u16;

            let mut left_top_point_x = starting_point_x;
            let mut left_top_point_y: u16 = starting_point_y;

            // do horizontal lines
            for i in 0..average_height {
                left_top_point_x = ( left_top_point_x as i32 + rng.gen_range( -x_offset..x_offset ) as i32 ) as u16;
                current_width = ( current_width as i32 + rng.gen_range( -width_offset..width_offset ) as i32 ) as u16;
                if current_width > max_width { current_width = max_width };
                if current_width < min_width { current_width = min_width };

                let y = starting_point_y + i;
                for j in 0..current_width {
                    let x: u16 = left_top_point_x + j;
                    let index: usize = y as usize * self.tilemap_height as usize + x as usize;
                    if index < 0 || index > self.total_tiles { continue; };

                    let mut tile = &mut self.tilemap_tile_storage[ index ];
                    match cover_type {
                        CoverType::None =>{
                            tile.ground_type = ground_type.clone();
                            GroundTilemap::set_data_to_tile( tile, &ground_data );
                        },
                        _ =>{
                            tile.cover_type = cover_type.clone();
                            GroundTilemap::set_data_to_tile( tile, &cover_data );
                        },
                    };
                }
            }
        }

    }

    fn set_data_to_tile( tile: &mut GroundTilemapTile, data: &GroundTilemapTileDeploy ){
        tile.can_walk = data.can_walk;
        tile.movement_ratio = data.movement_ratio;
        tile.can_place_floor = data.can_place_floor;
        tile.can_place_object = data.can_place_object;
        tile.can_place_stuff = data.can_place_stuff;
        tile.can_remove_floor = data.can_remove_floor;
        tile.can_remove_object = data.can_remove_object;
    }
}