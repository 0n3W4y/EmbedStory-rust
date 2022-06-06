pub mod tile{
    use bevy::{
        prelude::*,
    };

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
    }

    #[derive( Component, Reflect )]
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
        pub tile_size:u16
        #[reflect( ignore )]
    }

    impl Tile{
        pub fn get_graphics_position()-> Position{
            return Position{ x: x * tile_size, y: y*tile_size };
        }

        pub fn get_position()-> Position{
            return Position{ x: x, y: y };
        }
    }



    pub fn new( 
        config:TileConfig
    ) -> Tile{
        return Tile{
            x = config.pos_x,
            y = config.pos_y,
            tile_size = config.tile_size,
            is_walkable = config.walkable,
            ground_type = config.ground,
            cover_type = config.cover,
            movement_speed_ratio = config.movement_speed,
            can_place_cover = config.place_cover,
            can_remove_cover = config.remove_cover,
            can_place_object = config.place_object,
            can_remove_object = config.remove_object,
        }            
    }
}