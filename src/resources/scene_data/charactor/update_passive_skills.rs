use bevy::prelude::*;
use rand::Rng;

use super::skills::{TargetType, PassiveSkill, PassiveSkillType};
use super::{
    CharactorType, CharactorStatus,
};
use crate::components::charactor_component::SkillAndEffectComponent;
use crate::components::{PositionComponent, TakenDamageComponent, TakenDamage};
use crate::resources::scene_data::charactor::skills::SkillDirectionType;
use crate::resources::scene_data::damage_text_informer::DamageTextInformer;
use crate::resources::scene_data::projectiles::{setup_projectile_with_passive_skill, ProjectileType};
use crate::resources::scene_manager::SceneManager;
use crate::scenes::game_scenes::game_scene::GameScene;
use crate::scenes::game_scenes::tilemap::tile::Position;
use crate::{
    components::charactor_component::{
        CharactorComponent, CharactorTargetComponent},
    resources::deploy::Deploy
};

pub fn update_passive_skills(
    mut skills_query: Query<(
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
    mut scene_manager: ResMut<SceneManager>,
) {
    let mut random = rand::thread_rng();
    for (
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
                    &position_component.position,
                    &target_component.target_position, 
                    &mut charactors_query, 
                    skill,
                    scene_manager.get_current_game_scene_mut(),
                    &deploy,
                );
            }
        }

        for skill in skills_for_remove.iter() {
            skill_and_effect_component.passive_skills.remove(skill);
        }
    }
}

fn find_targets(
    charactor_type: &CharactorType,
    source_position: &Position<i32>, 
    target_position: &Option<Position<i32>>,
    charactors: &mut Query<(
        &CharactorComponent,
        &PositionComponent,
        &mut TakenDamageComponent,
    ), With<CharactorComponent>>,
    skill: &mut PassiveSkill,
    scene: &mut GameScene,
    deploy: &Deploy,
) {
    let skill_range = skill.skill_range;
    let skill_target = skill.target_type.clone();
    let mut skill_target_quantity = skill.target_quantity;
    let skill_projectile_type = skill.projectile_type.clone();

    let source_position_x = source_position.x;
    let source_position_y = source_position.y;

    let mut have_target_position:bool = false;

    let source_target_position = match target_position.as_ref() {
        Some(v) => {
            have_target_position = true;
            v.clone()
        },
        None => {
            Position{x: 0, y: 0}
        },
    };

    let target_position_x_min = source_position_x - skill_range as i32;
    let target_position_x_max = source_position_x + skill_range as i32;
    let target_position_y_min = source_position_y - skill_range as i32;
    let target_position_y_max = source_position_y + skill_range as i32;

    for (
        target_charactor_component, 
        target_position, 
        mut target_taken_damage
    ) in charactors.iter_mut() {
        let target_position_x = target_position.position.x;
        let target_position_y = target_position.position.y;
        if target_position_x == source_position_x && target_position_y == source_position_y {
            continue;                                                                                                                           //ignore self (passive skill caster);
        }
        
        if !check_for_condition(charactor_type, &target_charactor_component.charactor_type, &skill_target) {
            continue;
        }

        if have_target_position {
            if target_position_x == source_target_position.x && target_position_x == source_target_position.y {
                if skill_projectile_type == ProjectileType::None {
                    do_direct_damage(skill, &mut target_taken_damage);
                } else {
                    setup_projectile_with_passive_skill(scene, skill, source_position, &target_position.position, deploy);
                }

                have_target_position = false;                                                                                                   //This flag for next iteration, if targets quantity > 1
                if skill_target_quantity == 0 {
                    return;
                }
            }
        } 

        if skill_target_quantity > 0 {
            if (target_position_x >= target_position_x_min && target_position_x <= target_position_x_max) 
            && (target_position_y >= target_position_y_min && target_position_y <= target_position_y_max) {
                if skill_projectile_type == ProjectileType::None {
                    do_direct_damage(skill, &mut target_taken_damage);
                } else {
                    setup_projectile_with_passive_skill(scene, skill, source_position, &target_position.position, deploy);
                }
                skill_target_quantity -= 1;
            }
        } else {
            if !have_target_position {
                return;
            }
        }
        
    }
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

    for (_, (effect, chance)) in skill.effects.iter() {
        let random_number_for_effect_trigger_chance: u8 = random.gen_range(0..100);
        if *chance > random_number_for_effect_trigger_chance {
            damage.effects.push(effect.clone());
        }
    }

    taken_damage.damage.push(damage);

}