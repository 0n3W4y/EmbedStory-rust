use bevy::prelude::*;

use crate::components::{StatsComponent, ResistsComponent, AttributesComponent, TakenDamageComponent, TakenDamage};
use crate::components::charactor_component::{
    AbilityComponent, CharactorComponent, EffectComponent,
    SkillComponent, InventoryComponent,
};
use crate::resources::deploy::Deploy;
use crate::resources::scene_data::Damage;
use crate::resources::scene_data::charactor::{self, skills};
use super::effects::EffectType;
use super::{CharactorStatus, change_stat_points, StuffWearSlot};


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
            &mut TakenDamageComponent,
        ),
        With<CharactorComponent>>,
    deploy: Res<Deploy>,
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
        mut damage_taken,

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
                            charactor::change_attribute_points(&mut attributes, &Damage::Health, *attribute_damage, true);
                        }

                        for (resist, resists_damage) in buff_debuff_effect.change_resist.iter() {
                            charactor::change_resist(&mut resists.resists, resist, *resists_damage);
                        }

                        for (ability, ability_damage) in buff_debuff_effect.change_ability .iter(){
                            charactor::change_ability(&mut abilities.ability, &ability, *ability_damage);
                        }

                        skills::setup_base_skill(
                            &deploy,
                            &mut skills.base_skill,
                            &abilities.ability, 
                            inventory.stuff_wear.get(&StuffWearSlot::PrimaryHand).unwrap()
                            );

                    },
                    None => {},
                }

                match effect.over_time_effect {
                    Some(over_time_effect) => {
                        let mut damage: TakenDamage = Default::default();
                        damage.damage.insert(over_time_effect.effect_damage_type.clone(), over_time_effect.effect_damage_value);
                        damage_taken.damage.push(damage);
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
                            charactor::change_attribute_points(&mut attributes, &Damage::Health, -(*attribute_damage), true);
                        }

                        for (resist, resists_damage) in buff_debuff_effect.change_resist.iter() {
                            charactor::change_resist(&mut resists.resists, resist, -(*resists_damage));
                        }

                        for (ability, ability_damage) in buff_debuff_effect.change_ability .iter(){
                            charactor::change_ability(&mut abilities.ability, &ability, -(*ability_damage));
                        }

                        skills::setup_base_skill(
                            &deploy,
                            &mut skills.base_skill,
                            &abilities.ability, 
                            inventory.stuff_wear.get(&StuffWearSlot::PrimaryHand).unwrap()
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

                        let mut damage: TakenDamage = Default::default();
                        damage.damage.insert(over_time_effect.effect_damage_type.clone(), -over_time_effect.effect_damage_value);
                        damage_taken.damage.push(damage);
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

