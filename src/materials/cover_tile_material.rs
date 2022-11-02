use bevy::prelude::*;

#[derive( Debug, Clone )]
pub struct CoverTileMaterial{
    pub grass: Handle<Image>,
    pub sand: Handle<Image>,
    pub snow: Handle<Image>,
    pub shallow: Handle<Image>,
    pub water: Handle<Image>,
    pub ice: Handle<Image>,
    pub wooden_floor: Handle<Image>,
}