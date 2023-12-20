use bevy::prelude::*;

use crate::components::{StatsComponent, ResistsComponent, AttributesComponent, DamageTextComponent};
use crate::components::charactor_component::{
    AbilityComponent, CharactorComponent, EffectComponent,
    SkillComponent, InventoryComponent,
};
use crate::resources::scene_data::{AbilityType, get_resist_from_damage_type};
use crate::resources::scene_data::charactor::{self, skills};
use crate::resources::scene_data::damage_text_informer::DamageTextInformer;
use super::effects::EffectType;
use super::{CharactorStatus, change_stat_points};

pub fn add_new_effect(
    mut charactor_query: Query<
    (
        &mut EffectComponent,
        &StatsComponent,
        &AttributesComponent,
        &AbilityComponent
    ),
    With<CharactorComponent>    
    >
) {
    for (
        mut effects,
        stats,
        attrbiutes,
        abilities
    ) in charactor_query.iter_mut() {
        for effect in effects.added_effect.iter_mut(){
            for immune_effect_type in effects.effect_immunes.iter() {                                           //check immune for new added effect;
                if effect.effect_type == *immune_effect_type {
                    continue;
                }
            }

            match abilities.ability.get(&AbilityType::ReducingEffectTime) {
                Some(v) => {
                    effect.effect_lifetime -= effect.effect_lifetime * *v as f32 / 100.0;
                },
                None => {},
            };

            match effect.buff_debuff_effect {
                Some(mut buff_debuff_effect) => {
                    for (stat, value) in buff_debuff_effect.change_stat.iter_mut() {
                        match stats.stats.get(stat) {                                                           //convert percent to flat;
                            Some(v) => *value = *value * *v / 100,
                            None => *value = 0,
                        }
                    }

                    for (attribute, value) in buff_debuff_effect.change_attribute_cache.iter_mut() {
                        match attrbiutes.attributes_cache.get(attribute) {                                      //convert percent to flat;
                            Some(v) => *value = *value * *v / 100,
                            None => *value = 0, 
                        }                                                                    
                    }


                },
                None => {},
            }

            for effect_status in effect.effect_status.iter(){                               //store effect status to charactor effect status;
                effects.effect_status.push(effect_status.clone());
            }
            
            effects.effects.entry(effect.effect_type.clone()).and_modify(|x| x.effect_lifetime += effect.effect_lifetime).or_insert(effect.clone());
        }

        effects.added_effect.clear();
    }
}

