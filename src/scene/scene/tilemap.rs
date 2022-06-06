pub mod tilemap{
    use bevy::{
        prelude::*,
    };
    use std::collections::HashMap;

    pub enum GroundType{
        Earth,
        Rock,
        DryEarth,
        Dirt,    
    }

    pub enum CoverType{
        None,
        Snow,
        Water,
        Sand,
        WoodenFloor,
        Ice,
        Shallow,
    }

    pub struct TilemapConfig{

    }

    pub struct TileConfig{
        pos_x:u16,
        pos_y:u16,
        tile_size:u16,
        walkable:bool,
        ground:GroundType,
        cover:CoverType,
        movement_ratio:u16,
        place_cover:bool,
        place_object:bool,
        remove_cover:bool,
        remove_object:bool,
        index:u32,
    }

    #[derive( Component )]
    struct Tile{
        pub x:u16,
        pub y:u16,
        pub is_walkable:bool,
        pub ground_type:GroundType,
        pub cover_type:CoverType,
        pub movement_speed_ratio:u16,
        pub can_place_cover:bool,
        pub can_remove_cover:bool,
        pub can_place_object:bool,
        pub can_remove_object:bool,
        pub tile_size:u16,
        pub index:u32,
    }

    impl Tile{
        pub fn get_graphics_position( &self )-> Position{
            return Position{ x: self.x * self.tile_size, y: self.y * self.tile_size };
        }

        pub fn get_position( &self )-> Position{
            return Position{ x: self.x, y: self.y };
        }
    }


    #[derive( Component )]
    struct Tilemap{
        pub tiles:Vec<Tile>,
        pub width:u16,
        pub height:u16,
        pub tile_size:u16,
        pub tile_config:HashMap<u16, TileConfig>,
    }

    impl Tilemap{
        pub fn generate_tilemap(){

        }

        pub fn change_tile_ground( pos:Position, ground:GroundType ){

        }

        pub fn change_tile_cover( pos:Position, cover:CoverType ){

        }

        fn create_tile(){

        }
    }

    pub fn new( config: TilemapConfig ) -> Tilemap{
        Tilemap{
            
        }
    }

    pub fn create_tile( 
        config:TileConfig
    ) -> Tile{
        return Tile{
            x: config.pos_x,
            y: config.pos_y,
            tile_size: config.tile_size,
            is_walkable: config.walkable,
            ground_type: config.ground,
            cover_type: config.cover,
            movement_speed_ratio: config.movement_ratio,
            can_place_cover: config.place_cover,
            can_remove_cover: config.remove_cover,
            can_place_object: config.place_object,
            can_remove_object: config.remove_object,
            index: config.index,
        }            
    }
}