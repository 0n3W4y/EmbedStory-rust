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
        #[reflect( ignore )]
    }



    pub fn new() -> Tile{
        Tile{

        }            
    }
}