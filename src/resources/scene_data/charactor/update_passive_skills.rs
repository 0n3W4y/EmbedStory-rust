use bevy::prelude::*;
use rand::Rng;

use super::{
    skills::CastSource,
    stats::ExtraStat,
    CharactorType, effects::Effect, CharactorStatus,
};
use crate::resources::scene_data::charactor::{self, skills::SkillDirectionType};
use crate::{
    components::charactor_component::{
        CharactorComponent, CharactorTargetComponent, EffectComponent, ExtraStatsComponent,
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
        &mut ExtraStatsComponent,
        &mut EffectComponent,
    )>,
    mut charactors_query: Query<(
        &CharactorComponent,
        &PositionComponent,
        &ResistsComponent,
        &mut ExtraStatsComponent,
        &mut EffectComponent,
    )>,
    time: Res<Time>,
    deploy: Res<Deploy>,
) {
    let delta = time.delta_seconds();
    let mut rng = rand::thread_rng();
    for (charactor_component, mut skill_component, position_component, target_component, resists_component, mut extra_stats_component, mut effect_component) in
        skills_query.iter_mut()
    {
        //if char is dead we skip all passive skills;
        if charactor_component.status == CharactorStatus::Dead {
            continue;
        }

        for (skill_type, skill) in skill_component.passive_skills.iter_mut() {
            let trigger_time = skill.trigger_time;
            let current_duration = skill.current_duration;
            let total_duration = skill.total_duration;
            let trigger_duration = skill.trigger_duration;

            //first check for end of this skill;
            if total_duration <= trigger_duration {
                //skill is end and removed from skills storage;
                skill_component.passive_skills.remove(skill_type);
                //todo: remove endless effect;
                todo!();
                continue;
            }

            if current_duration >= trigger_time || total_duration == 0.0 {
                //do skill
                if total_duration > 0.0 {
                    skill.current_duration -= trigger_time;
                }                
                let trigger_chance = skill.trigger_chanse;

                //check for trigger chance
                if trigger_chance < 100 {
                    let trigger_chance_random_number: u8 = rng.gen_range(0..=99);
                    if trigger_chance < trigger_chance_random_number {
                        //not triggered
                        continue;
                    }
                }

                //calculate crit chance and crit multiplier;
                let crit_chance = skill.current_crit_chance;
                let crit_chance_random_number = rng.gen_range(0..=99);
                let crit_multiplier = skill.current_crit_multiplier;
                let crit_chance_triggered: bool = if crit_chance >= crit_chance_random_number {
                    true
                } else {
                    false
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
    
                    match skill.skill_direction {
                        SkillDirectionType::Point => {},
                        SkillDirectionType::Arc180 => {},
                        SkillDirectionType::Arc90 => {},
                        SkillDirectionType::Arc360 => {},
                        SkillDirectionType::Line => {},
                        SkillDirectionType::Arc45 => {},
                    }
                    
                    //todo: Create projectile, add damage, add effect, add crit chance and multiplier damage;
                } else {
                    
                    //buff or debuff skill
                    if skill.range == 0 {

                        // if skill have a damage to health to self
                        for (damage_type, value) in skill.current_damage.iter() {
                            let self_resist = match resists_component.damage_resists.get(damage_type) {
                                Some(v) => *v,
                                None => {
                                    println!("Update_passive_skills. Can not get self resist, self have no resist '{:?}' in storage.", damage_type);
                                    0
                                }
                            };

                            let damage = if crit_chance_triggered {
                                (value * crit_multiplier / 100) - ((value * crit_multiplier / 100) * self_resist / 100)
                            } else {
                                value - (value * self_resist / 100)
                            };

                            //do damage on HealthPoints;
                            charactor::change_extra_stat_current(
                                &mut extra_stats_component.extra_stats,
                                &mut extra_stats_component.extra_stats_cache,
                                &ExtraStat::HealthPoints,
                                damage,
                            );
                        }

                        // if skill have an effect, place effect;
                        for (effect_type, effect_trigger) in skill.effect.iter() {
                            let trigger_effect_random_number: u8 = rng.gen_range(0..=99);
                            if *effect_trigger >= trigger_effect_random_number {
                                //effect is triggered;
                                let effect_config = deploy.charactor_deploy.effects_deploy.get_effect_config(effect_type);
                                let mut effect = Effect::new(effect_config);

                                let self_effect_resist = match resists_component.effect_resists.get(&effect.effect_type) {
                                    Some(v) => *v,
                                    None => 0, // if not exist, use 0;
                                };

                                //check for resist this effect;
                                if self_effect_resist >= 100 {
                                    //ignore that effect;
                                    continue;
                                };

                                //check for effect provide damage?
                                if effect.change_extra_stat_is_damage {
                                    //get damage from weapon;
                                    let damage = match skill.current_damage.get(&effect.damage_type) {
                                        Some(v) => *v,
                                        None => {
                                            println!(
                                                "No damage for effect: '{:?}' in passiмe skill : '{:?}', so i use 0",
                                                effect.effect_type, 
                                                skill.skill_type
                                            );
                                            0
                                        }
                                    };
                                    //set new damage value to effect;
                                    for (_, value) in effect.change_extra_stat.iter_mut() {
                                        *value = damage ;
                                    };
                                };

                                if effect.duration == 0.0 {
                                    //try to insert, or ignore if effect already exist;
                                    //Maybe change damage todo();
                                    effect_component.endless_effect.entry(effect_type.clone()).or_insert(effect);
                                } else {
                                    //temporary effect;
                                    effect.duration -= effect.duration * self_effect_resist as f32 / 100.0;                                        

                                    let old_effect = effect_component.temporary_effect.entry(effect_type.clone()).and_modify(|x| x.duration += effect.duration).or_insert(effect);
                
                                    //select > value of old effect and new effect;
                                    for (key, value) in old_effect.change_extra_stat.iter_mut() {
                                        let effect_value = match effect.change_extra_stat.get(key) {
                                            Some(v) => *v,
                                            None => 0,
                                        };
                                        *value = (*value).max(effect_value);
                                    }
                                }
                            }
                        }
                        
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
                            mut target_exra_stat,
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
                            if target_position_x >= x_min
                                && target_position_x <= x_max
                                && traget_position_y >= y_min
                                && traget_position_y <= y_max
                            {
                                //bingo, we have a target;
                                for (damage_type, value) in skill.current_damage.iter() {
                                    let target_resist = match target_resists
                                        .damage_resists
                                        .get(damage_type)
                                    {
                                        Some(v) => *v,
                                        None => {
                                            println!("Update_passive_skills. Can not get target resist, target have no resist '{:?}' in storage.", damage_type);
                                            0
                                        }
                                    };
    
                                    let damage = if crit_chance_triggered {
                                        (value * crit_multiplier / 100) - ((value * crit_multiplier / 100) * target_resist / 100)
                                    } else {
                                        value - (value * target_resist / 100)
                                    };
    
                                    //do damage on HealthPoints;
                                    charactor::change_extra_stat_current(
                                        &mut target_exra_stat.extra_stats,
                                        &mut target_exra_stat.extra_stats_cache,
                                        &ExtraStat::HealthPoints,
                                        damage,
                                    );
                                }
    
                                for (effect_type, effect_trigger) in skill.effect.iter() {
                                    let trigger_effect_random_number: u8 = rng.gen_range(0..=99);
                                    if *effect_trigger >= trigger_effect_random_number {
                                        //effect is triggered;
                                        let effect_config = deploy.charactor_deploy.effects_deploy.get_effect_config(effect_type);
                                        let mut effect = Effect::new(effect_config);
    
                                        let target_effect_resist = match target_resists.effect_resists.get(&effect.effect_type) {
                                            Some(v) => *v,
                                            None => 0, // if not exist, use 0;
                                        };
    
                                        //check for resist this effect;
                                        if target_effect_resist >= 100 {
                                            //ignore that effect;
                                            continue;
                                        };
    
                                        //check for effect provide damage?
                                        if effect.change_extra_stat_is_damage {
                                            //get damage from weapon;
                                            let damage = match skill.current_damage.get(&effect.damage_type) {
                                                Some(v) => *v,
                                                None => {
                                                    println!(
                                                        "No damage for effect: '{:?}' in passiмe skill : '{:?}', so i use 0",
                                                        effect.effect_type, 
                                                        skill.skill_type
                                                    );
                                                    0
                                                }
                                            };
                                            //set new damage value to effect;
                                            for (_, value) in effect.change_extra_stat.iter_mut() {
                                                *value = damage ;
                                            };
                                        };
    
                                        if effect.duration == 0.0 {
                                            //try to insert, or ignore if effect already exist;
                                            //Maybe change damage todo();
                                            target_effects.endless_effect.entry(effect_type.clone()).or_insert(effect);
                                        } else {
                                            //temporary effect;
                                            effect.duration -= effect.duration * target_effect_resist as f32 / 100.0;                                        
    
                                            let old_effect = target_effects.temporary_effect.entry(effect_type.clone()).and_modify(|x| x.duration += effect.duration).or_insert(effect);
                        
                                            //select > value of old effect and new effect;
                                            for (key, value) in old_effect.change_extra_stat.iter_mut() {
                                                let effect_value = match effect.change_extra_stat.get(key) {
                                                    Some(v) => *v,
                                                    None => 0,
                                                };
                                                *value = (*value).max(effect_value);
                                            }
                                        }
                                    }
                                }
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
