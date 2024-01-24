use std::collections::HashMap;

use bevy::prelude::*;
use rand::Rng;

use super::change_attribute_points;
use super::effects::EffectType;
use super::skills::{TargetType, PassiveSkill, PassiveSkillType};
use super::{
    CharactorType, CharactorStatus,
};
use crate::components::charactor_component::SkillAndEffectComponent;
use crate::components::{PositionComponent, IdentificationComponent, StatsComponent, TakenDamageComponent, TakenDamage};
use crate::components::projectile_component::Projectile;
use crate::materials::material_manager::MaterialManager;
use crate::resources::scene_data::charactor::skills::SkillDirectionType;
use crate::resources::scene_data::damage_text_informer::DamageTextInformer;
use crate::resources::scene_data::projectiles::ProjectileType;
use crate::scenes::game_scenes::tilemap::tile::Position;
use crate::{
    components::charactor_component::{
        CharactorComponent, CharactorTargetComponent},
    resources::deploy::Deploy
};
use crate::resources::scene_data::projectiles::update_projectile::create_projectile;

pub fn update_passive_skills(
    mut commands: Commands,
    mut skills_query: Query<(
        &IdentificationComponent,
        &CharactorComponent,
        &mut SkillAndEffectComponent,
        &PositionComponent,
        &CharactorTargetComponent,
        &mut TakenDamageComponent,
    ), With<CharactorComponent>>,
    mut charactors_query: Query<(
        &CharactorComponent,
        &PositionComponent,
        &mut TakenDamageComponent,
    ), With<CharactorComponent>>,
    deploy: Res<Deploy>,
    material_manager: Res<MaterialManager>,
) {
    let delta: f32 = 0.1;
    let mut random = rand::thread_rng();
    for (
        identification_component,
        charactor_component, 
        mut skill_and_effect_component, 
        position_component, 
        target_component, 
        mut taken_damage,
    ) in skills_query.iter_mut() {

        if charactor_component.status == CharactorStatus::Dead {                                                    //if char is dead we skip all passive skills;
            continue;
        }

        let mut skills_for_remove: Vec<PassiveSkillType> = vec![];                                                      //skills for remove;

        for (skill_type, skill) in skill_and_effect_component.passive_skills.iter_mut() {
            let trigger_frequency = skill.trigger_time_frequency;                                               // time to trigger skill;
            let current_time = skill.current_time_duration;                                                     // current tick time
            let total_duration = skill.total_duration;                                                          //total time every tick
            let life_time = skill.skill_life_time;                                                              // full life time of skill before remove

            if life_time <= total_duration {                                                                            //check for passive skill ends;
                skills_for_remove.push(skill_type.clone());                                                             //store skill sub type for next remove;
                continue;
            }

            if current_time >= trigger_frequency || total_duration == 0.0 {                                             //first run or trigger by time;
                if total_duration > 0.0 {                                                                                   //check for trigger time and substruct trigger time from current duration;
                    skill.current_time_duration -= current_time;
                }                
                
                let trigger_chance = skill.trigger_chance;                                                          //check for trigger chance
                if trigger_chance < 100 {
                    let trigger_chance_random_number: u8 = random.gen_range(0..=99);
                    if trigger_chance < trigger_chance_random_number {  
                        continue;                                                                                       //not triggered
                    }
                }

                if skill.skill_direction == SkillDirectionType::Itself{
                    do_direct_damage(skill, &mut taken_damage);                                                        //take damage to self;
                }

                find_targets(
                    &charactor_component.charactor_type, 
                    &target_component.target_position.unwrap(), 
                    &mut charactors_query, 
                    skill
                );
            }
        }
    }
}

