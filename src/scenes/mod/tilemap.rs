#[path = "mod/tile.rs"] mod tile;

use bevy::utils::HashMap;
use tile::*;
use rand::{ thread_rng, Rng};
use serde::{ Deserialize, Serialize};

#[derive( PartialEq, Eq, Clone, Copy )]
pub enum RiverType{
    Horizontal,
    Vertical,
    Generate,
}

#[derive( Deserialize, Serialize )]
pub struct TilemapGeneratorConfig{
    ground: GroundType,
    cover: CoverType,
    additional_cover: HashMap< CoverType, u8>,
    additional_ground: HashMap< GroundType, u8>,
    rock:,
    lake:,
    river:,
}

#[derive( Copy, Clone )]
pub struct LiquidSolidConfig{
    emerging: u8,
    amount: u16,
    widthMax: u16,
    widthMin: u16,
    heightMax: u16,
    heightMin: u16,
    offsetX: u16,
    offsetY: u16,
    widthOffset: u16,
    heightOffset: u16,
}

#[derive( Copy, Clone )]
pub struct LiquidSolidRiverConfig {
    pub emerging: u8,
    pub widthMax: u16,
    pub widthMin: u16,
    pub offset: u16,
    pub widthOffset: u16,
    pub river_type: RiverType,
}

#[derive( Clone )]
pub struct TilemapConfig{
    pub width:u16,
    pub height:u16,
    pub tile_size:u16,
}

#[derive( Deserialize, Serialize )]
pub struct Tilemap {
    pub tiles: Vec<Tile>,
    pub width: u16,
    pub height: u16,
    pub tile_size: u16,
}

impl Tilemap{
    pub fn init( &mut self, config: TilemapConfig ){
        self.width = config.width;
        self.height = config.height;
        self.tile_size = config.tile_size;
    }

    pub fn generate_tilemap( &self, config: TilemapGeneratorConfig ){
        self.generate_ground( config.ground );
        self.generate_additional_ground( config.additional_ground );
        self.generate_rocks( config.rock );
        self.generate_cover( config.cover );
        self.generate_additional_cover( config.additional_cover );
        self.generate_lakes( config.lake );
        self.generate_rivers( config.river );
    }

    pub fn change_tile_ground( &self, index: u32, ground: &GroundType ){
        let tile_deploy_config: &TileDeployConfig = self.get_tile_ground_config( ground );
        let tile: &mut Tile = &mut self.tiles[ index as usize ];
        self.change_tile_fields( tile, &tile_deploy_config );
        tile.ground_type = ground.clone();
    }

    pub fn change_tile_cover( &self, index :u32, cover: &CoverType ){
        let tile_deploy_config: &TileDeployConfig = self.get_tile_cover_config( cover );
        let tile: &mut Tile = &mut self.tiles[ index as usize ];
        self.change_tile_fields( tile, &tile_deploy_config );
        tile.cover_type = cover.clone();
    }

    fn generate_rocks( &self, config: &Vec<SolidRock> ){
        for solids in config{
            let solid_config = &solids.1;
            let ground = solid_config.ground.clone();
            let liquid_solid_config = LiquidSolidConfig {
                emerging: solid_config.emerging,
                amount: solid_config.amount,
                widthMax: solid_config.widthMax,
                widthMin: solid_config.widthMin,
                heightMax: solid_config.heightMax,
                heightMin: solid_config.heightMin,
                offsetX: solid_config.offsetX,
                offsetY: solid_config.offsetY,
                widthOffset: solid_config.widthOffset,
                heightOffset: solid_config.heightOffset,
            };
            self.generate_liquid_solid( liquid_solid_config, ground, CoverType::Nothing );
        }
    }

    fn generate_rivers( &self, config: &Vec<River> ){
        for river in config{
            let riv_config = &river.1;
            let cover = riv_config.cover.clone();

            let river_config = LiquidSolidRiverConfig{
                emerging: riv_config.emerging,
                widthMax: riv_config.widthMax,
                widthMin: riv_config.widthMin,
                offset: riv_config.offset,
                widthOffset: riv_config.widthOffset,
                river_type: riv_config.river_type
            };
            self.generate_liquid_solid_river( river_config, GroundType::None, cover );
        }        
    }

