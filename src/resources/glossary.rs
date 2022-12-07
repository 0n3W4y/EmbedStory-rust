use serde::{ Deserialize, Serialize } ;
use std::fs::File;
use std::io::prelude::*;

use crate::resources::language::Language;
use crate::config::*;

#[derive( Serialize, Deserialize, Debug, Clone )]
pub struct Glossary{
    pub loading_scene_text: LoadingSceneText,
    pub main_menu_text: MainMenuText,
    pub options_text: OptionsText,
}

#[derive( Serialize, Deserialize, Debug, Clone )]
pub struct LoadingSceneText{
    pub loading: String,
}

#[derive( Serialize, Deserialize, Debug, Clone )]
pub struct MainMenuText{
    pub play: String,
    pub save: String,
    pub load: String,
    pub options: String,
    pub quit: String,
}

#[derive( Serialize, Deserialize, Debug, Clone )]
pub struct OptionsText{
    pub options: String,
    pub enable_music: String,
    pub enable_sound: String,
    pub language: String,
    pub return_back: String,
    pub on: String,
    pub off: String,
}

impl Glossary{
    pub fn new( language: Language ) -> Self{
        let file_name = match language{
            Language::EN => ENGLISH_LANGUAGE_FILE,
            Language::RU => RUSSIAN_LANGUAGE_FILE,
        };
        
        match File::open( file_name ){
            Ok( mut file ) => {
                let err_msg = format!( "{}: JSON file have bad format",
                    if language == Language::EN{
                        "Language::EN"
                    }
                    else{
                        "Language::RU"
                    }
                );

                let mut contents = String::new();
                file.read_to_string( &mut contents ).unwrap();
                let glossary = serde_json::from_str( &contents ).unwrap_or_else(|_| { panic!("{}", err_msg )});
                return glossary;
            }
            Err( err ) => panic!( "Can not open language file: {}, {}", err, file_name ),
        }
    }  
}