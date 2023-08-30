use bevy::prelude::*;
use rand::Rng;

use crate::components::charactor_component::{
    AbilityComponent, ActionType, CharactorComponent, EffectComponent,
    ExtraStatsComponent, PositionComponent,
    ResistsComponent, SkillComponent, CharactorTargetComponent, CharactorTextComponent,
};

use crate::resources::deploy::Deploy;
use crate::resources::deploy::charactor_deploy::EffectsDeploy;
use crate::resources::scene_data::charactor::{self, SkillSlot};

use super::damage_text_informer::{DamageTextInformer, DamageColorType};
use super::effects::Effect;
use super::{abilities::AbilityType, skills::Skill, stats::ExtraStat, CharactorStatus};

pub fn attacking_from_basic_skill(
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
    )>,

    deploy: Res<Deploy>,
) {
    let effects_deploy = &deploy.charactor_deploy.effects_deploy;
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
        //for safe;
        let target_id = match charactor_target.target {
            Some(v) => v,
            None => {
                println!("Can not attack, because charactor '{:?}, {:?}, {:?}' have Attack action, but doesnt have a target", charactor.charactor_type, charactor.gender_type, charactor.race_type);
                continue;
            }
        };

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
    skill: &mut Skill,
    ability_component: &AbilityComponent,
    target_text_component: &CharactorTextComponent,
    target_resists: &ResistsComponent,
    target_extra_stats: &mut ExtraStatsComponent,
    target_effect: &mut EffectComponent,
    target_ability: &AbilityComponent,
    effects_deploy: &EffectsDeploy,
) {
    let mut rng = rand::thread_rng();
    //set skill on cooldown;
    skill.on_cooldown = true;

    //check for melee or ranged+magic attack;
    if skill.projectiles > 0 {
        todo!();
        //TODO: create projectile;
    } else {
        // let check for accuracy
        let accuracy = match ability_component.ability.get(&AbilityType::Accuracy) {
            Some(v) => *v,
            _ => {
                println!("Can't get Accuracy, use 0.0 instead, so 100% chance to miss");
                0
            }
        };

        if accuracy <= 0 {
            //TODO: take this value to target, interface ( sprite ) need to text it to user; "MISS";
            target_text_component.text_upper_charactor.push( DamageTextInformer::new("MISS".to_string(), false, DamageColorType::Gray)); 
            return;
        } else if accuracy >= 100 {

        } else {
            let random_accuracy_number: u8 = rng.gen_range(0..=99);
            if accuracy <= random_accuracy_number as i16 {
                //TODO: take this value to target, interface ( sprite ) need to text it to user; "MISS";
                return;
            }
        }

        // if we here, let chech the evasion of tagert;
        let target_evasion = match target_ability.ability.get(&AbilityType::Evasion) {
            Some(v) => *v,
            _ => {
                println!("Target has no ability Evasion, so i use 0 instead");
                0.0
            }
        };

        if target_evasion > 0.0 {
            let random_evasion_number: u8 = rng.gen_range(0..=99);
            if target_evasion >= random_evasion_number as f32 {
                //TODO: take this value to target, interface ( sprite ) need to text it to user; "EVADED";
                return;
            }
        }

        //so if we are here, let's get damage types and resists
        //let chect for block chanse;
        let block_amount: f32 = match target_ability.ability.get(&AbilityType::BlockAmount) {
            Some(v) => *v,
            _ => {
                println!("Target has no block amount, i use 0 instead");
                0.0
            }
        };

        let block_percent: f32 = match target_ability.ability.get(&AbilityType::BlockChance) {
            Some(v) => *v,
            _ => {
                println!("Target has no block chance, i use 0 istead");
                0.0
            }
        };

        let block_chance_random_number: u8 = rng.gen_range(0..=99);
        let is_blocked: bool = if block_percent >= block_chance_random_number as f32 {
            true
        } else {
            false
        };

        //create text damage, take it to target text into userinterface
        //create vec of damage ; Maybe i'll do color damage;
        for (damage_type, value) in skill.current_damage {
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
            let damage_value = if is_blocked {
                let new_value = value - (value as f32 * block_percent / 100.0) as i16;
                new_value - (new_value * target_damage_resist / 100) as i16
            } else {
                value - (value * target_damage_resist / 100) as i16
            };

            charactor::change_extra_stat_current(
                &mut target_extra_stats.extra_stats,
                &mut target_extra_stats.extra_stats_cache,
                &ExtraStat::HealthPoints,
                damage_value,
            );
            //TODO:: Take value to target;
        }

        //now we need to set effect to target;
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

            //check effect for damage to HP or SP {
            if effect.change_extra_stat_is_damage {
                // set damage to extra_stats;
                //get weapon damage from inventory
                let damage_from_weapon = match inventory_component.stuff_wear.get(&charactor::StuffWearSlot::Weapon) {
                    Some(v) => {
                        match *v {
                            Some(f) => {
                                match f.current_damage.get(&effect.damage_type) {
                                    Some(d) => *d,
                                    None => {
                                        println!("Can not get damage from weapon damage type: '{:?}'. Weapon type: {:?}", &effect.damage_type, f.stuff_subtype );
                                        0
                                    },
                                }
                            },
                            None => {
                                println!("Can not get weapon from inventory storage in weapon slot. I use 0 instead");
                                0
                            },
                        }
                    },
                    None => {
                        println!("Can not get weapon slot from inventory storage. I use 0 instead");
                        0
                    },
                };

                let resist_damage_from_target = match target_resists.damage_resists.get(&effect.damage_type) {
                    Some(v) => *v,
                    None => {
                        println!("Can not get damage resists: '{:?}' in target resists. I use 0", &effect.damage_type);
                        0
                    },
                };
                
                for (_, value) in effect.change_extra_stat.iter_mut() {
                    *value = damage_from_weapon - (damage_from_weapon * resist_damage_from_target / 100);
                };
            }

            

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
                    return;
                }

                //calculate new effect duration by target resist;
                let effect_duration = effect.duration * target_effect_resist as f32 / 100.0;
                effect.duration -= effect_duration;

                let old_effect = target_effect.temporary_effect.entry(effect_type.clone()).and_modify(|x| x.duration += effect.duration).or_insert(effect);
                
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
}