    fn generate_lakes( &self, config: &Vec<Lake> ){
        for lake in config{
            let lake_config = &lake.1;
            let cover = lake_config.cover.clone();
            let liquid_solid_config = LiquidSolidConfig {
                emerging: lake_config.emerging,
                amount: lake_config.amount,
                widthMax: lake_config.widthMax,
                widthMin: lake_config.widthMin,
                heightMax: lake_config.heightMax,
                heightMin: lake_config.heightMin,
                offsetX: lake_config.offsetX,
                offsetY: lake_config.offsetY,
                widthOffset: lake_config.widthOffset,
                heightOffset: lake_config.heightOffset,
            };
            self.generate_liquid_solid( liquid_solid_config, GroundType::Nothing, cover );
        }
    }

    fn change_tile_fields( &self, tile: &mut Tile, tile_config: &TileDeployConfig ){
        if tile_config.movement_ratio != 0 {
            tile.movement_speed_ratio = tile_config.movement_ratio;
        }
        tile.is_walkable = tile_config.walkable;
        tile.can_place_cover = tile_config.place_cover;
        tile.can_place_object = tile_config.place_object;
        tile.can_remove_cover = tile_config.remove_cover;
        tile.can_remove_object = tile_config.remove_object;
    }

    fn generate_ground( &self, ground: &GroundType ){
        let tile_deploy_config: &TileDeployConfig = self.get_tile_ground_config( &ground );
        for i in 0..self.height{
            for j in 0..self.width{
                let index:u32 = ( i * self.height + j ) as u32;
                let tile = self.create_tile( ground, j, i, index );
                self.tiles.push( tile );
            }
        }
    }

    fn generate_cover( &self, cover: &CoverType ){
        for i in 0..self.tiles.len(){
            let tile:&Tile = &self.tiles[ i ];
            if cover == &CoverType::Grass && ( tile.ground_type == GroundType::RockEnvirounment || tile.ground_type == GroundType::SandrockEnvirounment ) {
                continue;
            }else{
                self.change_tile_cover( i as u32, cover );
            }
            
        }

    }

    fn generate_additional_cover( &self, additional_cover: &Vec<AdditionalCover> ){
        for some in additional_cover{
            let cover = some.0.clone();
            let percent = some.1;
            let mut total_tiles: u32 = self.width as u32 * self.height as u32 * percent as u32 /100;
            let mut rnd = thread_rng();
            let total_max_width:u16 = self.width / 20; // 5% of max width;
            let total_max_height: u16 = self.height / 20; // 5% of max height;
            for _ in 0..100 {
                let max_width = rnd.gen_range( 1..=total_max_width );
                let max_height = rnd.gen_range( 1..=total_max_height );
                let min_width: u16 = max_width / 4; // 25% of current value;
                let min_height: u16 = max_height / 4; // 25%

                let config = LiquidSolidConfig {
                    emerging: 100,
                    amount: 1,
                    widthMax: max_width,
                    widthMin: min_width,
                    heightMax: max_height,
                    heightMin: min_height,
                    offsetX: 1,
                    offsetY: 1,
                    widthOffset: 1,
                    heightOffset: 1,
                };
                self.generate_liquid_solid( config, GroundType::Nothing, cover );

                total_tiles -= (( max_width as u32 + min_width as u32 ) / 2 ) * (( max_height as u32 + min_height as u32 ) / 2 );
                if total_tiles <= 20 {
                    break;
                }
            }
        }
    }
    
    fn generate_additional_ground( &self, additional_ground: &Vec<AdditionalGround> ){
        for some in additional_ground {
            let ground = some.0.clone();
            let percent = some.1;
            let mut total_tiles: u32 = self.width as u32 * self.height as u32 * percent as u32 / 100;
            let mut rnd = thread_rng();
            let total_max_width:u16 = self.width / 20; // 5% of max width;
            let total_max_height: u16 = self.height / 20; // 5% of max height;
            for _ in 0..100 {
                let max_width = rnd.gen_range( 1..=total_max_width );
                let max_height = rnd.gen_range( 1..=total_max_height );
                let min_width: u16 = max_width / 4; // 25% of current value;
                let min_height: u16 = max_height / 4; // 25%

                let config = LiquidSolidConfig {
                    emerging: 100,
                    amount: 1,
                    widthMax: max_width,
                    widthMin: min_width,
                    heightMax: max_height,
                    heightMin: min_height,
                    offsetX: 1,
                    offsetY: 1,
                    widthOffset: 1,
                    heightOffset: 1,
                };
                self.generate_liquid_solid( config, ground, CoverType::Nothing );

                total_tiles -= (( max_width as u32 + min_width as u32 ) / 2 ) * (( max_height as u32 + min_height as u32 ) / 2 );
                if total_tiles <= 20 {
                    break;
                }
            }            
        }
    }

