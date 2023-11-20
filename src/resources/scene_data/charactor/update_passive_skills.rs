use std::collections::HashMap;

use bevy::prelude::*;
use rand::Rng;

use super::effects::EffectType;
use super::skills::{TargetType, SkillType, Skill};
use super::stats::Stat;
use super::{
    skills::CastSource,
    CharactorType, effects::Effect, CharactorStatus,
};
use crate::components::{PositionComponent, IdenteficationComponent};
use crate::components::charactor_component::{StatsComponent, DestinationComponent};
use crate::components::projectile_component::Projectile;
use crate::materials::material_manager::MaterialManager;
use crate::resources::scene_data::charactor::skills::SkillDirectionType;
use crate::resources::scene_data::stuff::damage_type::DamageType;
use crate::resources::scene_data::stuff::resists_types::{self, get_resist_from_damage_type, ResistType};
use crate::scenes::game_scenes::tilemap::tile::Position;
use crate::{
    components::charactor_component::{
        CharactorComponent, CharactorTargetComponent, EffectComponent,
        ResistsComponent, SkillComponent,
    },
    resources::deploy::Deploy
};
use crate::resources::scene_data::projectiles::update_projectile::create_projectile;
use crate::resources::scene_data::charactor;

pub fn update_passive_skills(
    mut commands: Commands,
    mut skills_query: Query<(
        &IdenteficationComponent,
        &CharactorComponent,
        &mut SkillComponent,
        &PositionComponent,
        &DestinationComponent,
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
        identification_component,
        charactor_component, 
        mut skill_component, 
        position_component, 
        destination_component,
        target_component, 
        resists_component, 
        mut stats_component, 
        mut effect_component
    ) in skills_query.iter_mut() {

        if charactor_component.status == CharactorStatus::Dead {        //if char is dead we skip all passive skills;
            continue;
        }

        let mut skills_for_remove: Vec<SkillType> = vec![];              //skills for remove;

        for (skill_type, skill) in skill_component.passive_skills.iter_mut() {
            let trigger_frequency = skill.trigger_time_frequency;                  // time to trigger skill;
            let current_time = skill.current_time_duration;          // current tick time
            let total_time = skill.total_time_duration;              //total time every tick
            let life_time = skill.life_time;                        // full life time of skill before remove

            if life_time <= total_time {                                    //check for passive skill ends;
                skills_for_remove.push(skill_type.clone());             //store skill sub type for next remove;
                continue;
            }

            if current_time >= trigger_frequency || total_time == 0.0 {      //first run or trigger by time;
                if total_time > 0.0 {                                       //check for trigger time and substruct trigger time from current duration;
                    skill.current_time_duration -= current_time;
                }                
                
                let trigger_chance = skill.trigger_chance;              //check for trigger chance
                if trigger_chance < 100 {
                    let trigger_chance_random_number: u8 = rng.gen_range(0..=99);
                    if trigger_chance < trigger_chance_random_number {  
                        continue;                                           //not triggered
                    }
                }

                let crit_chance = skill.crit_chance;
                let crit_chance_random_number = rng.gen_range(0..=99);
                let crit_multiplier = if crit_chance >= crit_chance_random_number {
                    skill.crit_multiplier
                } else {
                    100
                };

                let skill_target_type = &skill.target;
                let skill_cast_source = &skill.cast_source;

                //create cast position
                let cast_position = match *skill_cast_source {
                    CastSource::Itself => position_component.position.clone(),
                    CastSource::Mouse => panic!("Can not trigger passive skill: {:?}, because cast position is on Mouse!!! Only Active skills can casts from mouse", skill_type),
                };

                if skill.projectiles > 0 {
                    let projectiles = skill.projectiles;
                    let skill_range = skill.range;
                    if skill_range == 0 {
                        println!("Some error in update passive skills, because skill have and projectiles, but skill range = 0!");
                        return;
                    }

                    let projectile_config = deploy.projectile_deploy.get_config(&skill.projectile_type);
                    let mut projectile = Projectile{
                        projectile_type: skill.projectile_type,
                        current_position: cast_position,
                        is_missed: false,
                        damage: skill.damage.clone(),
                        velocity: projectile_config.velocity,
                        pierce_chance: projectile_config.pierce_chance,
                        can_pierce: projectile_config.can_pierce,
                        range: skill.range,
                        ..Default::default()
                    };

                    for (skill_type, chance) in skill.extra_skill.iter() {
                        let random_trigger_chance: u8 = rng.gen_range(0..=99);
                        if *chance < random_trigger_chance {
                            continue;                                           //not triggered;
                        }

                        let skill_config = deploy.charactor_deploy.skills_deploy.get_skill_deploy(skill_type);
                        let skill = Skill::new(skill_config);
                        projectile.passive_skills.push(skill);
                    }

                    for (effect_type, chance) in skill.effect.iter() {
                        let random_trigger_chance: u8 = rng.gen_range(0..=99);
                        if *chance < random_trigger_chance {
                            continue;
                        }
                        projectile.effects.push(effect_type.clone());
                    }

                    let end_point_position: Position<i32> = match target_component.target_position {
                        Some(v) => v.clone(),
                        None => {
                            let direction = &destination_component.destination_direction;
                            if direction.x == 0 && direction.y == 0 {
                                println!("No target and no direction by casting passive skill");
                                return;
                            } else {
                                let x = position_component.position.x + direction.x as i32;
                                let y = position_component.position.y + direction.y as i32;
                                Position {x, y}
                            }
                        },
                    };
                    create_projectile(&mut commands, &material_manager, projectile, end_point_position, projectiles, &skill.skill_direction);
                } else {
                    //buff or debuff skill; if skill range == 0 then we understand skill can buff or debuff self when triggered. We must ignore target_type;
                    if skill.range == 0 {
                        match *skill_cast_source {
                            CastSource::Itself => {
                                do_damage(&skill.damage, &mut stats_component, crit_multiplier, &resists_component.resists);
                                add_effect(&skill.effect, &deploy, &resists_component.resists, &mut effect_component);
                            },
                            CastSource::Mouse => { 
                                println!(
                                    "Can't cast passive skill into MOUSE position! Charactor info: Type:{:?}, RaceType:{:?}, Id:{:?}", 
                                    charactor_component.charactor_type, charactor_component.race_type, identification_component.id
                                );
                            },
                        }       
                    } else {

                        match skill.skill_direction {
                            _ => {
                                println!("Can not cast skill with 0 projectiles, and skilldirection neither ARC360 or point");
                                return;
                            },
                            SkillDirectionType::Arc360 => {                                         //AURA

                            },
                            SkillDirectionType::Point => {                                          // single target skill;

                            },
                        }
                        // AOE Aura

                        //for check target in range of skill
                        let x_min = cast_position.x - skill.range as i32;
                        let x_max = cast_position.x + skill.range as i32;
                        let y_min = cast_position.y - skill.range as i32;
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
                                TargetType::Enemies=> {
                                    match charactor_component.charactor_type {
                                        CharactorType::Player | CharactorType::Companion => {
                                            match target.charactor_type {
                                                CharactorType::Player | CharactorType::Companion => continue,
                                                CharactorType::NPC => continue,
                                                CharactorType::Monster => {},
                                            }
                                        },
                                        CharactorType::NPC => continue,
                                        CharactorType::Monster => {
                                            match target.charactor_type {
                                                CharactorType::Player | CharactorType::Companion => {},
                                                CharactorType::NPC => continue,
                                                CharactorType::Monster => continue,
                                            }
                                        },
                                    }
                                },
                                TargetType::Allies => {
                                    match charactor_component.charactor_type {
                                        CharactorType::Player | CharactorType::Companion => {
                                            match target.charactor_type {
                                                CharactorType::Player | CharactorType::Companion => {},
                                                CharactorType::NPC => continue,
                                                CharactorType::Monster => continue,
                                            }
                                        },
                                        CharactorType::NPC => continue,
                                        CharactorType::Monster => {
                                            match target.charactor_type {
                                                CharactorType::Player | CharactorType::Companion => continue,
                                                CharactorType::NPC => continue,
                                                CharactorType::Monster => {},
                                            }
                                        },
                                    }
                                },
                                TargetType::All => {},
                            }
    
                            //ok if we r here, check the position of target;
                            let target_position_x = target_position.position.x;
                            let traget_position_y = target_position.position.y;
                            if target_position_x >= x_min &&
                                target_position_x <= x_max &&
                                traget_position_y >= y_min &&
                                traget_position_y <= y_max {

                                //bingo, we have a target;
                                do_damage(&skill.damage, &mut stats_component, crit_multiplier, &target_resists.resists);
                                add_effect(&skill.effect, &deploy, &target_resists.resists, &mut target_effects);
                            } else {
                                //position of target not in range;
                                continue;
                            }
                        }
                    }
                }
            }
            //Add time to skill duration time; 0.0 -> +delta;
            skill.current_time_duration += delta;
            skill.total_time_duration += delta;
        }

        for skill_type in skills_for_remove.iter() {            //remove ended skills;
            skill_component.passive_skills.remove(skill_type);
        }
        skills_for_remove.clear();
    }
}