pub fn update_effects(
    mut charactors_query: Query<
        (
            &CharactorComponent,
            &mut EffectComponent,
            &mut StatsComponent,
            &mut AttributesComponent,
            &mut ResistsComponent,
            &mut AbilityComponent,
            &mut SkillComponent,
            &InventoryComponent,
            &mut DamageTextComponent,
        ),
        With<CharactorComponent>,
    >
) {
    let delta_time: f32 = 0.1;                                                              //this function running with criteria triggered by 0.1 sec;
    for (
        charactor_component,
        mut effects, 
        mut stats, 
        mut attributes,
        mut resists, 
        mut abilities, 
        mut skills,
        inventory,
        mut damage_text,

    ) in charactors_query.iter_mut() {
        if charactor_component.status == CharactorStatus::Dead {                            //check for dead
            continue;                                                                       //do nothing with dead charactors;
        };

        let mut effects_to_remove:Vec<EffectType> = vec![];                                     //create vec of effects for deleting, which one ends at this moment;

        for (_, effect) in effects.effects.iter_mut() {                                     //update  effects;
            if effect.time_duration == 0.0 {
                match effect.buff_debuff_effect {
                    Some(buff_debuff_effect) => {
                        for (stat, stat_damage) in buff_debuff_effect.change_stat.iter() {
                            change_stat_points(                    
                                &mut stats,
                                &mut resists.resists,
                                &mut abilities.ability,
                                &mut attributes,
                                stat,
                                *stat_damage,
                            );
                        }

                        for (attribute_cache, attribute_damage) in buff_debuff_effect.change_attribute_cache.iter() {
                            charactor::change_attribute_points(&mut attributes, attribute_cache, *attribute_damage, true);
                        }

                        for (resist, resists_damage) in buff_debuff_effect.change_resist.iter() {
                            charactor::change_resist(&mut resists.resists, resist, *resists_damage);
                        }

                        for (ability, ability_damage) in buff_debuff_effect.change_ability .iter(){
                            charactor::change_ability(&mut abilities.ability, &ability, *ability_damage);
                        }

                        skills::update_basic_skill_by_changes_in_ability(                        //update base skill by changes in abilities and stats;
                            &mut skills.base_skill,
                            &abilities.ability, 
                            &inventory.stuff_wear
                            );

                    },
                    None => {},
                }

                match effect.over_time_effect {
                    Some(over_time_effect) => {
                        for (attribute, attribute_damage) in over_time_effect.change_attributes.iter() {
                            let new_attribute_damage = match resists.resists.get(&get_resist_from_damage_type(&over_time_effect.effect_damage_type)) {
                                Some(v) => *attribute_damage - *attribute_damage * *v / 100,
                                None => *attribute_damage,
                            };
                            charactor::change_attribute_points(&mut attributes, attribute, new_attribute_damage, false);
                            damage_text.text_upper.push(DamageTextInformer::new(new_attribute_damage, None, false, Some(&over_time_effect.effect_damage_type)));
                        }
                    },
                    None => {},
                }
            } else if effect.time_duration >= effect.effect_lifetime {
                match effect.buff_debuff_effect {
                    Some(buff_debuff_effect) => {
                        for (stat, stat_damage) in buff_debuff_effect.change_stat.iter() {
                            change_stat_points(                    
                                &mut stats,
                                &mut resists.resists,
                                &mut abilities.ability,
                                &mut attributes,
                                stat,
                                -(*stat_damage),
                            );
                        }

                        for (attribute_cache, attribute_damage) in buff_debuff_effect.change_attribute_cache.iter() {
                            charactor::change_attribute_points(&mut attributes, attribute_cache, -(*attribute_damage), true);
                        }

                        for (resist, resists_damage) in buff_debuff_effect.change_resist.iter() {
                            charactor::change_resist(&mut resists.resists, resist, -(*resists_damage));
                        }

                        for (ability, ability_damage) in buff_debuff_effect.change_ability .iter(){
                            charactor::change_ability(&mut abilities.ability, &ability, -(*ability_damage));
                        }

                        skills::update_basic_skill_by_changes_in_ability(                        //update base skill by changes in abilities and stats;
                            &mut skills.base_skill,
                            &abilities.ability, 
                            &inventory.stuff_wear
                            );

                    },
                    None => {},
                }
                if effect.effect_status.len() > 0 {
                    for effect_status in effect.effect_status.iter() {
                        let index = effect.effect_status.iter().position(|x| x == effect_status).unwrap();
                        effect.effect_status.remove(index);
                    }
                }
                effects_to_remove.push(effect.effect_type.clone());
            } else {
                effect.time_duration += delta_time;
                match effect.over_time_effect {
                    Some(mut over_time_effect) => {
                        over_time_effect.time_duration += delta_time;
                        if over_time_effect.time_duration < over_time_effect.trigger_time_effect {
                            continue;
                        } else {
                            over_time_effect.time_duration -= over_time_effect.trigger_time_effect;
                        }

                        for (attribute, attribute_damage) in over_time_effect.change_attributes.iter() {
                            let new_attribute_damage = match resists.resists.get(&get_resist_from_damage_type(&over_time_effect.effect_damage_type)) {
                                Some(v) => *attribute_damage - *attribute_damage * *v / 100,
                                None => *attribute_damage,
                            };
                            charactor::change_attribute_points(&mut attributes, attribute, new_attribute_damage, false);
                            damage_text.text_upper.push(DamageTextInformer::new(new_attribute_damage, None, false, Some(&over_time_effect.effect_damage_type)));
                        }
                    },
                    None => {},
                }
            }                   
        }

        for effect_type in effects_to_remove.iter() {
            effects.effects.remove(effect_type);
        }
    }
}

