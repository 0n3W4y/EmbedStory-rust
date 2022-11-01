use bevy::prelude::*;

#[derive( Debug, Clone )]
pub struct GroundTiles{
    pub earth: Handle<Image>,
    pub dry_earth: Handle<Image>,
    pub dirt: Handle<Image>,
    pub rock: Handle<Image>,
    pub rock_envirounment: Handle<Image>,
}