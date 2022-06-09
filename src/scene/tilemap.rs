pub mod tile;

use std::collections::HashMap;
use tile::*;
use rand::{ thread_rng, Rng};
use crate::scene::*;

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




#[derive( Clone )]
pub struct TilemapConfig{
    width:u16,
    height:u16,
    tile_size:u16,
    tile_deploy: HashMap<u16, TileDeployConfig>,
}

#[derive( Clone )]
pub struct Tilemap {
    pub tiles: Vec<Tile>,
    pub width: u16,
    pub height: u16,
    pub tile_size: u16,
    pub tile_deploy: HashMap<u16, TileDeployConfig>,
}

impl Tilemap{
    pub fn generate_tilemap( &self, biome: &BiomeConfig ){
        self.generate_ground( &biome.groud_type  );
        self.generate_additional_ground( &biome.ground_type_additional );
        //self.generate_cover( &biome.cover_type );
        //self.generate_additional_cover( &biome.cover_type_additional );
    }

    pub fn change_tile_ground( &self, index: u16, ground: &GroundType ){
        let tile_deploy_config: &TileDeployConfig = self.get_tile_ground_config( ground );
        let tile: &mut Tile = &mut self.tiles[ index as usize ];
        self.change_tile_fields( tile, &tile_deploy_config );
        tile.ground_type = ground.clone();
    }

    pub fn change_tile_cover( &self, index :u16, cover: &CoverType ){
        let tile_deploy_config: &TileDeployConfig = self.get_tile_cover_config( cover );
        let tile: &mut Tile = &mut self.tiles[ index as usize ];
        self.change_tile_fields( tile, &tile_deploy_config );
        tile.cover_type = cover.clone();
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
    
    fn generate_additional_ground( &self, additional_ground: &Vec<AdditionalGround> ){
        for some in additional_ground{
            let ground = some.0.clone();
            let percent = some.1;
            let mut total_tiles: u32 = self.width as u32 * self.height as u32 * percent as u32 / 100;
            let mut rnd = thread_rng();
            let total_max_width:u16 = self.width / 20; // 5% of max width;
            let total_max_height: u16 = self.height / 20; // 5% of max height;
            for _ in 0..100 {
                let max_width = rnd.gen_range( 1..total_max_width );
                let max_height = rnd.gen_range( 1..total_max_height );
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
                self.generate_liquid_solid( config, ground, tile::CoverType::Nothing );

                total_tiles -= (( max_width as u32 + min_width as u32 ) / 2 ) * (( max_height as u32 + min_height as u32 ) / 2 );
                if total_tiles <= 20 {
                    break;
                }
            }            
        }
    }

    fn generate_liquid_solid( &self, config: LiquidSolidConfig, ground: GroundType, cover: CoverType ){
        let mut rnd = thread_rng();       
        for _ in 0..config.amount { 
            let random_num = rnd.gen_range( 0..99 ); // 100%;
            if config.emerging <= random_num {
                continue;
            }
            
            var startingPointX:Int = Math.floor( Math.random() * ( this.width - widthMax )); // random starting point on x with safe distance on right;
            var startingPointY:Int = Math.floor( Math.random() * ( this.height - heightMax )); // random starting point on y with safe distance on bottom;
            var currentWidth:Int = Math.floor( widthMin + Math.random() * ( widthMin + widthMax + 1 ));
            var currentHeight:Int = Math.floor( heightMin + Math.random() * ( heightMax + heightMin + 1 ));

            var leftTopPointX:Int = startingPointX;
            var leftTopPointY:Int = startingPointY;
            var averageWidth:Int = Math.round(( widthMax + widthMin) / 2 );
            var averageHeight:Int = Math.round(( heightMax + heightMin ) / 2 );

            for( i in 0...averageHeight ){ // do horizontal lines;
                leftTopPointX += Math.floor( -offsetX + Math.random()*( offsetX*2 + 1 ));
                currentWidth += Math.floor( -widthOffset + Math.random()*( widthOffset*2 + 1 ));
                if( currentWidth > widthMax )
                    currentWidth = widthMax;

                if( currentWidth < widthMin )
                    currentWidth = widthMin;

                var y:Int = startingPointY + i;
                for( j in 0...currentWidth ){
                    var x:Int = leftTopPointX + j;
                    var index:Int = y * this.height + x;
                    if( index < 0 || index >= this._totalTiles )
                        continue;

                    var tile:Tile = this.tileStorage[ index ];
                    if( params.FloorType == null ){
                        tile.changeGroundType( groundTileConfig );
                        tile.changeFloorType( floorTileConfigForNothing );
                        this._createGroundEnvironment( tile );
                    }else{
                        tile.changeFloorType( floorTileConfig );
                    }                    
                }
            }

            for( k in 0...averageWidth ){ // do vertical lines;
                leftTopPointY = Math.floor( -offsetY + Math.random()*( offsetY*2 + 1 ));
                currentHeight = Math.floor( -heightOffset + Math.random()*( heightOffset*2 + 1));
                if( currentHeight > heightMax )
                    currentHeight = heightMax;

                if( currentHeight < heightMin )
                    currentHeight = heightMin;

                var x:Int = startingPointX + k;
                for( l in 0...currentHeight ){
                    var y:Int = leftTopPointY + l;
                    var index:Int = y * this.height + x;
                    if( index < 0 || index >= this._totalTiles )
                        continue;

                    var tile:Tile = this.tileStorage[ index ];
                    if( params.FloorType == null ){
                        tile.changeGroundType( groundTileConfig );
                        tile.changeFloorType( floorTileConfigForNothing );
                        this._createGroundEnvironment( tile );
                    }else{
                        tile.changeFloorType( floorTileConfig );
                    }
                }
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
            cover: tile::CoverType::Nothing,
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
        match ground_type {
            Earth => return &self.tile_deploy[ &100 ],
            Rock => return &self.tile_deploy[ &103 ],
            DryEarth => return &self.tile_deploy[ &101 ],
            Dirt => return &self.tile_deploy[ &102 ],
            Sandrock => return &self.tile_deploy[ &104 ],
            RockEnvirounment => return &self.tile_deploy[ &105 ],
            SandrockEnvironment =>  return &self.tile_deploy[ &106 ],
            _ => {
                let mut emsg = self.error_message();
                emsg += &"create_tile. wrong type 'ground_type'".to_string();
                panic!("{}:{:?}", emsg, ground_type );
            }, 
        };
    }

    fn get_tile_cover_config( &self, cover_type: &CoverType ) -> &TileDeployConfig{
        match cover_type {
            Nothing => return &self.tile_deploy[ &120 ],
            Grass => return &self.tile_deploy[ &121 ],
            Snow => return &self.tile_deploy[ &123 ],
            Water => return &self.tile_deploy[ &124 ],
            Sand => return &self.tile_deploy[ &122 ],
            WoodenFloor => return &self.tile_deploy[ &127 ],
            Ice => return &self.tile_deploy[ &125 ],
            Shallow => return &self.tile_deploy[ &126 ],
            _ => {
                let mut emsg = self.error_message();
                emsg += &"get_tile_cover_config. wrong type 'cover_type'".to_string();
                panic!("{}:{:?}", emsg, cover_type );
            }
        };
    }

    fn error_message( &self ) -> String{
        return "Error in tilemap.".to_string();
    }
}

pub fn new( config: TilemapConfig ) -> Tilemap{
    return Tilemap{
        tiles: vec![],
        width: config.width,
        height: config.height,
        tile_size: config.tile_size,
        tile_deploy: config.tile_deploy,
    }
}