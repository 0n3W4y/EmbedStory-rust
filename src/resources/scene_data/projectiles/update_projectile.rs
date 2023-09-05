use bevy::prelude::*;

use crate::{components::projectile_component::Projectile, resources::deploy::Deploy};

pub fn update_projectiles(projectile_query: Query<&Projectile>) {

}

pub fn create_projectile(commands: Commands, projectile: Projectile, deploy: &Deploy) {
    
}