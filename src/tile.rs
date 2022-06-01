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
        Nothing,
        Snow,
        Water,
        Sand,
        WoodenFloor,
        Ice,
        Shallow,
    }

    #[derive( Component, Reflect )]
    struct Tile{
        pub position:Position,
        pub is_walkable:bool,
        pub ground_type:GroundType,
        pub cover_type:CoverType,
        pub movement_speed_ratio:i32,
        pub can_place_cover:bool,
        pub can_remove_cover:bool,
        pub can_place_object:bool,
        pub can_remove_object:bool,
        #[reflect( ignore )]
    }

    impl Tile{
        pub fn change_cover_type( cover:CoverType ){

        }
    }



    pub fn new( pos:Position, walable:bool, ground:GroundType, cover: CoverType ) -> Tile{
        Tile{

        }            
    }
}