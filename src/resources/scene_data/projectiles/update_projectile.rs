use bevy::prelude::*;

use crate::{components::projectile_component::Projectile, materials::material_manager::MaterialManager};

pub fn update_projectiles(projectile_query: Query<&Projectile>) {

}

pub fn create_projectile(commands: &mut Commands, projectile: Projectile, material_manager: &MaterialManager) {
    
}