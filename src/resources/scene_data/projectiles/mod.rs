use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use rand::Rng;

use crate::{components::projectile_component::Projectile, resources::deploy::Deploy, scenes::game_scenes::{game_scene::GameScene, tilemap::tile::Position}};

use super::{charactor::{effects::{Effect, EffectType}, skills::{ActiveSkill, PassiveSkill, PassiveSkillType}}, Damage};

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

pub fn setup_projectile_with_active_skill(
    scene: &mut GameScene,
    skill: &ActiveSkill, 
    source_position: &Position<i32>,
    target_position: &Position<i32>,
    deploy: &Deploy,
    is_missed: bool,
) {
    let projectile = setup_projectile(
        deploy,
        skill.crit_chance,
        skill.crit_multiplier,
        skill.skill_range,
        &skill.projectile_type,
        &skill.damage,
        &skill.effects,
        Some(&skill.passive_skills),
        source_position,
        target_position,
        is_missed,
    );
    scene.projectiles.push(projectile);
}


pub fn setup_projectile_with_passive_skill(
    scene: &mut GameScene,
    skill: &PassiveSkill, 
    source_position: &Position<i32>,
    target_position: &Position<i32>,
    deploy: &Deploy,
) {
    let projectile = setup_projectile(
        deploy,
        skill.crit_chance,
        skill.crit_multiplier,
        skill.skill_range,
        &skill.projectile_type,
        &skill.damage,
        &skill.effects,
        None,
        source_position,
        target_position,
        false,
    );    
    scene.projectiles.push(projectile);
}

fn setup_projectile(
    deploy: &Deploy,
    skill_crit_chance: i16,
    skill_crit_multiplier: i16,
    skill_range: u8,
    skill_projectile_type: &ProjectileType,
    skill_damage: &HashMap<Damage, i16>,
    skill_effects: &HashMap<EffectType, (Effect, u8)>,
    skill_passive_skills: Option<&HashMap<PassiveSkillType, (PassiveSkill, u8)>>,
    source_position: &Position<i32>,
    target_position: &Position<i32>,
    is_missed: bool,
) -> Projectile {
    let mut random = rand::thread_rng();

    let projectile_config = deploy.projectile_deploy.get_config(&skill_projectile_type);
    let mut projectile = Projectile::new(projectile_config);
    projectile.range = skill_range;

    projectile.starting_position = source_position.clone();

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

    if is_missed {
        projectile.is_missed = true;
        return projectile;
    }

    let crit_chance_random_number: i16 = random.gen_range(0..100);
    let crit_multiplier = if skill_crit_chance >= crit_chance_random_number {
        skill_crit_multiplier
    } else {
        100
    };

    projectile.is_critical_hit = if crit_multiplier > 100 {
        true
    } else {
        false
    };

    for (damage, damage_value) in skill_damage.iter() {
        let damage_value_with_critical_multiplier = *damage_value * crit_multiplier / 100;
        projectile.damage.insert(damage.clone(), damage_value_with_critical_multiplier);
    }

    for (_, (effect, trigger_chance)) in skill_effects.iter() {
        let trigger_chance_random_number: u8 = random.gen_range(0..100);
        if *trigger_chance > trigger_chance_random_number || *trigger_chance >= 100 {
            projectile.effects.push(effect.clone());
        }
    }

    match skill_passive_skills {
        Some(v) => {
            for (_, (passive_skill, trigger_chance)) in v.iter() {
                let trigger_chance_random_number: u8 = random.gen_range(0..100);
                if *trigger_chance < trigger_chance_random_number {
                    continue;                                                                                                                                   //not triggered;
                }
        
                projectile.passive_skills.push(passive_skill.clone());
            }
        },
        None => {},
    }
    projectile
}