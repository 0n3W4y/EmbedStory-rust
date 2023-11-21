use bevy::prelude::*;

pub mod thing_animation_component;

use crate::resources::scene_data::thing::ThingType;
use crate::resources::scene_data::thing::ThingPermissions;

#[derive(Component, Default)]
pub struct ThingComponent{
    pub thing_type: ThingType,
    pub graphic_index: u8,
}

#[derive(Component, Default)]
pub struct ThingPermissionsComponent {
    pub permissions: Vec<ThingPermissions>,
}