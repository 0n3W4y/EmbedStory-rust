use bevy::prelude::*;

use crate::{components::projectile_component::Projectile, materials::material_manager::MaterialManager, scenes::game_scenes::tilemap::tile::Position};

pub fn update_projectiles(mut projectile_query: Query<(&Projectile, &mut Transform)>, time: Res<Time>) {
    let delta = time.delta_seconds();
    for(projectile, mut transfrom) in projectile_query.iter_mut() {
        let new_x = projectile.motion_coefficient.x as f32 * projectile.velocity as f32 * delta + projectile.starting_position.x as f32;
        let new_y = projectile.motion_coefficient.y as f32 * projectile.velocity as f32 * delta + projectile.starting_position.y as f32;
        transfrom.translation.x += new_x;
        transfrom.translation.y += new_y;
        //check for collision;
    }
}

pub fn check_for_collision(){}

pub fn create_projectile(commands: &mut Commands, mut projectile: Projectile, material_manager: &MaterialManager, end_point_position: Position<i32>) {
    projectile.motion_coefficient.x = end_point_position.x - projectile.starting_position.x;
    projectile.motion_coefficient.y = end_point_position.y - projectile.starting_position.y;
    //commands.. bla bla bla
    /*                                          //this is how to rotate projectile sprite;
    let a = Position{ x: 2, y: -3};
    let b = Position{x : -3, y: 4};
    let delta_x = b.x as f32 - a.x as f32;
    let delta_y = b.y as f32 - a.y as f32;
    let tan = delta_x.atan2(delta_y);
    println!("{tan}");
    let angle = tan.to_degrees();
    println!("{angle}");
     */
}