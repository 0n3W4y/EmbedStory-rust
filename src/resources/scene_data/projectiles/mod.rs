use serde::{Deserialize, Serialize};
use rand::Rng;

use crate::{components::projectile_component::Projectile, resources::deploy::Deploy, scenes::game_scenes::{game_scene::GameScene, tilemap::tile::Position}};

use super::charactor::skills::PassiveSkill;

pub mod update_projectile;

#[derive(Default, Debug, Deserialize, PartialEq, Eq, Clone, Serialize)]
pub enum ProjectileType {
    #[default]
    Arrow,
    Bullet,
    FireSphere,
    None,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProjectileConfig{
    pub projectile_type: ProjectileType,
    pub can_pierce: bool,
    pub pierce_chance: u8,
    pub velocity: u16,
}


pub fn setup_projectile(
    scene: &mut GameScene,
    skill: &PassiveSkill, 
    source_position: &Position<i32>,
    target_position: &Position<i32>,
    deploy: &Deploy,
) {
    let mut random = rand::thread_rng();
    let mut projectile = Projectile::new(&skill.projectile_type);
    projectile.range = skill.skill_range;
    
    let starting_point_x = projectile.starting_position.x;
    let starting_point_y = projectile.starting_position.y;

    let delta_x = target_position.x - starting_point_x;
    let delta_y = target_position.y - starting_point_y;

    projectile.direction.x = if delta_x < 0 {
        -1
    } else if delta_x > 0 {
        1
    } else {
        0
    };

    projectile.direction.y = if delta_y < 0 {
        -1
    } else if delta_y > 0 {
        1
    } else {
        0
    };

    let distance = ((delta_x as f32).powf(2.0) + (delta_y as f32).powf(2.0)).sqrt();
    projectile.motion_coefficient.x = delta_x as f32 / distance;
    projectile.motion_coefficient.y = delta_y as f32 / distance;

    let crit_chance = skill.crit_chance;
    let crit_chance_random_number: i16 = random.gen_range(0..100);
    let crit_multiplier = if crit_chance >= crit_chance_random_number {
        skill.crit_multiplier
    } else {
        0
    };

    
    scene.projectiles.push( );
}