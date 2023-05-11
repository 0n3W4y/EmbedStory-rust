use bevy::prelude::*;

use crate::components::charactor_component::CharactorComponent;
use crate::resources::profile::Profile;
use crate::materials::material_manager::MaterialManager;

pub fn spawn(

){

}

pub fn spawn_player(
    mut commands: Commands,
    profile: Res<Profile>,
    material_manager: Res<MaterialManager>,
){
    let player_component: CharactorComponent = Default::default();
}