use bevy::prelude::*;
use rand::Rng;

use crate::components::charactor_component::{
    AbilityComponent, ActionType, CharactorComponent, EffectComponent,
    ExtraStatsComponent, PositionComponent,
    ResistsComponent, SkillComponent, CharactorTargetComponent, CharactorTextComponent,
};

use crate::components::projectile_component::Projectile;
use crate::resources::deploy::Deploy;
use crate::resources::scene_data::charactor::{self, SkillSlot};
use crate::resources::scene_data::projectiles::ProjectileType;
use crate::resources::scene_data::projectiles::update_projectile::create_projectile;
use crate::resources::scene_data::stuff::damage_type::DamageType;

use super::damage_text_informer::DamageTextInformer;
use super::effects::{Effect, StatDamageType};
use super::{abilities::AbilityType, skills::Skill, stats::ExtraStat, CharactorStatus};

pub fn attacking_from_basic_skill(
    mut commands: Commands,
    mut charactor_query: Query<(
        &CharactorComponent,
        &PositionComponent,
        &mut SkillComponent,
        &CharactorTargetComponent,
        &AbilityComponent,
    )>,

    mut target_query: Query<(
        &CharactorComponent,
        &mut ExtraStatsComponent,
        &ResistsComponent,
        &mut EffectComponent,
        &PositionComponent,
        &AbilityComponent,
        &mut CharactorTextComponent,
        &mut SkillComponent,
    )>,

    deploy: Res<Deploy>,
) {
    for (
        charactor, 
        charactor_position,
        mut charactor_skill, 
        charactor_target, 
        charactor_ability, 
    ) in charactor_query.iter_mut() {
        //check for attack
        if charactor.status != CharactorStatus::CanAttack {
            continue;
        }

        //let's attack or create projectile to attack;

        let target_id = match charactor_target.target {
            Some(v) => v,
            None => {
                println!("Can not attack, because charactor '{:?}, {:?}, {:?}' have Attack action, but doesnt have a target", charactor.charactor_type, charactor.gender_type, charactor.race_type);
                continue;
            }
        };

        //safe, we r already check for skill previosly;
        let skill = charactor_skill.skills.get_mut(&SkillSlot::Base).unwrap();

        for (
            target_component,
            mut target_extra_stats, 
            target_resists, 
            mut target_effects, 
            target_position, 
            target_abilities,
            mut target_text_component,
            mut target_skills,
        ) in target_query.iter_mut() {
            if target_id == target_component.id {
                attack(
                    commands,
                    skill, 
                    charactor_ability,
                    charactor_position,
                    target_position,
                    &mut target_text_component, 
                    target_resists, 
                    &mut target_extra_stats, 
                    &mut target_effects, 
                    target_abilities, 
                    &mut target_skills,
                    &deploy
                );
                break;
            }
        }
    }
}





pub fn update_attack_from_basic_skill(
    mut charactor_query: Query<(
        &CharactorComponent,
        &mut SkillComponent,
        &CharactorTargetComponent,
        &PositionComponent
    )>,

    mut target_query: Query<(
        &CharactorComponent,
        &PositionComponent,
    )>
) {
    

    for (
        charactor, 
        mut charactor_skill, 
        charactor_target, 
        charactor_position
    ) in charactor_query.iter_mut() {
        if charactor_target.action != ActionType::Attack {
            continue;
        }        

        let target_id = match charactor_target.target {
            Some(v) => v,
            None => {
                println!("Can not attack, because charactor '{:?}, {:?}, {:?}' have Attack action, but doesnt have a target", charactor.charactor_type, charactor.gender_type, charactor.race_type);
                continue;
            }
        };
        //get base attack skill
        match charactor_skill.skills.get_mut(&SkillSlot::Base) {
            Some(skill) => {
                //check for colldown
                if skill.on_cooldown {
                    //go to next charactor;
                    continue;
                }

                for (
                    target,
                    target_position,
                ) in target_query.iter_mut() {
                    //check for target
                    if target.id == target_id {
                        //try to attack;
                        if try_to_attack(charactor_position, target_position, skill) {
                            //for animation; when animation ends = attacking change to None or Stand;
                            charactor.status = CharactorStatus::CanAttack;
                            break;
                        }
                        println!("Can not attacking target, becasue target not a monster, or player or companion or not NPC");
                    }
                }
            },
            None => {
                println!("can't attacking target, because autoattack skill not found in skills storage");
            },
        }        
    }
}

fn try_to_attack(
    position: &PositionComponent,
    target_position: &PositionComponent,
    skill: &mut Skill,
) -> bool {
    //check for target position;
    let skill_range = skill.range;
    let diff_x = (position.position.x - target_position.position.x).abs(); // always positive value;
    let diff_y = (position.position.y - target_position.position.y).abs(); // always positive value;
    let diff = diff_x.max(diff_y);

    //check for skill range;
    if skill_range as i32 >= diff {
            return true;
    } else {
        return false;
    }
}

