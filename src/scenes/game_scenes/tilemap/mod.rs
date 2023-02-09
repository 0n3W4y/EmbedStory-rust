use serde::{ Serialize, Deserialize };
use std::vec;
use rand::Rng;

pub mod tile;

use tile::{ Tile, CoverType, GroundType, TileDeploy };
use crate::resources::deploy::Deploy;
use crate::resources::deploy_addiction::game_scene_biome_deploy::{ BiomeType, SpotSetting, RiverSetting, RiverType, Spots, Rivers, Biome };

#[derive( Serialize, Deserialize, Clone )]
pub struct Tilemap{
    tile_size: u16,
    tilemap_width: u16,
    tilemap_height: u16,
    tilemap_tile_storage: Vec<Tile>,
    total_tiles: usize,
}

impl Tilemap{
    pub fn new() -> Tilemap{
        return Tilemap{
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

    pub fn get_tilemap_tile_storage_mut( &mut self ) -> &mut Vec<Tile>{
        return &mut self.tilemap_tile_storage;
    }

    pub fn get_tilemap_tile_storage( &self ) -> &Vec<Tile>{
        return &&self.tilemap_tile_storage;
    }

    pub fn get_tile_by_index( &self, value: usize ) -> &Tile{
        let vector_length = self.tilemap_tile_storage.len();
        if value >= vector_length{
            panic!( "ground_tilemap::get_tile_by_index. Value > vec.len(); Value:{}, vec.len():{}", value, vector_length );
        }

        return &self.tilemap_tile_storage[ value ];
    }

    pub fn get_tile_by_index_mut( &mut self, value: usize ) -> &mut Tile{
        let vector_length = self.tilemap_tile_storage.len();
        if value >= vector_length{
            panic!( "ground_tilemap::get_tile_by_index. Value > vec.len(); Value:{}, vec.len():{}", value, vector_length );
        }

        return &mut self.tilemap_tile_storage[ value ];
    }

    pub fn generate_tilemap( &mut self, deploy: &Deploy, biome_type: &BiomeType ){
        if self.tile_size == 0 || self.total_tiles == 0{
            panic!( "ground_tilemap::generate_tilemap. Tilemap not setted yet!" );
        }

        let biome_setting: &Biome = deploy.game_scene_biome.get_biome_setting( &biome_type );

        self.generate_ground( &biome_setting.main_ground, deploy );
        self.generate_additional_ground( &biome_setting.additional_ground, &biome_setting.additional_ground_value, deploy ); 
        
        self.generate_cover( &biome_setting.main_cover, biome_setting.main_cover_filling, deploy );
        self.generate_additional_cover( &biome_setting.additional_cover, &biome_setting.additional_cover_value, deploy );

        self.generate_solids_liquids( &biome_setting.spots, &biome_setting.rivers, deploy );

        self.generate_environment( deploy, 2 );
        self.spread_indexes_for_cover_tiles();
    }

    fn generate_ground( &mut self, ground_type: &GroundType, deploy: &Deploy ){
        let tile_setting = deploy.tile.get_ground_tile_deploy( &ground_type );
        for i in 0..self.tilemap_height {
            for j in 0..self.tilemap_width {
                let mut tile = Tile::new();
                tile.x = j;
                tile.y = i;
                tile.graphic_x = j as u32 * self.tile_size as u32;
                tile.graphic_y = i as u32 * self.tile_size as u32;
                tile.index = i as usize * self.tilemap_height as usize + j as usize;
                tile.ground_type = ground_type.clone();
                Tilemap::set_data_to_tile( &mut tile, &tile_setting );
                self.tilemap_tile_storage.push( tile );
            }
        }
    }

    fn generate_additional_ground( &mut self, additional_ground_type: &Vec<GroundType>, additional_ground_type_value: &Vec<f32>, deploy: &Deploy ){
        if additional_ground_type.len() != additional_ground_type_value.len(){
            panic!( "ground_tilemap.generate_additional_ground. Wrong cofiguration in JSON two vectors not equal" );
        }

        let additional_ground_length: usize = additional_ground_type.len();
        let mut rng = rand::thread_rng();
        for i in 0..additional_ground_length {
            let percent: f32 = additional_ground_type_value[ i ];
            let ground_type = additional_ground_type[ i ].clone();
            let mut remain_tiles: usize = ( self.total_tiles as f32 * percent  / 100.0 ) as usize; //how manu tiles need to be created;
            let mut max_width = ( self.tilemap_width * 5 / 100 ) as u16; // 5% of tilemap width;
            if max_width < 5 { max_width = 5; }; // min value;
            let mut max_height: u16 = ( self.tilemap_height * 5 / 100 ) as u16; // 5% of tilemap height;
            if max_height < 5 { max_height = 5; }; // min value;

            //guard for infinity loop;
            while remain_tiles > 10 {
                let current_max_width = rng.gen_range( 4..( 1+ max_width ));
                let current_max_height = rng.gen_range( 4..( 1+ max_height ));

                let mut current_max_width_for_min_width_range: u16 = ( current_max_width / 4 ) as u16;
                if current_max_width_for_min_width_range < 2 { current_max_width_for_min_width_range = 2; };

                let mut current_max_height_for_min_height_range: u16 = ( current_max_height / 4 ) as u16;
                if current_max_height_for_min_height_range < 2 { current_max_height_for_min_height_range = 2; };

                let current_min_width = rng.gen_range( 1..( 1+ current_max_width_for_min_width_range )); // 25% of maximum value
                let current_min_height = rng.gen_range( 1..( 1+ current_max_height_for_min_height_range )); // 25% of maximum value

                let spot_setting: SpotSetting = SpotSetting { 
                    amount: 1, 
                    emerging: 100, //100%
                    ground_type: ground_type.clone(), 
                    cover_type: CoverType::None, 
                    max_width: current_max_width, 
                    max_height: current_max_height, 
                    min_width: current_min_width, 
                    min_height: current_min_height, 
                    x_offset: 1, 
                    y_offset: 1, 
                    height_offset: 1, 
                    width_offset: 1,
                };

                self.generate_spot( deploy, &spot_setting );
                let tiles_used = (( current_max_width + current_min_width ) / 2 ) * (( current_max_height + current_min_height ) / 2 );
                remain_tiles -= tiles_used as usize;
            }
        }
    }

    fn generate_cover( &mut self, cover_type: &CoverType, percent: u8, deploy: &Deploy ){
        let mut rng = rand::thread_rng();
        let tile_setting = deploy.tile.get_cover_tile_deploy( &cover_type );
        for tile in self.tilemap_tile_storage.iter_mut(){
            let random_num = rng.gen_range( 0..100 ); // 100%
            if percent >random_num {
                let set_cover_to_tile: bool = match cover_type {
                    CoverType::Grass => {
                        match tile.ground_type {
                            GroundType::Earth => { true },
                            _ => { false }
                        }
                    },
                    _ => { true }
                };
                if set_cover_to_tile{
                    tile.cover_type = tile_setting.cover_type.clone();
                    Tilemap::set_data_to_tile( tile, &tile_setting );
                }
            }
        }
        
    }

    fn generate_additional_cover( &mut self, additional_cover_type: &Vec<CoverType>, additional_cover_type_value: &Vec<f32>, deploy: &Deploy ){
        if additional_cover_type.len() != additional_cover_type_value.len() {
            panic!( "ground_tilemap.generate_additional_cover. Wrong JSON file, two vectors are not equal" );
        }

        let additional_cover_num: usize = additional_cover_type.len();
        let mut rng = rand::thread_rng();
        for i in 0..additional_cover_num {
            let percent : f32 = additional_cover_type_value[ i ];
            let cover_type: CoverType = additional_cover_type[ i ].clone();
            let mut remain_tiles: usize = ( self.total_tiles as f32 * percent / 100.0 ) as usize;

            let mut max_width = ( self.tilemap_width * 5 / 100 ) as u16; // 5% of tilemap width;
            if max_width < 5 { max_width = 5; };

            let mut max_height: u16 = ( self.tilemap_height * 5 / 100 ) as u16; // 5% of tilemap height;
            if max_height < 5 { max_height = 5; };

            while remain_tiles > 10 {
                let current_max_width = rng.gen_range( 4..( max_width +1 ));
                let current_max_height = rng.gen_range( 4..( max_height +1 ));

                let mut current_max_width_for_min_width_range: u16 = ( current_max_width / 4)  as u16;
                if current_max_width_for_min_width_range < 2 { current_max_width_for_min_width_range = 2; };

                let mut current_max_height_for_min_height_range: u16 = ( current_max_height / 4 ) as u16;
                if current_max_height_for_min_height_range < 2 { current_max_height_for_min_height_range = 2; };

                let current_min_width = rng.gen_range( 1..( current_max_width_for_min_width_range +1 )); // 25% of maximum value
                let current_min_height = rng.gen_range( 1..( current_max_height_for_min_height_range +1 )); // 25% of maximum value

                let spot_setting: SpotSetting = SpotSetting { 
                    amount: 1, 
                    emerging: 100, //100%
                    ground_type: GroundType::Earth, 
                    cover_type: cover_type.clone(), 
                    max_width: current_max_width, 
                    max_height: current_max_height, 
                    min_width: current_min_width, 
                    min_height: current_min_height, 
                    x_offset: 1, 
                    y_offset: 1, 
                    height_offset: 1, 
                    width_offset: 1,
                };

                self.generate_spot( deploy, &spot_setting );
                let tiles_used = (( current_max_width + current_min_width ) / 2 ) * (( current_max_height + current_min_height ) / 2 );
                remain_tiles -= tiles_used as usize;
            }
        }
    }

    fn generate_spots( &mut self, spot_vector: &Vec<SpotSetting>, deploy: &Deploy ){
        let vec_len = spot_vector.len();
        if vec_len == 0 { return; };

        for i in 0..vec_len {
            let spot_setting = &spot_vector[ i ];
            self.generate_spot( deploy, spot_setting );
        }
    }

    fn generate_spot( &mut self, deploy: &Deploy, spot_setting: &SpotSetting ){
        let mut rng = rand::thread_rng();
        for _ in 0..spot_setting.amount {
            let random_num = rng.gen_range( 0..100 ); //100%
            if random_num >= spot_setting.emerging { continue; };


            let ground_type = spot_setting.ground_type.clone();
            let cover_type: CoverType = if ground_type == GroundType::Rock || ground_type == GroundType::Dirt {
                CoverType::None
            }else{
                spot_setting.cover_type.clone()
            };

            let ground_data = deploy.tile.get_ground_tile_deploy( &ground_type );
            let cover_data = deploy.tile.get_cover_tile_deploy( &cover_type );
            let max_width: u16 = spot_setting.max_width;
            let max_height: u16 = spot_setting.max_height;
            let min_width: u16 = spot_setting.min_width;
            let min_height: u16 = spot_setting.min_height;
            let x_offset: i8 = spot_setting.x_offset;
            let y_offset: i8 = spot_setting.y_offset;
            let height_offset: i8 = spot_setting.height_offset;
            let width_offset: i8 = spot_setting.width_offset;

            if self.tilemap_height <= max_height { panic!( "ground_tilemap.generate_spot; Map Height: {}, Max Height: {} ", self.tilemap_height, max_height )};
            if self.tilemap_width <= max_width { panic!( "ground_tilemap.generate_spot; Map Width: {}, Max Width: {} ", self.tilemap_width, max_width )};
            let starting_point_x = rng.gen_range( 0..( self.tilemap_width - max_width ));
            let starting_point_y = rng.gen_range( 0..( self.tilemap_height - max_height ));

            let mut current_width = rng.gen_range( min_width..( max_width +1 ));
            let mut current_height = rng.gen_range( min_height..( max_height +1 ));

            let average_width = (( min_width + max_width ) / 2 ) as u16;
            let average_height: u16 = (( min_height + max_height ) / 2 ) as u16;

            let mut left_top_point_x = starting_point_x;
            let mut left_top_point_y: u16 = starting_point_y;

            // do horizontal lines
            for i in 0..average_height {
                let left_top_point_x_i32:i32 = left_top_point_x as i32 + rng.gen_range( -x_offset..( x_offset +1 )) as i32;
                if left_top_point_x_i32 < 0 { 
                    left_top_point_x = 0; 
                }else{ 
                    left_top_point_x = left_top_point_x_i32 as u16; 
                };

                current_width = ( current_width as i32 + rng.gen_range( -width_offset..( width_offset +1 )) as i32 ) as u16;
                if current_width > max_width { current_width = max_width };
                if current_width < min_width { current_width = min_width };

                let y = starting_point_y + i;
                for j in 0..current_width {
                    let x: u16 = left_top_point_x + j;
                    let index: usize = y as usize * self.tilemap_height as usize + x as usize;
                    if index > self.total_tiles { continue; };

                    let mut tile = self.get_tile_by_index_mut( index );
                    match cover_type {
                        CoverType::None =>{
                            tile.ground_type = ground_type.clone();
                            tile.cover_type = cover_type.clone();
                            Tilemap::set_data_to_tile( tile, &ground_data );
                        },
                        _ =>{
                            tile.cover_type = cover_type.clone();
                            Tilemap::set_data_to_tile( tile, &cover_data );
                        },
                    };
                }
            }

            for k in 0..average_width {
                let left_top_point_y_i32 = left_top_point_y as i32 + ( rng.gen_range( -y_offset..( y_offset +1 ))) as i32;
                if left_top_point_y_i32 < 0 {
                    left_top_point_y = 0;
                }else{
                    left_top_point_y = left_top_point_y_i32 as u16;
                }
                
                let current_height_i32 = current_height as i32 + ( rng.gen_range( -height_offset..( height_offset +1 ))) as i32;
                if current_height_i32 < 1 {
                    current_height = 1;
                }else{
                    current_height = current_height_i32 as u16;
                }

                if current_height > max_height { current_height = max_height };
                if current_height < min_height { current_height = min_height };

                let x = starting_point_x + k;
                for l in 0..current_height {
                    let y = left_top_point_y + l;
                    let index = y as usize * self.tilemap_height as usize + x as usize;
                    if  index > self.total_tiles { continue; };

                    let mut tile = self.get_tile_by_index_mut( index );
                    match cover_type {
                        CoverType::None =>{
                            tile.ground_type = ground_type.clone();
                            tile.cover_type = cover_type.clone();
                            Tilemap::set_data_to_tile( tile, &ground_data );
                        },
                        _ =>{
                            tile.cover_type = cover_type.clone();
                            Tilemap::set_data_to_tile( tile, &cover_data );
                        },
                    };
                }
            }
        }

    }

    fn generate_rivers( &mut self, river_vector: &Vec<RiverSetting>, deploy: &Deploy ){
        let vec_len = river_vector.len();
        if vec_len == 0 { return; };

        for i in 0..vec_len {
            let river_setting = &river_vector[ i ];
            self.generate_river( &river_setting, &deploy );
        }
    }

    fn generate_river( &mut self, river_setting: &RiverSetting, deploy: &Deploy ){
        let mut rng = rand::thread_rng();
        let mut random_num: u8 = rng.gen_range( 0..100 ); // 100%
        if random_num >= river_setting.emerging { return; };

        let max_width = river_setting.max_width;
        let min_width = river_setting.min_width;
        let offset = river_setting.offset;
        let offset_width = river_setting.offset_width;

        let mut current_width = rng.gen_range( min_width..max_width );
        let mut river_type = river_setting.river_type.clone();

        if river_type == RiverType::Random{
            random_num = rng.gen_range( 0..2 );
            if random_num == 0 {
                river_type = RiverType::Vertical;
            }else{
                river_type = RiverType::Horizontal;
            }
        }

        match river_type{
            RiverType::Horizontal => {
                let mut river_point_y = rng.gen_range(( current_width as i32 + offset as i32 ) as u16..( 1 + self.tilemap_height as i32 - ( current_width as i32 + offset as i32 )) as u16 );
                for i in 0..self.tilemap_width {
                    let river_point_y_i32 = river_point_y as i32 + ( rng.gen_range( - offset as i32..( offset + 1 ) as i32 ));
                    river_point_y = river_point_y_i32 as u16;
                    if river_point_y_i32 < 0 { continue; };
                    if river_point_y_i32 > self.tilemap_height as i32 { river_point_y = self.tilemap_height; };

                    let current_width_i32: i32 = current_width as i32 + rng.gen_range( -offset_width as i32..( offset_width + 1 ) as i32 );
                    current_width = if current_width_i32 < min_width as i32 { min_width }
                        else if current_width_i32 > max_width as i32 { max_width }
                        else { current_width_i32 as u16 };
                    
                    for j in 0..current_width {
                        let index = ( river_point_y + j ) * self.tilemap_height + i;
                        if index as usize >= self.total_tiles { continue; };
                        let tile = self.get_tile_by_index_mut( index as usize );
                        let mut tile_data: &TileDeploy = deploy.tile.get_ground_tile_deploy( &river_setting.ground_type );

                        if river_setting.cover_type == CoverType::None  { 
                            tile.ground_type = river_setting.ground_type.clone();
                            if tile.ground_type == GroundType::Rock || tile.ground_type == GroundType::Dirt {
                                tile.cover_type = CoverType::None;
                            };
                        }else{
                           tile_data = deploy.tile.get_cover_tile_deploy( &river_setting.cover_type );
                           tile.cover_type = river_setting.cover_type.clone();
                        };

                        Tilemap::set_data_to_tile( tile, tile_data );
                    }
                };
            },
            RiverType::Vertical => {
                let mut river_point_x = rng.gen_range(( current_width as i32 + offset as i32 ) as u16 .. ( 1 + self.tilemap_width as i32 - ( current_width as i32 + offset as i32 )) as u16 );
                for i in 0..self.tilemap_height {
                    let river_point_x_i32 = river_point_x as i32 + rng.gen_range( -offset as i32..( offset +1 ) as i32 );
                    if river_point_x_i32 < 0 { continue; };

                    river_point_x = if river_point_x_i32 > self.tilemap_width as i32 { self.tilemap_width }
                        else { river_point_x_i32 as u16 };

                    let current_width_i32: i32 = current_width as i32 + rng.gen_range( -offset_width as i32..( offset_width + 1 ) as i32 );
                    current_width = if current_width_i32 < min_width as i32 { min_width }
                        else if current_width_i32 > max_width as i32 { max_width }
                        else { current_width_i32 as u16 };
                    for j in 0..current_width {
                        let index = river_point_x + j + self.tilemap_height * i;
                        if index as usize >= self.total_tiles { continue };

                        let tile = self.get_tile_by_index_mut( index as usize );
                        let mut tile_data: &TileDeploy = deploy.tile.get_ground_tile_deploy( &river_setting.ground_type );

                        if river_setting.cover_type == CoverType::None  { 
                            tile.ground_type = river_setting.ground_type.clone();
                            if tile.ground_type == GroundType::Rock || tile.ground_type == GroundType::Dirt {
                                tile.cover_type = CoverType::None;
                            };
                        }else{
                           tile_data = deploy.tile.get_cover_tile_deploy( &river_setting.cover_type );
                           tile.cover_type = river_setting.cover_type.clone();
                        };

                        Tilemap::set_data_to_tile( tile, tile_data );
                    }
                }
            },
            _ => panic!(" Unknown river type: {:?}", river_type ),
        }
    }

    fn generate_environment( &mut self, deploy: &Deploy, enviroument: u8 ){
        let height: u16 = self.get_tilemap_height();
        let total_tiles: usize = self.get_total_tiles();
        let mut rng = rand::thread_rng();

        let max_envirounment: u8 = enviroument;

        for i in 0..self.tilemap_tile_storage.len(){
            let x = self.tilemap_tile_storage[ i ].x;
            let y = self.tilemap_tile_storage[ i ].y;
            let tile_cover_type: CoverType = self.tilemap_tile_storage[ i ].cover_type.clone();
            let tile_ground_type: GroundType = self.tilemap_tile_storage[ i ].ground_type.clone();   
            
            //рандомно выбираем "подложку" 0 - 1 - 2 по умолчанию
            let current_envirounment = rng.gen_range( 0..max_envirounment + 1 );
            if current_envirounment == 0 { continue; };

            let grid_multiplier = current_envirounment * 2 + 1; // окружность вокруг тайла ( CurEnv = 1; x = 3, y = 3 ( 3 x 3 ) ); 

            for i in 0..grid_multiplier {
                for j in 0..grid_multiplier {
                    let index_i32: i32 = ( y as i32 - current_envirounment as i32 + i as i32 ) * height as i32 + ( x as i32 - current_envirounment as i32 + j as i32 );
                    if index_i32 < 0 || index_i32 >= total_tiles as i32 { continue; }; // защита от значений не принадлежащих текущей карте

                    let mut environment_tile: &mut Tile = self.get_tile_by_index_mut( index_i32 as usize );

                    match tile_ground_type {
                        GroundType::Rock => {
                            //do rock_environment;
                            if environment_tile.ground_type == GroundType::RockEnvironment || environment_tile.ground_type == GroundType::Rock {
                                continue;
                            }else{
                                let data_tile: &TileDeploy = deploy.tile.get_ground_tile_deploy( &GroundType::RockEnvironment );
                                environment_tile.ground_type = GroundType::RockEnvironment.clone();
                                if environment_tile.cover_type == CoverType::Grass || environment_tile.cover_type == CoverType::Flowers {
                                    environment_tile.cover_type = CoverType::None.clone();
                                };
                                
                                Tilemap::set_data_to_tile( environment_tile, data_tile );
                            }
                        },
                        _ => {
                            match tile_cover_type {
                                CoverType::Water => {
                                    //do water invironment;
                                    if environment_tile.cover_type == CoverType::Water || environment_tile.cover_type == CoverType::Shallow {
                                        continue;
                                    }else{
                                        let cover_type = CoverType::Shallow;
                                        let data_tile = deploy.tile.get_cover_tile_deploy( &cover_type );
                                        environment_tile.cover_type = cover_type;
                                        Tilemap::set_data_to_tile( environment_tile, data_tile );
                                    }
                                },
                                _ => {},
                            };
                        },
                    };
                }
            }
        }
        
    }

    fn spread_indexes_for_cover_tiles( &mut self ){
        for i in 0..self.tilemap_tile_storage.len(){
            let x = self.tilemap_tile_storage[ i ].x;
            let y: u16 = self.tilemap_tile_storage[ i ].y;
            let tile_cover: CoverType = self.tilemap_tile_storage[ i ].cover_type.clone();

            let cover_graphic_index: u8 = match tile_cover{
                CoverType::Water | CoverType::Shallow | CoverType::Ice => { self.find_cover_graphic_index_for_shallow_water_ice( x, y ) },
                _ => { continue },
            };

            self.tilemap_tile_storage[ i ].cover_graphic_index = cover_graphic_index;
        }
    }

    fn generate_solids_liquids( &mut self, spots: &Spots, rivers: &Rivers, deploy: &Deploy ){
        let solid_rivers = &rivers.solid_river;
        let solid_spots = &spots.solid_spot;
        let liquid_rivers = &rivers.liquid_river;
        let liquid_spots = &spots.liquid_spot;

        //first generate solids, then liquids;
        self.generate_spots( &solid_spots, &deploy );
        self.generate_rivers( &solid_rivers, &deploy );
        self.generate_spots( &liquid_spots, &deploy );
        self.generate_rivers( &liquid_rivers, &deploy );
    }

    fn find_cover_graphic_index_for_shallow_water_ice( &self, x: u16, y: u16 ) -> u8{
        let storage: &Vec<Tile> = &self.tilemap_tile_storage;
        let height = self.tilemap_height;
        let total_tiles = self.total_tiles;

        let top_index: isize = if y == 0{ -1 }else{(( y - 1 ) * height + x ) as isize};
        let left_index: isize = ( y * height + x - 1 ) as isize;
        let right_index: isize = ( y * height + x + 1 ) as isize;
        let bottom_index: isize = (( y + 1 ) * height + x ) as isize;

        let top:bool = if top_index < 0 || top_index as usize >= total_tiles {
            false
        }else{
            match storage[ top_index as usize ].cover_type {
                CoverType::Shallow | CoverType::Water | CoverType::Ice => true,
                _ => false,
            }
        };
        
        let left: bool = if left_index < 0 || left_index as usize >= total_tiles {
            false
        }else{
            match storage[ left_index as usize ].cover_type {
                CoverType::Shallow | CoverType::Water | CoverType::Ice => true,
                _ => false,
            }
        };

        let right: bool = if right_index < 0 || right_index as usize >= total_tiles {
            false
        }else{
            match storage[ right_index as usize ].cover_type {
                CoverType::Shallow | CoverType::Water | CoverType::Ice => true,
                _ => false,
            }
        };

        let bottom: bool = if bottom_index < 0 || bottom_index as usize >= total_tiles {
            false
        }else{
            match storage[ bottom_index as usize ].cover_type {
                CoverType::Shallow | CoverType::Water | CoverType::Ice => true,
                _ => false,
            }
        };

        if top && left && right && bottom {
            return 0; // all
        }else if top && left && right && !bottom {
            return 1; // top + left + right;
        }else if top && left && !right && bottom {
            return 2; // top + left + bottom;
        }else if top && !left && right && bottom {
            return 3; // top + right + bottom;
        }else if !top && left && right && bottom {
            return 4; // left + right + bottom;
        }else if top && !left && !right && bottom {
            return 5; // top + bottom;
        }else if !top && left && right && !bottom {
            return 6; // left + right;
        }else if top && left && !right && !bottom {
            return 7; // top + left;
        }else if top && !left && right && !bottom {
            return 8; // top + right;
        }else if !top && left && !right && bottom {
            return 9; // left + bottom;
        }else if !top && !left && right && bottom {
            return 10; // right + bottom;
        }else if top && !left && !right && !bottom {
            return  11; // top;
        }else if !top && left && !right && !bottom {
            return 12; // left;
        }else if !top && !left && right && !bottom {
            return 13; // right;
        }else if !top && !left && !right && bottom {
            return 14; // bottom;
        }else{
            return 15; // alone;
        }
    }

    fn set_data_to_tile( tile: &mut Tile, data: &TileDeploy ){
        tile.can_walk = data.can_walk;
        tile.movement_ratio = data.movement_ratio;
        tile.can_place_floor = data.can_place_floor;
        tile.can_place_thing = data.can_place_thing;
        tile.can_place_stuff = data.can_place_stuff;
        tile.can_remove_floor = data.can_remove_floor;
    }
}