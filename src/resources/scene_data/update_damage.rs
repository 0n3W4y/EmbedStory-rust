use bevy::prelude::*;
use rand::Rng;

use crate::{components::{
    charactor_component::SkillAndEffectComponent, IdentificationComponent, ObjectType, PositionComponent, StatsComponent, TakenDamage, TakenDamageComponent
}, resources::scene_data::Ability};

use super::{damage_text_informer::DamageTextInformer, Resist, charactor};

pub fn update_damage(
    mut source_query: Query<
        (
            &IdentificationComponent,
            &PositionComponent,
            Option<&mut SkillAndEffectComponent>,
            &mut StatsComponent,
            &mut TakenDamageComponent,
        )
    >,
    mut target_query: Query<
        (
            &IdentificationComponent,
            &PositionComponent,
            &mut TakenDamageComponent,
        )
    >,
) {
    for (
        identification, 
        position,
        mut skills_and_effects_component, 
        mut stats,
        mut damage,
        ) in source_query.iter_mut()
    {
        let mut vec_of_text_damage: Vec<DamageTextInformer> = vec![];
        for taken_damage in damage.damage.iter_mut() {
            let area_of_impact = taken_damage.area_of_impact;
            if area_of_impact == 0 {                                                                                                            //damage only self;
                match identification.object_type {
                    ObjectType::Charactor(_, _) => {
                        match skills_and_effects_component.as_mut() {
                            Some(mut skills_and_effects) => {
                                do_damage_to_charactor(taken_damage, &mut stats, &mut skills_and_effects, &mut vec_of_text_damage);
                            },
                            None => {},
                        }
                    },
                    ObjectType::Thing(_) => {
                        do_damage_to_thing(taken_damage, &mut stats, &mut vec_of_text_damage);
                    },
                    _ => panic!("There is no {:?} in damage taken function", identification.object_type)
                }
            }else {
                match identification.object_type {
                    ObjectType::Charactor(_, _) => {
                        match skills_and_effects_component.as_mut() {
                            Some(mut skills_and_effects) => {
                                do_damage_to_charactor(taken_damage, &mut stats, &mut skills_and_effects, &mut vec_of_text_damage);         //damage to self;
                            },
                            None => {},
                        }
                    },
                    ObjectType::Thing(_) => {
                        do_damage_to_thing(taken_damage, &mut stats, &mut vec_of_text_damage);
                    },
                    _ => panic!("There is no {:?} in damage taken function", identification.object_type)
                }

                let source_position_x = position.position.x;
                let source_position_y = position.position.y;
                let skill_x_min = source_position_x - area_of_impact as i32;
                let skill_x_max = source_position_x + area_of_impact as i32;
                let skill_y_min = source_position_y - area_of_impact as i32;
                let skill_y_max = source_position_y + area_of_impact as i32;

                for (
                    target_identification, 
                    target_position,
                    mut target_damage,
                    ) in target_query.iter_mut() 
                {
                    let target_position_x = target_position.position.x;
                    let target_position_y = target_position.position.y;
                    if source_position_x == target_position_x && source_position_y == target_position_y {
                        continue;
                    };

                    if (target_position_x >= skill_x_min && target_position_x <= skill_x_max) && (target_position_y >= skill_y_min && target_position_y <= skill_y_max) {
                        let mut new_taken_damage = taken_damage.clone();
                        new_taken_damage.area_of_impact = 0;                                                                                                            //change this to prevent circle of damage reaction;
                        match target_identification.object_type {
                            ObjectType::Charactor(_, _) => target_damage.damage.push(new_taken_damage),
                            ObjectType::Thing(_) => target_damage.damage.push(new_taken_damage),
                            _ => panic!("There is no {:?} in damage taken function", identification.object_type)
                        }
                    } else {
                        continue;
                    }
                }
            }
        }
        damage.damage.clear();

        for text in vec_of_text_damage {
            damage.text.push(text);
        }
    }
}

fn do_damage_to_charactor(
    taken_damage: &mut TakenDamage, 
    stats: &mut StatsComponent, 
    skills_and_effects: &mut SkillAndEffectComponent, 
    vec_of_text_damage: &mut Vec<DamageTextInformer>
) {
    let mut random = rand::thread_rng();
    if !taken_damage.no_evade {
        let chance_to_evade = match stats.ability.get(&Ability::Evasion) {
            Some(v) => *v,
            None => 0,
        };
        let random_number_for_evade_chance = random.gen_range(0..100);
        if chance_to_evade > random_number_for_evade_chance {
            let damage_text = DamageTextInformer::new("Evaded".to_string(), false, None);
            vec_of_text_damage.push(damage_text);
            return;                                                                                                                  //damage evaded
        }
    }                    
    
    let block_amount = match stats.ability.get(&Ability::BlockChance) {
        Some(v) => {
            match stats.ability.get(&Ability::BlockAmount) {
                Some(a) => {
                    let random_chace_to_block = random.gen_range(0..100);
                    if *v > random_chace_to_block && !taken_damage.no_block {
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
        let value_after_blocking = *value - *value * block_amount / 100;
        let overall_damage = value_after_blocking - value_after_blocking * resist / 100;
        charactor::change_attribute_points(stats, damage_type, overall_damage, false);
        let damage_text = DamageTextInformer::new(overall_damage.to_string(),taken_damage.is_critical_hit, Some(damage_type));
        vec_of_text_damage.push(damage_text);
    }

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
}

fn do_damage_to_thing(taken_damage: &mut TakenDamage, stats: &mut StatsComponent, vec_of_text_damage: &mut Vec<DamageTextInformer>) {
    for (damage_type, value) in taken_damage.damage.iter() {
        charactor::change_attribute_points(stats, damage_type, *value, false);
        let text_informer = DamageTextInformer::new(value.to_string(),taken_damage.is_critical_hit, Some(damage_type));
        vec_of_text_damage.push(text_informer);
    }
}
