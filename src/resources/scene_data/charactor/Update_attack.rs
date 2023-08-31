use bevy::prelude::*;
use rand::Rng;

use crate::components::charactor_component::{
    AbilityComponent, ActionType, CharactorComponent, EffectComponent,
    ExtraStatsComponent, PositionComponent,
    ResistsComponent, SkillComponent, CharactorTargetComponent, CharactorTextComponent,
};

use crate::resources::deploy::Deploy;
use crate::resources::scene_data::charactor::{self, SkillSlot};
use crate::resources::scene_data::stuff::damage_type::DamageType;

use super::damage_text_informer::DamageTextInformer;
use super::effects::{Effect, StatDamageType};
use super::{abilities::AbilityType, skills::Skill, stats::ExtraStat, CharactorStatus};

pub fn attacking_from_basic_skill(
    mut commands: Commands,
    mut charactor_query: Query<(
        &CharactorComponent,
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
    )>,

    deploy: Res<Deploy>,
) {
    for (
        charactor, 
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
            mut target_text_component
        ) in target_query.iter_mut() {
            if target_id == target_component.id {
                attack(
                    &mut commands,
                    skill, 
                    charactor_ability,
                    &mut target_text_component, 
                    target_resists, 
                    &mut target_extra_stats, 
                    &mut target_effects, 
                    target_abilities, 
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
    commands: &mut Commands,
    skill: &mut Skill,
    charactor_ability: &AbilityComponent,
    target_text_component: &mut CharactorTextComponent,
    target_resists: &ResistsComponent,
    target_extra_stats: &mut ExtraStatsComponent,
    target_effect: &mut EffectComponent,
    target_ability: &AbilityComponent,
    deploy: &Deploy,
) {
    let effects_deploy = &deploy.charactor_deploy.effects_deploy;
    let skills_deploy = &deploy.charactor_deploy.skills_deploy;
    let mut rng = rand::thread_rng();

    //set skill on cooldown;
    skill.on_cooldown = true;

    //check for melee or ranged+magic attack;
    if skill.projectiles > 0 {
        todo!();
        //TODO: create projectile;
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
            //TODO: take this value to target, interface ( sprite ) need to text it to user; "MISS";
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
                //TODO: take this value to target, interface ( sprite ) need to text it to user; "EVADED";
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

        //create text damage, take it to target text into userinterface
        //create vec of damage ; Maybe i'll do color damage;
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
        for (effect_type, trigger_chace) in skill.effect.iter_mut() {
            //check for trigger effect
            let trigger_chance_random_number: u8 = rng.gen_range(0..=99);
            if *trigger_chace < trigger_chance_random_number{
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

            //check for passivly skills on damage
            for (skill_type, value) in skill.passive_skill.iter() {
                let mut skill = Skill::new(skills_deploy.get_skill_deploy(skill_type));

                for (damage_type, value) in skill.damage.iter_mut() {
                    *value = match *damage_type {
                        DamageType::Fire => {
                            let damage_multuplier_from_ability = match charactor_ability.ability.get(&AbilityType::FireDamage) {
                                Some(v) => *v,
                                None => 100,
                            };
                            let resist_from_target = match target_ability.ability.get(&AbilityType::FireDamage) {
                                Some(v) => *v,
                                None => 0,
                            };
                            (*value * damage_multuplier_from_ability / 100) - (*value * resist_from_target / 100)
                        },
                        DamageType::Cold => todo!(),
                        DamageType::Electric => todo!(),
                        DamageType::Cutting => todo!(),
                        DamageType::Piercing => todo!(),
                        DamageType::Crushing => todo!(),
                        DamageType::Water => todo!(),
                        DamageType::Acid => todo!(),
                        DamageType::Poison => todo!(),
                    }
                }
            }          
        }
    }
}