    fn generate_liquid_solid_river( &self, config: LiquidSolidRiverConfig, ground: GroundType, cover: CoverType ){
        let mut rnd = thread_rng();
        let total_tiles: u32 = self.height as u32 * self.width as u32;

        let random_num = rnd.gen_range( 0..=100 ); // 100%;
        if config.emerging <= random_num {
            return;
        }

        let mut width = rnd.gen_range( config.widthMin..=config.widthMax );
        let mut river_type = config.river_type;
        if river_type == RiverType::Generate {
            let random = rnd.gen_range( 0..=1 );
            if random == 0{
                river_type = RiverType::Horizontal;
            }else{
                river_type = RiverType::Vertical;
            }
        }

        match river_type {
            RiverType::Horizontal => {
                let mut river_point = rnd.gen_range( width + config.offset..=( self.height - width - config.offset ));
                for i in 0..self.width {
                    river_point = ( river_point as i16 + rnd.gen_range(-(config.offset as i16)..=( config.offset as i16 * 2 ))) as u16;
                    width = ( width as i16 + rnd.gen_range( -( config.widthOffset as i16 )..=( config.widthOffset as i16 * 2 ))) as u16;

                    if width > config.widthMax {
                        width = config.widthMax;
                    }else if width < config.widthMin{
                        width = config.widthMin;
                    }

                    for j in 0..width {
                        let index: u32 = (( river_point + j ) * self.height + i ) as u32;
                        if index < 0 || index >= total_tiles{
                            continue;
                        }

                        if cover == CoverType::Nothing {
                            self.change_tile_ground( index, &ground );
                        }else{
                            self.change_tile_cover( index, &cover );
                        }
                        
                    }
                }
            },
            RiverType::Vertical => {
                let river_point = rnd.gen_range( width + config.offset..=( self.width - width - config.offset ));
                for i in 0..self.height {
                    river_point = ( river_point as i16 + rnd.gen_range( -( config.offset as i16 )..=( config.offset as i16 * 2 ))) as u16;
                    width = ( width as i16 + rnd.gen_range( -( config.widthOffset as i16 )..=( config.widthOffset as i16 * 2 ))) as u16;
                    
                    if width > config.widthMax {
                        width = config.widthMax;
                    }else if width < config.widthMin {
                        width = config.widthMin;              
                    }

                    for j in 0..width {
                        let index: u32 = (( river_point + j ) * self.height + i ) as u32;
                        if index < 0 || index >= total_tiles{
                            continue;
                        }

                        if cover == CoverType::Nothing {
                            self.change_tile_ground( index, &ground );
                        }else{
                            self.change_tile_cover( index, &cover );
                        }
                    }                
                }
            },
        };
    }

    fn generate_liquid_solid( &self, config: LiquidSolidConfig, ground: GroundType, cover: CoverType ){
        let mut rnd = thread_rng();
        let total_tiles: u32 = self.height as u32 * self.width as u32;    
        for _ in 0..config.amount { 
            let random_num = rnd.gen_range( 0..=100 ); // 100%;
            if config.emerging <= random_num {
                continue;
            }
            
            let starting_point_x: u16 = rnd.gen_range(0..=( self.width - config.widthMax )); // random starting point on x with safe distance on right;
            let starting_point_y: u16 = rnd.gen_range( 0..=( self.height - config.heightMax )); // random starting point on y with safe distance on bottom;
            let mut current_width = rnd.gen_range( config.widthMin..config.widthMax );
            let mut current_height = rnd.gen_range( config.heightMin..config.heightMax );

            let offset_x: i16 = config.offsetX as i16;
            let offset_y: i16 = config.offsetY as i16;
            let width_offset: i16 = config.widthOffset as i16;
            let height_offset: i16 = config.heightOffset as i16;

            let mut left_top_x: u16 = starting_point_x;
            let mut left_top_y: u16 = starting_point_y;
            let average_width: u16 = ( config.widthMax + config.widthMin ) / 2;
            let average_height: u16 = ( config.heightMax + config.heightMin ) / 2;

            for i in 0..average_height { // do horizontal lines;
                left_top_x += ( rnd.gen_range( -offset_x..( offset_x*2 ))) as u16;
                current_width += ( rnd.gen_range( -width_offset..( width_offset*2 ))) as u16;
                if current_width > config.widthMax {
                    current_width = config.widthMax;
                }

                if current_width < config.widthMax{
                    current_width = config.widthMin;
                }

                let y: u16 = starting_point_y + i;
                for j in 0..current_width {
                    let x = left_top_x + j;
                    let index: u32 = y as u32 * self.height as u32 + x as u32;
                    if index < 0 || index >= total_tiles {
                        continue;
                    }

                    if cover == CoverType::Nothing{
                        self.change_tile_ground( index, &ground );
                    }else{
                        self.change_tile_cover( index, &cover );
                    }                    
                }
            }

            for k in 0..average_width { // do vertical lines;
                left_top_y =  ( rnd.gen_range( -offset_y..( offset_y*2 ))) as u16;
                current_height = ( rnd.gen_range( -height_offset..( height_offset * 2 ))) as u16;
                if current_height > config.heightMax {
                    current_height = config.heightMax;
                }

                if current_height < config.heightMin {
                    current_height = config.heightMin;
                }

                let x  = starting_point_x + k;
                for l in 0..current_height {
                    let y = left_top_y + l;
                    let index: u32 = y as u32 * self.height as u32 + x as u32;
                    if index < 0 || index >= total_tiles {
                        continue;
                    }

                    if cover == CoverType::Nothing{
                        self.change_tile_ground( index, &ground );
                        self.generate_ground_envirounment( index );
                    }else{
                        self.change_tile_cover( index, &cover );
                    }
                }
            }
        }
    }

