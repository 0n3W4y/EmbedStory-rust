use bevy::prelude::*;
//use bevy_inspector_egui::Inspectable;

#[derive(Component)]
pub struct TileComponent{
    pub index: usize // vec index in tilemapstorage;
}