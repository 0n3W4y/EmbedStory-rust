use serde::{ Deserialize };
use std::fs::File;
use std::io::prelude::*;


#[derive( Deserialize, Debug )]
pub struct SceneMiscellaneousDeploy{
    //TODO: include all scenes into;
    //loading:,
    //main_menu:,
    create_char: CreateCharSceneDeploy,
    //options:,
    //ground_scene:,
}

impl SceneMiscellaneousDeploy{
    pub fn new( path: &str ) -> Self{
        let data:SceneMiscellaneousDeploy  = match File::open( path ){
            Ok( mut file ) => {
                let mut contents = String::new();
                file.read_to_string( &mut contents ).unwrap();
                serde_json::from_str( &contents ).expect( "JSON was not well-formatted" )
            }
            Err( err ) => panic!( "Can not open cover data file: {}, {}", err, path ),
        };
        return data;
    }
}

#[derive( Deserialize, Debug )]
pub struct CreateCharSceneDeploy{
    
}