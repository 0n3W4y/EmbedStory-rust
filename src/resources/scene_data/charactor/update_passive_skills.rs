use std::collections::HashMap;

use bevy::prelude::*;
use rand::Rng;

use super::effects::EffectType;
use super::{
    skills::CastSource,
    CharactorType, effects::Effect, CharactorStatus,
};
use crate::components::charactor_component::StatsComponent;
use crate::materials::material_manager::MaterialManager;
use crate::resources::scene_data::charactor::{self, skills::SkillDirectionType};
use crate::resources::scene_data::stuff::damage_type::DamageType;
use crate::{
    components::charactor_component::{
        CharactorComponent, CharactorTargetComponent, EffectComponent,
        PositionComponent, ResistsComponent, SkillComponent,
    },
    resources::deploy::Deploy,
    scenes::game_scenes::tilemap::tile::Position,
};

pub fn update_passive_skills(
    mut commands: Commands,
    mut skills_query: Query<(
        &CharactorComponent,
        &mut SkillComponent,
        &PositionComponent,
        &CharactorTargetComponent,
        &ResistsComponent,
        &StatsComponent,
        &mut EffectComponent,
    )>,
    mut charactors_query: Query<(
        &CharactorComponent,
        &PositionComponent,
        &ResistsComponent,
        &mut StatsComponent,
        &mut EffectComponent,
    )>,
    time: Res<Time>,
    deploy: Res<Deploy>,
    material_manager: Res<MaterialManager>,
) {
    let delta = time.delta_seconds();
    let mut rng = rand::thread_rng();
    for (
        charactor_component, 
        mut skill_component, 
        position_component, 
        target_component, 
        resists_component, 
        mut stats_component, 
        mut effect_component
    ) in skills_query.iter_mut() {

        //if char is dead we skip all passive skills;
        if charactor_component.status == CharactorStatus::Dead {
            continue;
        }

        for (skill_type, skill) in skill_component.passive_skills.iter_mut() {
            let trigger_time = skill.trigger_time; // time to trigger skill;
            let current_duration = skill.current_duration; // current tick time
            let total_duration = skill.total_duration; // time every tick
            let trigger_duration = skill.trigger_duration; // full time of skill 

            //first check for end of this skill;
            if total_duration <= trigger_duration {
                //skill is end and removed from skills storage;
                skill_component.passive_skills.remove(skill_type);
                continue;
            }

            //first run or trigger by time;
            if current_duration >= trigger_time || total_duration == 0.0 {
                //do skill
                if total_duration > 0.0 {
                    skill.current_duration -= trigger_time;
                }                
                
                //check for trigger chance
                let trigger_chance = skill.trigger_chanse;
                if trigger_chance < 100 {
                    let trigger_chance_random_number: u8 = rng.gen_range(0..=99);
                    if trigger_chance < trigger_chance_random_number {
                        //not triggered
                        continue;
                    }
                }

                //calculate crit chance and crit multiplier;
                let crit_chance = skill.crit_chance;
                let crit_chance_random_number = rng.gen_range(0..=99);
                let crit_multiplier = if crit_chance >= crit_chance_random_number {
                    skill.crit_multiplier
                } else {
                    0
                };

                let skill_target_type = &skill.target;
                let skill_cast_source = &skill.cast_source;

                //create cast position
                let cast_position = match *skill_cast_source {
                    CastSource::Itself => position_component.position.clone(),
                    CastSource::Mouse => panic!("Can not trigger passive skill: {:?}, because cast position is on Mouse!!! Only Active skills can casts from mouse", skill_type),
                    CastSource::Target => {
                        match target_component.target_position {
                            Some(v) => Position { x: v.x, y: v.y },
                            None => {
                                println!(
                                    "Can not trigger passive skill: '{:?}', because it casts from target and target position is empty! I use @Itself position",
                                    skill.skill_type
                                );
                                position_component.position.clone()
                            }
                        }
                    },
                };

                if skill.projectiles > 0 {
                    let projectiles = skill.projectiles;
                    // passive skills can casts only from Itself;
                    !
                    match skill.skill_direction {
                        SkillDirectionType::Point => {},
                        SkillDirectionType::Arc180 => {},
                        SkillDirectionType::Arc90 => {},
                        SkillDirectionType::Arc360 => {},
                        SkillDirectionType::Line => {},
                        SkillDirectionType::Arc45 => {},
                    }
                } else {
                    
                    //buff or debuff skill; if skill range == 0 then we understand skill can buff or debuff self when triggered. We must ignore target_type;
                    if skill.range == 0 {
                        do_damage(&skill.damage, &mut extra_stats_component, crit_multiplier, &resists_component.damage_resists);
                        // if skill have an effect, place effect;
                        add_effect(&skill.effect, &deploy, &resists_component.effect_resists, &mut effect_component);
                        
                    } else {
                        // AOE Aura

                        //for check target in range of skill
                        let x_min = cast_position.x - (skill.range as i32);
                        let x_max = cast_position.x + skill.range as i32;
                        let y_min = cast_position.y - (skill.range as i32);
                        let y_max = cast_position.y + skill.range as i32;
    
                        for (
                            target,
                            target_position,
                            target_resists,
                            mut target_stats,
                            mut target_effects,
                        ) in charactors_query.iter_mut()
                        {
                            //check for target
                            match skill_target_type {
                                CharactorType::Player => {
                                    if target.charactor_type != CharactorType::Player || target.charactor_type != CharactorType::Companion {
                                        continue;
                                    }
                                },
                                CharactorType::Monster => {
                                    if target.charactor_type != CharactorType::Monster {
                                        continue;
                                    }
                                },
                                _ => { 
                                    println!("Skill taget type should be Player or Enemy. Skill type: {:?}", skill_type);
                                    continue;
                                },
                            }
    
                            //ok if we r here, check the position of target;
                            let target_position_x = target_position.position.x;
                            let traget_position_y = target_position.position.y;
                            if target_position_x >= x_min &&
                                target_position_x <= x_max &&
                                traget_position_y >= y_min &&
                                traget_position_y <= y_max {

                                //bingo, we have a target;
                                do_damage(&skill.damage, &mut target_exra_stat, crit_multiplier, &target_resists.damage_resists);
                                add_effect(&skill.effect, &deploy, &target_resists.effect_resists, &mut target_effects);
                            } else {
                                //position of target not in range;
                                continue;
                            }
                        }
                    }
                }
            }
            //Add time to skill duration time; 0.0 -> +delta;
            skill.current_duration += delta;
            skill.total_duration += delta;
        }
    }
}