fn attack(
    mut commands: Commands,
    skill: &mut Skill,
    charactor_ability: &AbilityComponent,
    charactor_position: &PositionComponent,
    target_position: &PositionComponent,
    target_text_component: &mut CharactorTextComponent,
    target_resists: &ResistsComponent,
    target_extra_stats: &mut ExtraStatsComponent,
    target_effect: &mut EffectComponent,
    target_ability: &AbilityComponent,
    target_skills: &mut SkillComponent,
    deploy: &Deploy,
) {
    let effects_deploy = &deploy.charactor_deploy.effects_deploy;
    let skills_deploy = &deploy.charactor_deploy.skills_deploy;
    let mut rng = rand::thread_rng();

    //set skill on cooldown;
    skill.on_cooldown = true;

    //check for melee or ranged+magic attack;
    if skill.projectiles > 0 {
        //create default projectile component;
        let mut projectile_component = Projectile {
            ..Default::default()
        };

        //change projectile type by weapon type 
        projectile_component.projectile_type = match skill.projectile_type {
            Some(v) => v.clone(),
            None => {
                println!("Can not get projectile type from Base skill: '{:?}', '{:?}, {:?}', use 'Arrow' projectile type", skill.skill_name, skill.skill_type, skill.skill_subtype);
                ProjectileType::Arrow
            },
        };

        //insert to projectile starting point and destination point;
        projectile_component.starting_position.x = charactor_position.position.x;
        projectile_component.starting_position.y = charactor_position.position.y;
        projectile_component.destination_point.x = target_position.position.x;
        projectile_component.destination_point.y = target_position.position.y;

        //let check for accuracy
        let accuracy = match charactor_ability.ability.get(&AbilityType::Accuracy) {
            Some(v) => *v,
            _ => {
                println!("Can't get Accuracy, use 0 instead, so 100% chance to miss");
                0
            }
        };

        let random_accuracy_number: u8 = rng.gen_range(0..=99);
        if accuracy <= 0 || accuracy < random_accuracy_number as i16 {
            projectile_component.is_missed = true;
            create_projectile(commands, projectile_component, deploy);
            //if accuracy <= 0 we r create projectile with @miss field; and return from func;
            return;
        };

        //if not missed
        //clone damage values;
        projectile_component.damage = skill.damage.clone();

        //check for trigger effects and passive skills;
        for (effect_type, trigger_chance) in skill.effect.iter() {
            if *trigger_chance <= 0 {
                continue;
            }

            let random_trigger_chance_number: u8 = rng.gen_range(0..=99);

            if random_trigger_chance_number > *trigger_chance {
                continue;
            }

            projectile_component.effects.push(effect_type.clone());                  
        }

        for (skill_type, trigger_chance) in skill.passive_skill.iter() {
            if *trigger_chance < 100 && *trigger_chance > 0 {

            }
        }

    } else {
        // let check for accuracy
        let accuracy = match charactor_ability.ability.get(&AbilityType::Accuracy) {
            Some(v) => *v,
            _ => {
                println!("Can't get Accuracy, use 0 instead, so 100% chance to miss");
                0
            }
        };

        let random_accuracy_number: u8 = rng.gen_range(0..=99);
        if accuracy <= 0 || accuracy < random_accuracy_number as i16 {
            target_text_component.text_upper_charactor.push( DamageTextInformer::new("MISS".to_string(), false, None)); 
            return;
        }

        // if we here, let chech the evasion of tagert;
        let target_evasion = match target_ability.ability.get(&AbilityType::Evasion) {
            Some(v) => *v,
            _ => {
                println!("Target has no ability Evasion, so i use 0 instead");
                0
            }
        };

        //check for target evasion;
        if target_evasion > 0 {
            let random_evasion_number: u8 = rng.gen_range(0..=99);
            if target_evasion >= random_evasion_number as i16 {
                target_text_component.text_upper_charactor.push( DamageTextInformer::new("EVADED".to_string(), false, None)); 
                return;
            }
        }

        //so if we are here, let's get damage types and resists
        //let chect for block amount cnad chanse;
        let mut block_amount: i16 = 0;
        let block_chance: i16 = match target_ability.ability.get(&AbilityType::BlockChance) {
            Some(v) => *v,
            _ => {
                println!("Target has no block chance, i use 0 istead");
                0
            }
        };

        let block_chance_random_number: u8 = rng.gen_range(0..=99);
        if block_chance >= block_chance_random_number as i16 {
            block_amount = match target_ability.ability.get(&AbilityType::BlockAmount) {
                Some(v) => *v,
                _ => {
                    println!("Target has no block amount, i use 0 instead");
                    0
                }
            };
        }

        //create  damage 
        for (damage_type, value) in skill.damage.iter() {
            let target_damage_resist = match target_resists.damage_resists.get(&damage_type) {
                Some(v) => *v,
                _ => {
                    println!(
                        "Target has no damage resist: '{:?}', I use 0 instead",
                        damage_type
                    );
                    0
                }
            };

            let damage_value = value - (value * target_damage_resist / 100) - (value * block_amount / 100 );

            charactor::change_extra_stat_current(
                &mut target_extra_stats.extra_stats,
                &mut target_extra_stats.extra_stats_cache,
                &ExtraStat::HealthPoints,
                damage_value,
                &StatDamageType::Flat,
            );
            target_text_component.text_upper_charactor.push( DamageTextInformer::new(damage_value.to_string(), false, Some(damage_type))); 
        }

        //now we need to set effect to target if effects have on skill;
        for (effect_type, trigger_chance) in skill.effect.iter_mut() {
            //check for trigger effect
            let trigger_chance_random_number: u8 = rng.gen_range(0..=99);
            if *trigger_chance < trigger_chance_random_number {
                //skip effect, because not triggered;
                continue;
            };
            
            //create new effect;
            let effect_config = effects_deploy.get_effect_config(effect_type);
            let mut effect = Effect::new(effect_config);

            //check for ednless effect or temporary
            if effect.duration == 0.0 {
                //try to insert, or ignore if effect already exist;
                //Maybe change damage todo();
                target_effect.endless_effect.entry(effect_type.clone()).or_insert(effect);
            } else {
                //temporary
                //get resist from target on this effect to change duration;
                let target_effect_resist = match target_resists.effect_resists.get(effect_type) {
                    Some(v) => *v,
                    _ => {
                        println!(
                            "Target has no effect resist: '{:?}', I use 0 instead",
                            effect_type
                        );
                        0
                    }
                };

                //check target resist; if it > 100% just ignore this effect;
                if target_effect_resist > 100 {
                    continue;
                }

                //calculate new effect duration by target resist;
                let effect_duration = effect.duration * target_effect_resist as f32 / 100.0;
                effect.duration -= effect_duration;

                let old_effect = target_effect.temporary_effect.entry(effect_type.clone()).and_modify(|x| x.duration += effect.duration).or_insert(effect);
            }
        }
        
        //check for passivly skills on damage
        for (skill_type, value) in skill.passive_skill.iter() {
            let mut skill = Skill::new(skills_deploy.get_skill_deploy(skill_type));

            for (damage_type, value) in skill.damage.iter_mut() {
                let target_resist_multiplier = match target_resists.damage_resists.get(damage_type) {
                    Some(v) => *v,
                    None => 0,
                };
                let damage_multuplier_from_ability: i16 = match *damage_type {
                    DamageType::Fire => {
                        match charactor_ability.ability.get(&AbilityType::FireDamage) {
                            Some(v) => *v,
                            None => 100,
                        }
                    },
                    DamageType::Cold => {
                        match charactor_ability.ability.get(&AbilityType::ColdDamage) {
                            Some(v) => *v,
                            None => 100,
                        }
                    },
                    DamageType::Electric => {
                        match charactor_ability.ability.get(&AbilityType::ElectricDamage) {
                            Some(v) => *v,
                            None => 100,
                        }
                    },
                    DamageType::Cutting => {
                        match charactor_ability.ability.get(&AbilityType::CuttingDamage) {
                            Some(v) => *v,
                            None => 100,
                        }
                    },
                    DamageType::Piercing => {
                        match charactor_ability.ability.get(&AbilityType::PiercingDamage) {
                            Some(v) => *v,
                            None => 100,
                        }
                    },
                    DamageType::Crushing => {
                        match charactor_ability.ability.get(&AbilityType::CrushingDamage) {
                            Some(v) => *v,
                            None => 100,
                        }
                    },
                    DamageType::Water => {
                            match charactor_ability.ability.get(&AbilityType::WaterDamage) {
                            Some(v) => *v,
                            None => 100,
                        }
                    },
                    DamageType::Acid => {
                        match charactor_ability.ability.get(&AbilityType::AcidDamage) {
                            Some(v) => *v,
                            None => 100,
                        }
                    },
                    DamageType::Poison => {
                        match charactor_ability.ability.get(&AbilityType::PoisonDamage) {
                            Some(v) => *v,
                            None => 100,
                        }
                    },
                }; 
                let temp_value = (*value as f32 + (*value as f32 * damage_multuplier_from_ability as f32 / 100.0)) as i16;
                *value = temp_value - (temp_value as f32 * target_resist_multiplier as f32 / 100.0) as i16;
                                    
            }
            //add skill time;
            target_skills.passive_skills.entry(skill_type.clone()).and_modify(|x| x.trigger_time += skill.trigger_time).or_insert(skill);
        } 
    }
}
