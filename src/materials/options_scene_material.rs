use bevy::prelude::*;

#[derive( Debug, Clone )]
pub struct OptionsSceneMaterial{
    pub sound_off: Handle<Image>,
    pub sound_on: Handle<Image>,
    pub sound_hovered: Handle<Image>,
    pub music_on: Handle<Image>,
    pub music_off: Handle<Image>,
    pub music_hovered: Handle<Image>,
    pub language_ru: Handle<Image>,
    pub language_en: Handle<Image>,
}