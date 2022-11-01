use bevy::prelude::*;

use crate::resources::language::Language;

pub struct FontMaterials {
    pub firasans_bold_font: Handle<Font>,
}

impl FontMaterials{
    pub fn get_font( &self, language: Language ) -> Handle<Font>{
        let font = match language{
            Language::EN => self.firasans_bold_font.clone(),
            Language::RU => self.firasans_bold_font.clone(),
        };
        return font;
    }
}