fn find_targets(
    charactor_type: &CharactorType,
    position: &Position<i32>, 
    charactors: &mut Query<(
        &CharactorComponent,
        &PositionComponent,
        &mut TakenDamageComponent,
    ), With<CharactorComponent>>,
    skill: &mut PassiveSkill,
) {
    let skill_range = skill.skill_range;
    let skill_target = skill.target_type.clone();
    let skill_target_quantity = skill.target_quantity;
    let skill_projectile_type = skill.projectile_type.clone();

    let position_x = position.x;
    let position_y = position.y;
    let position_x_min = position_x - skill_range as i32;
    let position_x_max = position_x + skill_range as i32;
    let position_y_min = position_y - skill_range as i32;
    let position_y_max = position_y + skill_range as i32;
    for (charactor_component, position_component, mut taken_damage_component) in charactors.iter_mut() {
        if position_component.position.x == position_x && position_component.position.y == position_y {
            if check_for_condition(charactor_type, &charactor_component.charactor_type, &skill_target) {
                do_direct_damage(skill, &mut taken_damage_component);
            }
        }
    }

    /*
    setup_projectile(
        &mut commands, 
        &material_manager, 
        skill, 
        &position_component.position, 
        &target_component.target_position.unwrap()
    );
*/
}

fn check_for_condition(charactor_type: &CharactorType, target_type: &CharactorType, skill_target: &TargetType) -> bool {
    match *charactor_type {
        CharactorType::Player => {
            match *skill_target {
                TargetType::Allies => {
                    if *target_type == CharactorType::Player || *target_type == CharactorType::Companion {
                        true
                    } else {
                        false
                    }
                },
                TargetType::Enemies => {
                    if *target_type == CharactorType::Monster {
                        true
                    } else {
                        false
                    }
                },
                TargetType::All => true,
            }
        },
        CharactorType::NPC => {
            false
        },
        CharactorType::Monster => {
            match *skill_target {
                TargetType::Allies => {
                    if *target_type == CharactorType::Monster {
                        true
                    } else {
                        false
                    }
                },
                TargetType::Enemies => {
                    if *target_type == CharactorType::Player || *target_type == CharactorType::Companion {
                        true
                    } else {
                        false
                    }
                },
                TargetType::All => true,
            }
        },
        CharactorType::Companion => {
            match *skill_target {
                TargetType::Allies => {
                    if *target_type == CharactorType::Player || *target_type == CharactorType::Companion {
                        true
                    } else {
                        false
                    }
                },
                TargetType::Enemies => {
                    if *target_type == CharactorType::Monster {
                        true
                    } else {
                        false
                    }
                }
                TargetType::All => true
            }
        },
    }
}

fn do_direct_damage(skill: &mut PassiveSkill, taken_damage: &mut TakenDamageComponent){
    let mut random = rand::thread_rng();
    let mut damage: TakenDamage = Default::default();
    damage.area_of_impact = skill.area_on_impact;

    let crit_chance = skill.crit_chance;
    let crit_chance_random_number: i16 = random.gen_range(0..100);
    let crit_multiplier = if crit_chance >= crit_chance_random_number {
        skill.crit_multiplier
    } else {
        0
    };

    for (skill_damage, value) in skill.damage.iter() {
       let new_value = *value + *value * crit_multiplier / 100;
       damage.damage.insert(skill_damage.clone(), new_value);
       let damage_text = DamageTextInformer::new(new_value.to_string(), if crit_multiplier == 0 {false}else{true}, Some(skill_damage));
       taken_damage.text.push(damage_text);
    }

    for (effect_type, (effect, chance)) in skill.effect.iter() {
        let random_number_for_effect_trigger_chance: u8 = random.gen_range(0..100);
        if *chance > random_number_for_effect_trigger_chance {
            damage.effects.push(effect.clone());
        }
    }

    taken_damage.damage.push(damage);

}

fn setup_projectile( 
    commands: &mut Commands, 
    material_manager: &MaterialManager,
    skill: &PassiveSkill, 
    source_position: &Position<i32>,
    target_position: &Position<i32>,
){
    let mut random = rand::thread_rng();
    let mut projectile = Projectile::new(&skill.projectile_type);
    
    let crit_chance = skill.crit_chance;
    let crit_chance_random_number: i16 = random.gen_range(0..100);
    let crit_multiplier = if crit_chance >= crit_chance_random_number {
        skill.crit_multiplier
    } else {
        0
    };

    //create_projectile(commands, material_manager, projectile, target_position)
}
