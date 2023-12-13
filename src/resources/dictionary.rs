use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::resources::language::Language;
use crate::resources::setting::Setting;
use crate::resources::glossary::Glossary;

#[derive( Serialize, Deserialize, Debug, Clone, Resource )]
pub struct Dictionary{
    en_glossary:Glossary,
    ru_glossary:Glossary,
    current_language:Language,
}

impl Dictionary{
    pub fn new( language:Language ) -> Self{
        return Dictionary{
            en_glossary: Glossary::new( Language::EN ),
            ru_glossary: Glossary::new( Language::RU ),
            current_language: language,
        }
    }

    pub fn get_glossary( &self ) -> Glossary{
        match self.current_language{
            Language::EN => return self.en_glossary.clone(),
            Language::RU => return self.ru_glossary.clone(),
        }
    }

    pub fn get_current_language( &self ) -> Language{
        return self.current_language;
    }

    pub fn set_current_language( &mut self, language:Language ){
        self.current_language = language;
    }
}

impl FromWorld for Dictionary{
    fn from_world( world:&mut World )-> Self{
        let settings = world.get_resource_mut::<Setting>().unwrap();
        return Dictionary::new( settings.get_language() );
    }
}