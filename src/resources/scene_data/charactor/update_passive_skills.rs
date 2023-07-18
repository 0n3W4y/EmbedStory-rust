use bevy::prelude::*;
use rand::Rng;

use super::{
    skills::{CastSource, SkillTargetType},
    stats::ExtraStat,
    CharactorType, effects::Effect,
};
use crate::resources::scene_data::charactor;
use crate::{
    components::charactor_component::{
        CharactorComponent, CharactorTargetComponent, EffectComponent, ExtraStatsComponent,
        PositionComponent, ResistsComponent, SkillComponent,
    },
    resources::deploy::Deploy,
    scenes::game_scenes::tilemap::tile::Position,
};

pub fn update_passive_skills(
    mut skills_query: Query<(
        &CharactorComponent,
        &mut SkillComponent,
        &PositionComponent,
        &CharactorTargetComponent,
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
    for (charactor_component, mut skill_component, position_component, target_component) in
        skills_query.iter_mut()
    {
        for (skill_type, skill) in skill_component.passive_skills.iter_mut() {
            let trigger_time = skill.trigger_time;
            let trigger_chance = skill.trigger_chanse;
            let current_duration = skill.current_duration;
            //check for trigger time;
            if current_duration < trigger_time {
                //add time;
                skill.current_duration += delta;
                continue;
            } else {
                //update time;
                skill.current_duration -= skill.trigger_time;
            }

            //check for trigger chance
            if trigger_chance < 100 {
                let trigger_chance_random_number: u8 = rng.gen_range(0..=99);
                if trigger_chance < trigger_chance_random_number {
                    //not triggered
                    continue;
                }
            }

            if skill.projectiles > 0 {
                todo!();
                //todo: Create projectile, add damage, add effect, add crit chance and multiplier damage;
            } else {
                //AOE skill
                if skill.range == 0 {
                    //self buff or debuff skill
                    todo!();
                } else {
                    // AOE Aura
                    let crit_chance = skill.current_crit_chance;
                    let crit_chance_random_number = rng.gen_range(0..=99);
                    let crit_chance_multuplier = skill.current_crit_multiplier;
                    let skill_target_type = &skill.target;
                    let skill_cast_source = &skill.cast_source;

                    //create cast position
                    let cast_position = if skill.cast_source == CastSource::Target {
                        match target_component.target_position {
                            Some(v) => Position { x: v.x, y: v.y },
                            None => {
                                println!(
                                    "Can not trigger passive skill: '{:?}', because it casts from target and target position is empty! I use @Itself position",
                                    skill.skill_type
                                );
                                Position {
                                    x: position_component.position.x,
                                    y: position_component.position.y,
                                }
                            }
                        }
                    } else {
                        Position {
                            x: position_component.position.x,
                            y: position_component.position.y,
                        }
                    };

                    //for check target in range of skill
                    let x_min = cast_position.x - (skill.range as i32);
                    let x_max = cast_position.x + skill.range as i32;
                    let y_min = cast_position.y - (skill.range as i32);
                    let y_max = cast_position.y + skill.range as i32;

                    let damage_multiplier: u8 = if crit_chance >= crit_chance_random_number {
                        1
                    } else {
                        0
                    };

                    for (
                        target,
                        target_position,
                        target_resists,
                        mut target_exra_stat,
                        mut target_effects,
                    ) in charactors_query.iter_mut()
                    {
                        //check for target
                        match skill.target {
                            SkillTargetType::Enemy => {
                                if target.charactor_type == charactor_component.charactor_type {
                                    continue;
                                };
                            }
                            SkillTargetType::Ally => match target.charactor_type {
                                CharactorType::Player => {
                                    if charactor_component.charactor_type
                                        != CharactorType::Companion
                                    {
                                        continue;
                                    }
                                }
                                CharactorType::NPC => {
                                    panic!("NPC!!!!!!!!!!!!!!!!!!! WTF? Update_passive_skills");
                                }
                                CharactorType::Monster => {
                                    if charactor_component.charactor_type != CharactorType::Monster
                                    {
                                        continue;
                                    }
                                }
                                CharactorType::Companion => {
                                    if charactor_component.charactor_type != CharactorType::Player {
                                        continue;
                                    }
                                }
                            },
                            _ => {}
                        }

                        //ok if we r here, check the position of target;
                        if target_position.position.x >= x_min
                            && target_position.position.x <= x_max
                            && target_position.position.y >= y_min
                            && target_position.position.y <= y_max
                        {
                            //bingo, we have a target;
                            for (damage_type, value) in skill.current_damage.iter() {
                                let target_resist = match target_resists
                                    .damage_resists
                                    .get(damage_type)
                                {
                                    Some(v) => *v,
                                    None => {
                                        println!("Can not get target resist, target have no resist '{:?}' in storage.", damage_type);
                                        0
                                    }
                                };

                                let damage = if damage_multiplier == 0 {
                                    value - (value * target_resist / 100)
                                } else {
                                    let temp_value = value
                                        + ((value * crit_chance_multuplier / 100)
                                            * crit_chance_multuplier);
                                    temp_value - (temp_value * target_resist / 100)
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
                                                    "No damage for effect: '{:?}' in passie skill : '{:?}', so i use 0",
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

            //check for effects on self
        }
    }
}
