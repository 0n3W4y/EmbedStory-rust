use bevy::prelude::*;

use crate::{components::projectile_component::Projectile, materials::material_manager::MaterialManager, scenes::game_scenes::tilemap::tile::Position, resources::scene_data::charactor::skills::SkillDirectionType};

pub fn update_projectiles(mut projectile_query: Query<(&Projectile, &mut Transform)>, time: Res<Time>) {
    let delta = time.delta_seconds();
    for(projectile, mut transfrom) in projectile_query.iter_mut() {
        let new_x = projectile.motion_coefficient.x as f32 * projectile.velocity as f32 * delta + projectile.starting_position.x as f32;
        let new_y = projectile.motion_coefficient.y as f32 * projectile.velocity as f32 * delta + projectile.starting_position.y as f32;
        transfrom.translation.x += new_x;
        transfrom.translation.y += new_y;
        //calculate grid postion by translation;
        //check for collision;
    }
}

pub fn check_for_collision(){}

pub fn create_projectile(
    commands: &mut Commands,
    material_manager: &MaterialManager,
    mut projectile: Projectile,      
    target_position: Position<i32>,
    projectiles_value: u8, 
    skill_direction: &SkillDirectionType
) {
    let arc: f32 = match *skill_direction {
        SkillDirectionType::Line => 0.0,
        SkillDirectionType::Arc15 => 15.0,
        SkillDirectionType::Arc30 => 30.0,
        SkillDirectionType::Arc45 => 45.0,
        SkillDirectionType::Arc60 => 60.0,
        SkillDirectionType::Arc90 => 90.0,
        SkillDirectionType::Arc180 => 180.0,
        SkillDirectionType::Arc360 => 360.0,
        SkillDirectionType::Point => {
            println!("Can not create projectiles where skill direction type is 'POINT'");
            return;
        },
    }; 
    
    let half_arc_angle = arc / 2.0;
    let angle_coefficient = if projectiles_value == 1 {                      //each angle to cast projectile;
        0.0
    } else {
        arc / projectiles_value as f32
    };                                     
    let delta_x = projectile.starting_position.x as f32 - target_position.x as f32;                //difference between target position and starting position;
    let delta_y = projectile.starting_position.y as f32 - target_position.y as f32;                
    let angle_between_ab_and_y = (delta_x.atan2(delta_y)).to_degrees();                                  //angle between Y and line cast to target position;

    projectile.motion_coefficient.x = target_position.x - projectile.starting_position.x;
    projectile.motion_coefficient.y = target_position.y - projectile.starting_position.y;
    //commands.. bla bla bla
    /*                                          //this is how to rotate projectile sprite;
    
   

    let arc = 90.0;
    let projectiles = 2;
    // if projectiles < 2 use line;
    let left_and_right_arc: f32 = arc / 2.0;
    let delta_x: i32 = b_x - a_x;
    let delta_y: i32 = b_y - a_y;
    let radius = ((delta_x*delta_x + delta_y*delta_y) as f32).sqrt();
    println!("radius:{}", radius);
    let x = a_x as f32 + radius * (angle_between_ab_and_y + left_and_right_arc).to_radians().sin();
    let y = a_y as f32 + radius * (angle_between_ab_and_y + left_and_right_arc).to_radians().cos();
    let x1 = a_x as f32 + radius * (angle_between_ab_and_y - left_and_right_arc).to_radians().sin();
    let y1 = a_y as f32 + radius * (angle_between_ab_and_y - left_and_right_arc).to_radians().cos();
    println!("x:{x}; y:{y}");
    println!("x1: {x1}; y1: {y1}");
     */

    /*
    параметрическое x = b.x - a.x *t + a.x;
    y = b.y - a.y * t + a.y;
     */
}