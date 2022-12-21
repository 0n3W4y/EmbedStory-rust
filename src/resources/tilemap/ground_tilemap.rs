use serde::{ Serialize, Deserialize };
use std::vec;
use rand::Rng;

use crate::resources::{
    tilemap::tile::ground_tilemap_tile::{ GroundType, CoverType, GroundTilemapTile, GroundTilemapTileDeploy }, 
    deploy::{Deploy, GroundTilemapTileDeployData}
};

#[derive( Deserialize, Clone )]
pub enum BiomeType{
    Plain,
    Desert,
    Forest,
    Rocks,
    Tropic,
    Snow,
    Swamp,
}

#[derive( Deserialize, Clone, Debug, Eq, PartialEq )]
pub enum RiverType{
    Horizontal,
    Vertical,
    Random,
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

#[derive( Deserialize, Clone )]
pub struct SpotSetting {
    pub amount: u8,
    pub emerging: u8,
    pub ground_type: GroundType,
    pub cover_type: CoverType,
    pub max_width: u16,
    pub max_height: u16,
    pub min_width: u16,
    pub min_height: u16,
    pub x_offset: i8,
    pub y_offset: i8,
    pub height_offset: i8,
    pub width_offset: i8,
}

#[derive( Deserialize, Clone )]
pub struct RiverSetting {
    pub emerging: u8,
    pub ground_type: GroundType,
    pub cover_type: CoverType,
    pub max_width: u16,
    pub min_width: u16,
    pub offset: i8,
    pub offset_width: i8,
    pub river_type: RiverType,
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
                GroundTilemap::set_data_to_tile( &mut tile, &tile_setting );
                self.tilemap_tile_storage.push( tile );
            }
        }
    }

    fn generate_additional_ground( &mut self, additional_ground_type: &Vec<GroundType>, additional_ground_type_value: &Vec<f32>, deploy: &Deploy ){
        let additional_ground_num: usize = additional_ground_type.len();
        let mut rng = rand::thread_rng();
        for i in 0..additional_ground_num {
            let percent: f32 = additional_ground_type_value[ i ];
            let ground_type = additional_ground_type[ i ].clone();
            let mut remain_tiles: usize = ( self.total_tiles as f32 * percent  / 100.0 ) as usize;
            let max_width = ( self.tilemap_width * 5 / 100 ) as u16; // 5% of tilemap width;
            let max_height: u16 = ( self.tilemap_height * 5 / 100 ) as u16; // 5% of tilemap height;

            //guard for infinity loop;
            while remain_tiles > 10 {
                let current_max_width = rng.gen_range( 4..max_width );
                let current_max_height = rng.gen_range( 4..max_height );
                let current_min_width = rng.gen_range( 1..current_max_width / 4 ); // 25% of maximum value
                let current_min_height = rng.gen_range( 1..current_max_height / 4 ); // 25% of maximum value

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

                self.generate_spots( deploy, &spot_setting );
                let tiles_used = (( current_max_width + current_min_width ) / 2 ) * (( current_max_height + current_min_height ) / 2 );
                remain_tiles -= tiles_used as usize;
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

    fn generate_additional_cover( &mut self, additional_cover_type: &Vec<CoverType>, additional_cover_type_value: &Vec<f32>, deploy: &Deploy ){
        let additional_cover_num: usize = additional_cover_type.len();
        let mut rng = rand::thread_rng();
        for i in 0..additional_cover_num {
            let percent : f32 = additional_cover_type_value[ i ];
            let cover_type: CoverType = additional_cover_type[ i ].clone();
            let mut remain_tiles: usize = ( self.total_tiles as f32 * percent / 100.0 ) as usize;
            let max_width = ( self.tilemap_width * 5 / 100 ) as u16; // 5% of tilemap width;
            let max_height: u16 = ( self.tilemap_height * 5 / 100 ) as u16; // 5% of tilemap height;

            while remain_tiles > 10 {
                let current_max_width = rng.gen_range( 4..max_width );
                let current_max_height = rng.gen_range( 4..max_height );
                let current_min_width = rng.gen_range( 1..current_max_width / 4 ); // 25% of maximum value
                let current_min_height = rng.gen_range( 1..current_max_height / 4 ); // 25% of maximum value

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

                self.generate_spots( deploy, &spot_setting );
                let tiles_used = (( current_max_width + current_min_width ) / 2 ) * (( current_max_height + current_min_height ) / 2 );
                remain_tiles -= tiles_used as usize;
            }
        }
    }

    fn generate_spots( &mut self, deploy: &Deploy, spot_setting: &SpotSetting ){
        let mut rng = rand::thread_rng();
        for n in 0..spot_setting.amount {
            let random_num = rng.gen_range( 0..99 ); //100%
            if random_num >= spot_setting.emerging { continue; };


            let ground_type = spot_setting.ground_type.clone();
            let cover_type: CoverType = spot_setting.cover_type.clone();
            let ground_data = deploy.get_ground_tile_data( &ground_type );
            let cover_data = deploy.get_cover_tile_data( &cover_type );
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

            let mut current_width = rng.gen_range( min_width..max_width );
            let mut current_height = rng.gen_range( min_height..max_height );

            let average_width = (( min_width + max_width ) / 2 ) as u16;
            let average_height: u16 = (( min_height + max_height ) / 2 ) as u16;

            let mut left_top_point_x = starting_point_x;
            let mut left_top_point_y: u16 = starting_point_y;

            // do horizontal lines
            for i in 0..average_height {
                let left_top_point_x_i32:i32 = left_top_point_x as i32 + rng.gen_range( -x_offset..x_offset ) as i32;
                if left_top_point_x_i32 < 0 { 
                    left_top_point_x = 0; 
                }else{ 
                    left_top_point_x = left_top_point_x_i32 as u16; 
                };

                current_width = ( current_width as i32 + rng.gen_range( -width_offset..width_offset ) as i32 ) as u16;
                if current_width > max_width { current_width = max_width };
                if current_width < min_width { current_width = min_width };

                let y = starting_point_y + i;
                for j in 0..current_width {
                    let x: u16 = left_top_point_x + j;
                    let index: usize = y as usize * self.tilemap_height as usize + x as usize;
                    if index > self.total_tiles { continue; };

                    let mut tile = self.get_tile_by_index( index );
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

            for k in 0..average_width {
                let left_top_point_y_i32 = left_top_point_y as i32 + ( rng.gen_range( -y_offset..y_offset )) as i32;
                if left_top_point_y_i32 < 0 {
                    left_top_point_y = 0;
                }else{
                    left_top_point_y = left_top_point_y_i32 as u16;
                }
                
                let current_height_i32 = current_height as i32 + ( rng.gen_range( -height_offset..height_offset )) as i32;
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

                    let mut tile = self.get_tile_by_index( index );
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

    fn generate_river( &mut self, river_setting: &RiverSetting ){
        let mut rng = rand::thread_rng();
        let mut random_num: u8 = rng.gen_range( 0..99 ); // 100%
        if random_num >= river_setting.emerging { return; };

        let max_width = river_setting.max_width;
        let min_width = river_setting.min_width;
        let offset = river_setting.offset;
        let offset_width = river_setting.offset_width;

        let mut current_width = rng.gen_range( min_width..max_width );
        let mut river_type = river_setting.river_type.clone();

        if river_type == RiverType::Random{
            random_num = rng.gen_range( 0..1 );
            if random_num == 0 {
                river_type = RiverType::Vertical;
            }else{
                river_type = RiverType::Horizontal;
            }
        }

        match river_type{
            RiverType::Horizontal => {
                let mut river_point_y = rng.gen_range(( current_width as i32 + offet as i32 ) as u16..( self.tilemap_height as i32 - ( current_width as i32 + offset as i32 )) as u16 );
                for i in 0..self.tilemap_width {
                    river_point_y_i32 = river_point_y as i32 + ( rng.gen_range( - offset as i32..offset as i32 ));
                    river_point_y = river_point_y_i32 as u16;
                    if river_point_y_i32 < 0 { river_point_y = 0; }
                    if river_point_y_i32 > self.tilemap_height as i32 { river_point_y = self.tilemap_height; };
                }
        
                riverPoint += Math.floor( -offset + Math.random()* ( offset*2 + 1 ));
                currentRiverWidth += Math.floor( -widthOffset + Math.random()*( widthOffset*2 + 1 ));

                if( currentRiverWidth > widthMax )
                    currentRiverWidth = widthMax;
                else if( currentRiverWidth < widthMin )
                    currentRiverWidth = widthMin;

                for( j in 0...currentRiverWidth ){
                    var index = ( riverPoint + j ) * this.height + i;
                    if( index < 0 || index >= this._totalTiles )
                        continue;

                    var tile:Tile = this.tileStorage[ index ];
                    tile.changeFloorType( floorTileConfig );
                }
            }
            },
            RiverType::Vertical => {
                var riverPoint:Int = Math.floor( currentRiverWidth + offset + Math.random()* ( this.width - currentRiverWidth - offset ));
            for( i in 0...this.height ){
                riverPoint += Math.floor( -offset + Math.random() * ( offset*2 + 1 ));
                currentRiverWidth += Math.floor( -widthOffset + Math.random()*( widthOffset*2 + 1 ));
                
                if( currentRiverWidth > widthMax )
                    currentRiverWidth = widthMax;
                else if( currentRiverWidth < widthMin )
                    currentRiverWidth = widthMin;                
                
                for( j in 0...currentRiverWidth ){
                    var index:Int = riverPoint + j + this.height * i;
                    if( this._totalTiles <= index || index < 0)
                        continue;

                    var tile:Tile = this.tileStorage[ index ];
                    tile.changeFloorType( floorTileConfig );
                }                
            }
            },
            _ => panic!(" Unknown river type: {:?}", river_type ),
        }
    }

    fn generate_envirounment(){
        //рандомно выбираем "подложку" 0 - 1 - 2 по умолчанию
        var number:Int = 2; // радиус распространения подложки. в теории можно вынести в конфиг.
        var randomNumber:Int = Math.floor( Math.random()*( number + 1 )); // 0 - 2;
        if( randomNumber == 0 )
            return;

        var y:Int = tile.gridY;
        var x:Int = tile.gridX;
        var height:Int = this.height;        
        var gridMultiplier:Int = randomNumber * 2 + 1;

        for( i in 0...gridMultiplier ){
            for( j in 0...gridMultiplier ){
                var index:Int = ( y - randomNumber + i ) * height + ( x - randomNumber + j );
                if( index < 0 || index >= this._totalTiles ) // защита от значений не принадлежащих текущей карты
                    continue; 

                var newTile:Tile = this.tileStorage[ index ];
                var indexTileGroundType = newTile.groundType;
                if( indexTileGroundType == tileGroundType ) // защита от перезаписи существующих тайлов для текущего значения тайла.
                    continue;

                newTile.changeGroundType( newTileConfig );
                if( newTile.floorType == "grass" ) // заменяем только если покрытие это трава, снег и песок можно не заменять.
                    newTile.changeFloorType( tileFloorTypeConfig );
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