pub fn do_damage(damage: &HashMap<DamageType, i16>, target_extra_stat: &mut ExtraStatsComponent, crit_multiplier: i16, damage_resists: &HashMap<DamageType, i16>){
    for (damage_type, value) in damage.iter() {
        let self_resist: i16 = if *damage_type == DamageType::Health || *damage_type == DamageType::Stamina {
            0
        } else {
            match damage_resists.get(damage_type) {
                Some(v) => *v,
                None => {
                    println!("Update_passive_skills. Can not get self resist, self have no resist '{:?}' in storage.", damage_type);
                    0
                }
            }
        };

        let mut damage = value + (value * crit_multiplier / 100);
        damage -= damage * self_resist / 100;
        damage = if (*damage_type == DamageType::Health || *damage_type == DamageType::Stamina) && damage < 0 {
            damage
        } else {
            0
        };

        let extra_stat = if *damage_type == DamageType::Stamina {
            ExtraStat::StaminaPoints
        } else {
            ExtraStat::HealthPoints
        };

        //do damage on HealthPoints;
        charactor::change_extra_stat_current(
            &mut target_extra_stat.extra_stats,
            &mut target_extra_stat.extra_stats_cache,
            &extra_stat,
            damage,
            &StatDamageType::Flat,
        );
    }
}

pub fn add_effect(effect: &HashMap<EffectType, u8>, deploy: &Deploy, effect_resists: &HashMap<EffectType, i16>, effect_component: &mut EffectComponent){
    let mut rng = rand::thread_rng();
    for (effect_type, effect_trigger) in effect.iter() {
        let trigger_effect_random_number: u8 = rng.gen_range(0..=99);
        if *effect_trigger >= trigger_effect_random_number {
            //effect is triggered;
            let effect_config = deploy.charactor_deploy.effects_deploy.get_effect_config(effect_type);
            let mut effect = Effect::new(effect_config);

            let effect_resist = match effect_resists.get(&effect.effect_type) {
                Some(v) => *v,
                None => 0, // if not exist, use 0;
            };

            //check for resist this effect;
            if effect_resist >= 100 {
                //ignore that effect;
                continue;
            };

            if effect.duration == 0.0 {
                //try to insert, or ignore if effect already exist;
                effect_component.endless_effect.entry(effect_type.clone()).or_insert(effect);
            } else {
                //temporary effect;
                effect.duration -= effect.duration * effect_resist as f32 / 100.0;           
                effect_component.temporary_effect.entry(effect_type.clone()).and_modify(|x| x.duration += effect.duration).or_insert(effect);
            }
        }
    }
}