    fn generate_ground_envirounment( & self, index: u32 ){
        let mut rnd = thread_rng();
        let total_tiles: u32 = self.height as u32 * self.width as u32;
        let tile: Tile = self.tiles[ index as usize ];
        let ground: GroundType;
        match tile.ground_type {
            GroundType::Rock => { ground = GroundType::RockEnvirounment},
            GroundType::Sandrock => { ground = GroundType::SandrockEnvirounment },
            _ => { return; },
        };

        //рандомно выбираем "подложку" 0 - 1 - 2 по умолчанию
        let number = 2; // радиус распространения подложки. в теории можно вынести в конфиг.
        let random_number = rnd.gen_range( 0..=2 ); // 0 - 2;
        let grid_multiplier = random_number * 2 + 1;
        if random_number == 0 {
            return;
        }

        for i in 0..grid_multiplier {
            for j in 0..grid_multiplier {
                let index: u32 = (( tile.y - random_number + i ) * self.height + ( tile.x - random_number + j )) as u32;
                if index < 0 || index >= total_tiles { // защита от значений не принадлежащих текущей карты
                    continue; 
                }

                let new_tile:Tile = self.tiles[ index as usize ];
                if tile.ground_type == new_tile.ground_type {
                    continue;
                }

                self.change_tile_ground( index, &ground );
            }
        }
    }

    fn create_tile( &self, ground_type: &GroundType, x:u16, y:u16, index:u32  ) -> Tile{
        let tile_deploy_config: &TileDeployConfig = self.get_tile_ground_config( ground_type );
        let tile_config = TileConfig {
            pos_x: x,
            pos_y: y,
            graph_x: x as u32 * self.tile_size as u32,
            graph_y: y as u32 * self.tile_size as u32,
            tile_size: self.tile_size,
            walkable: tile_deploy_config.walkable,
            ground: ground_type.clone(),
            cover: CoverType::Nothing,
            movement_ratio: tile_deploy_config.movement_ratio,
            place_cover: tile_deploy_config.place_cover,
            place_object: tile_deploy_config.place_object,
            remove_cover: tile_deploy_config.remove_cover,
            remove_object: tile_deploy_config.remove_object,
            index: index,
            cover_graphics_index: 0,
        };

        let tile = tile::new( tile_config );
        return tile;
    }

    fn get_tile_ground_config( &self, ground_type: &GroundType ) -> &TileDeployConfig{
        return self.deploy.get_tile_ground_config( ground_type );
        
    }

    fn get_tile_cover_config( &self, cover_type: &CoverType ) -> &TileDeployConfig{
        return self.deploy.get_tile_cover_config( cover_type );
    }

    fn error_message( &self ) -> String{
        return "Error in tilemap.".to_string();
    }
}

pub fn new() -> Tilemap{
    return Tilemap{
        tiles: vec![],
        width: 0,
        height: 0,
        tile_size: 0,
    };
}