pub fn do_damage(damage: &HashMap<DamageType, i16>, stats: &mut StatsComponent, crit_multiplier: i16, resists: &HashMap<ResistType, i16>){
    for (damage_type, value) in damage.iter() {
        let resist_type = get_resist_from_damage_type(damage_type);
        let resist: i16 =  match resists.get(&resist_type) {
            Some(v) => *v,
            None => {
                println!("Update_passive_skills. Can not get self resist, self have no resist '{:?}' in storage.", resist_type);
                0
            }
        };

        let mut damage_value = value * crit_multiplier / 100;
        damage_value -= damage_value * resist / 100;
        damage_value = if (*damage_type == DamageType::Health || *damage_type == DamageType::Stamina) || damage_value > 0 {
            damage_value
        } else {
            0
        };

        let stat = if *damage_type == DamageType::Stamina {
            Stat::StaminaPoints
        } else {
            Stat::HealthPoints
        };

        charactor::change_health_stamina_points(
            &mut stats.stats,
            &mut stats.stats_cache,
            &stat,
            damage_value,
        );
    }
}

pub fn add_effect(effects: &HashMap<EffectType, u8>, deploy: &Deploy, resists: &HashMap<ResistType, i16>, effect_component: &mut EffectComponent){
    let mut rng = rand::thread_rng();
    for (effect_type, effect_trigger) in effects.iter() {
        let trigger_effect_random_number: u8 = rng.gen_range(0..=99);
        if *effect_trigger >= trigger_effect_random_number {                    //check triegger on effect;
            let effect_config = deploy.charactor_deploy.effects_deploy.get_effect_config(effect_type);
            let mut effect = Effect::new(effect_config);

            let resist_type = resists_types::get_resist_from_effect_type(effect_type);
            let effect_resist = match resists.get(&resist_type) {
                Some(v) => *v,
                None => 0,                                             // if not exist, use 0;
            };

            if effect_resist >= 100 {                               //check for resist this effect
                continue;                                               //ignore that effect;
            };

            effect.duration -= effect.duration * effect_resist as f32 / 100.0;              //reduce effect duration by target resist;        
            effect_component.effects.entry(effect_type.clone()).and_modify(|x| x.duration += effect.duration).or_insert(effect);
        }
    }
}
