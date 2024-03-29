use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

use crate::config::*;
use crate::resources::language::Language;

#[derive(Serialize, Deserialize, Debug, Resource)]
pub struct Setting {
    enable_sound: bool,
    enable_music: bool,
    language: Language,
}

impl Setting{
    pub fn new( enable_sound: bool, enable_music: bool ) -> Self{
        Setting {
            enable_sound: enable_sound,
            enable_music: enable_music,
            language: Language::EN,
        }
    }
    
    pub fn get_enable_sound( &self ) -> bool{
        return self.enable_sound;
    }

    pub fn get_enable_music( &self ) -> bool{
        return self.enable_music;
    }

    pub fn get_language( &self ) -> Language{
        return self.language;
    }

    pub fn set_language( &mut self, language: Language ){
        self.language = language;
    }

    pub fn set_enable_music( &mut self, enable_music: bool ){
        self.enable_music = enable_music;
    }

    pub fn set_enable_sound( &mut self, enable_sound: bool ){
        self.enable_sound = enable_sound;
    }

    pub fn save_setting( &self ){
        let mut setting_file = File::create( SETTING_FILE ).expect( "Can not create setting file" );
        let setting_str: String = serde_json::to_string( &self ).unwrap();
        setting_file.write( setting_str.as_bytes() ).expect( "Can not to write file" );
    }

    pub fn load_setting( &mut self ){
        let setting:Setting = match File::open( SETTING_FILE ){
            Ok( mut file ) => {
                let mut contents = String::new();
                file.read_to_string( &mut contents ).unwrap();
                serde_json::from_str( &contents ).expect( "JSON was not well-formatted" )
            }
            Err( _err ) => {
                let mut setting_file = File::create( SETTING_FILE ).expect( "Can not create setting file" );
                let setting_str: String = serde_json::to_string( &Setting::new( true, true )).unwrap();
                setting_file.write( setting_str.as_bytes() ).expect( "Can not write file" );
                Setting::new( true, true )
            }
        };
        self.enable_music = setting.enable_music;
        self.enable_sound = setting.enable_sound;
        self.language = setting.language;
    }
}

impl FromWorld for Setting{
    fn from_world( _world: &mut World ) -> Self{
        let mut setting: Setting = Setting::new( true, true );
        // first start game will create "default" settings, when exit - settings will be created;
        setting.load_setting();
        return setting;
    }
}