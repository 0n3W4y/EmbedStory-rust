use bevy::prelude::*;
use rand::Rng;

use crate::{components::{
    charactor_component::SkillAndEffectComponent,
    IdentificationComponent, ObjectType, TakenDamageComponent, StatsComponent,
}, resources::scene_data::Ability};

use super::{damage_text_informer::DamageTextInformer, Resist, charactor};

pub fn update_damage(
    mut objects_query: Query<
        (
            &IdentificationComponent,
            Option<&mut SkillAndEffectComponent>,
            &mut StatsComponent,
            &mut TakenDamageComponent,
        )
    >,
) {
    for (
        identification, 
        mut skills_and_effects_component, 
        mut stats,
        mut damage,
        ) in objects_query.iter_mut()
    {
        let mut random = rand::thread_rng();
        match identification.object_type {
            ObjectType::Charactor(_, _) => {
                let mut vec_of_damage: Vec<DamageTextInformer> = vec![];
                for taken_damage in damage.damage.iter_mut(){
                    let chance_to_evade = match stats.ability.get(&Ability::Evasion) {
                        Some(v) => *v,
                        None => 0,
                    };
                    let random_number_for_evade_chance = random.gen_range(0..100);
                    if chance_to_evade > random_number_for_evade_chance {
                        let damage_text = DamageTextInformer::new("Evaded".to_string(), false, None);
                        vec_of_damage.push(damage_text);
                        return;                                                                                                     //damage evaded
                    }
                    
                    let block_amount = match stats.ability.get(&Ability::BlockChance) {
                        Some(v) => {
                            match stats.ability.get(&Ability::BlockAmount) {
                                Some(a) => {
                                    let random_chace_to_block = random.gen_range(0..100);
                                    if *v > random_chace_to_block {
                                        *a
                                    } else {
                                        0
                                    }
                                },
                                None => 0,
                            }
                        },
                        None => 0,
                    };

                    for (damage_type, value) in taken_damage.damage.iter() {
                        let resist = match stats.resists.get(&Resist::damage(&damage_type)) {
                            Some(v) => *v,
                            None => 0,
                        };
                        let blocked_value = *value - *value * block_amount / 100;
                        let overall_value = blocked_value - blocked_value * resist / 100;
                        charactor::change_attribute_points(&mut stats, damage_type, overall_value, false);
                        let damage_text = DamageTextInformer::new(overall_value.to_string(),taken_damage.is_critical_hit, Some(damage_type));
                        vec_of_damage.push(damage_text);
                    }

                    match skills_and_effects_component.as_mut() {
                        Some(skills_and_effects) => {
                            for effect in taken_damage.effects.iter_mut() {
                                match skills_and_effects.effect_immunes.iter().find(|&x| *x == effect.effect_type) {
                                    Some(_) => continue,                                                                                    //ignore effect;
                                    None => {
                                        match stats.ability.get(&Ability::ReducingEffectTime) {
                                            Some(v) => {
                                                effect.effect_lifetime -= effect.effect_lifetime * *v as f32 / 100.0;
                                            },
                                            None => {},
                                        };
                                        
                                        match skills_and_effects.effects.get_mut(&effect.effect_type) {                                           //get effect if it already in; prolong lifetime effect, and replace with new effect
                                            Some(v) => {
                                                effect.effect_lifetime += v.effect_lifetime;
                                                match v.over_time_effect.as_mut() {
                                                    Some(over_time_effect) => {
                                                        let time_duration = over_time_effect.time_duration;
                                                        match effect.over_time_effect.as_mut() {
                                                            Some(val) => {
                                                                val.time_duration += time_duration;
                                                            },
                                                            None => {},
                                                        }
                                                    },
                                                    None => {},
                                                }
                                                *v = effect.clone();
                                            },
                                            None => {
                                                for effect_status in effect.effect_status.iter(){                               //store effect status to charactor effect status;
                                                    skills_and_effects.effect_status.push(effect_status.clone());
                                                }
                                                skills_and_effects.effects.insert(effect.effect_type.clone(), effect.clone());
                                            },
                                        }
                                    }
                                }
                            }
        
                            for passive_skill in taken_damage.passive_skills.iter() {
                                match skills_and_effects.passive_skills.get_mut(&passive_skill.skill_type) {
                                    Some(v) => {
                                        let mut new_passive_skill = passive_skill.clone();
                                        new_passive_skill.skill_life_time += v.skill_life_time;
                                        new_passive_skill.current_time_duration += v.current_time_duration;
                                        new_passive_skill.total_duration += v.total_duration;
                                        *v = new_passive_skill;
                                    },
                                    None => {
                                        skills_and_effects.passive_skills.insert(passive_skill.skill_type.clone(), passive_skill.clone());
                                    },
                                }
        
                            }
                        },
                        None => {},
                    }
                }
                damage.damage.clear();

                for text in vec_of_damage {
                    damage.text.push(text);
                }
            },
            ObjectType::Thing(_) => {
                let mut vec_of_text_damage: Vec<DamageTextInformer> = vec![];
                for taken_damage in damage.damage.iter(){ 
                    for (damage_type, value) in taken_damage.damage.iter() {
                        charactor::change_attribute_points(&mut stats, damage_type, *value, false);
                        let text_informer = DamageTextInformer::new(value.to_string(),taken_damage.is_critical_hit, Some(damage_type));
                        vec_of_text_damage.push(text_informer);
                    }
                }

                for text in vec_of_text_damage {
                    damage.text.push(text);
                }
            },
            _ => {},
        }
    }
}
