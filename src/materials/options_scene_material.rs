use bevy::prelude::*;

#[derive( Debug, Clone )]
pub struct OptionsSceneMaterial{
    pub background_image: Handle<Image>,
    pub language_ru: Handle<Image>,
    pub language_en: Handle<Image>,
}