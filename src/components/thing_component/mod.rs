use bevy::prelude::*;

use crate::resources::scene_data::thing::ThingDefenseType;
use crate::resources::scene_data::thing::ThingType;
use crate::resources::scene_data::thing::ThingPermissions;

#[derive(Component, Default)]
pub struct ThingComponent{
    pub thing_type: ThingType,
    pub graphic_index: u8,
    pub thing_defense_type: ThingDefenseType,
}

#[derive(Component, Default)]
pub struct ThingPermissionsComponent {
    pub permissions: Vec<ThingPermissions>,
}