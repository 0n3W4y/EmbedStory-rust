pub mod tile;

use std::collections::HashMap;
use tile::*;
use crate::scene::*;


#[derive( Clone )]
pub struct TilemapConfig{
    width:u16,
    height:u16,
    tile_size:u16,
    tile_deploy: HashMap<u16, TileDeployConfig>,
}

#[derive( Clone )]
pub struct Tilemap{
    pub tiles:Vec<Tile>,
    pub width:u16,
    pub height:u16,
    pub tile_size:u16,
    //#[reflect( ignore )]
    pub tile_deploy: HashMap<u16, TileDeployConfig>,
}

impl Tilemap{
    pub fn generate_tilemap( &self, biome: &BiomeConfig ){
        self.generate_grid( &biome.groud_type,  );
        self.generate_additional_ground( &biome.ground_type_additional );
    }

    pub fn change_tile_ground( &self, index: u16, ground: GroundType ){
        let tile_deploy_config: &TileDeployConfig = self.get_tile_ground_config( ground );
        let tile: &mut Tile = &mut self.tiles[ index as usize ];
        self.change_tile( tile, &tile_deploy_config );
        tile.ground_type = ground;
    }

    pub fn change_tile_cover( &self, index :u16, cover: CoverType ){
        let tile_deploy_config: &TileDeployConfig = self.get_tile_cover_config( cover );
        let tile: &mut Tile = &mut self.tiles[ index as usize ];
        self.change_tile( tile, &tile_deploy_config );
        tile.cover_type = cover;
    }

    fn change_tile( &self, tile: &mut Tile, tile_config: &TileDeployConfig ){
        if tile_config.movement_ratio != 0 {
            tile.movement_speed_ratio = tile_config.movement_ratio;
        }
        tile.is_walkable = tile_config.walkable;
        tile.can_place_cover = tile_config.place_cover;
        tile.can_place_object = tile_config.place_object;
        tile.can_remove_cover = tile_config.remove_cover;
        tile.can_remove_object = tile_config.remove_object;
    }

    fn generate_grid( &self, ground: &GroundType ){
        let ground_type = ground.clone();
        let tile_deploy_config: &TileDeployConfig = self.get_tile_ground_config( ground_type );
        for i in 0..self.height{
            for j in 0..self.width{
                let index:u32 = ( i * self.height + j ) as u32;
                let tile = self.create_tile( ground_type, j, i, index );
                self.tiles.push( tile );
            }
        }
    }
    
    fn generate_additional_ground( additional_ground: &Vec<AdditionalGround> ){
        for some in additional_ground{
            let groun = some.0;
            let percent = some.1;
        }
    }

    fn create_tile( &self, ground_type:GroundType, x:u16, y:u16, index:u32  ) -> Tile{
        let tile_deploy_config: &TileDeployConfig = self.get_tile_ground_config( ground_type );
        let tile_config = TileConfig {
            pos_x: x,
            pos_y: y,
            tile_size: self.tile_size,
            walkable: tile_deploy_config.walkable,
            ground: tile_deploy_config.ground.clone(),
            cover: tile_deploy_config.cover.clone(),
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

    fn get_tile_ground_config( &self, ground_type: GroundType ) -> &TileDeployConfig{
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

    fn get_tile_cover_config( &self, cover_type: CoverType ) -> &TileDeployConfig{
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