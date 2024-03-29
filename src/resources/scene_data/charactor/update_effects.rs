use bevy::prelude::*;

use crate::components::{StatsComponent,  TakenDamageComponent, TakenDamage};
use crate::components::charactor_component::{
    CharactorComponent,
    InventoryComponent, SkillAndEffectComponent,
};
use crate::resources::deploy::Deploy;
use crate::resources::scene_data::Damage;
use crate::resources::scene_data::charactor;
use super::effects::{EffectType, EffectStatus};
use super::skills::setup_base_skill;
use super::{CharactorStatus, change_stat_points, StuffWearSlot, do_stat_dependences};


pub fn update_effects(
    mut charactors_query: Query<
        (
            &CharactorComponent,
            &mut StatsComponent,
            &mut SkillAndEffectComponent,
            &InventoryComponent,
            &mut TakenDamageComponent,
        ),
        With<CharactorComponent>>,
    deploy: Res<Deploy>,
) {
    let delta_time = 0.1;
    for (
        charactor_component,
        mut stats, 
        mut skills_and_effects,
        inventory,
        mut damage_taken,

    ) in charactors_query.iter_mut() {
        if charactor_component.status == CharactorStatus::Dead {                            //check for dead
            continue;                                                                       //do nothing with dead charactors;
        };

        let mut effects_to_remove:Vec<EffectType> = vec![];                                     //create vec of effects for deleting, which one ends at this moment;
        let mut effect_status_to_remove: Vec<EffectStatus> = vec![];

        for (_, effect) in skills_and_effects.effects.iter_mut() {                                     //update  effects;
            if effect.time_duration == 0.0 {
                for buff_debuff_effect in effect.buff_debuff_effect.iter_mut() {
                    for (stat, stat_damage) in buff_debuff_effect.change_stat.iter() {
                        if let Some((new_value, old_value)) = change_stat_points(&mut stats,  stat, *stat_damage) {
                            do_stat_dependences(&mut stats, stat, new_value, old_value);
                        }
                    }
                    for (_, attribute_damage) in buff_debuff_effect.change_attribute_cache.iter() {
                        charactor::change_attribute_points(&mut stats, &Damage::Health, *attribute_damage, true);
                    }

                    for (resist, resists_damage) in buff_debuff_effect.change_resist.iter() {
                        charactor::change_resist(&mut stats, resist, *resists_damage);
                    }

                    for (ability, ability_damage) in buff_debuff_effect.change_ability .iter(){
                        charactor::change_ability(&mut stats, &ability, *ability_damage);
                    }
                }

                for over_time_effect in effect.over_time_effect.iter_mut() {
                    let mut damage: TakenDamage = Default::default();
                        damage.damage.insert(over_time_effect.effect_damage_type.clone(), over_time_effect.effect_damage_value);
                        damage_taken.damage.push(damage);
                }
            } else if effect.time_duration >= effect.effect_lifetime {
                for buff_debuff_effect in effect.buff_debuff_effect.iter_mut() {
                    for (stat, stat_damage) in buff_debuff_effect.change_stat.iter() {
                        if let Some((old_value, new_value)) = change_stat_points(&mut stats,  stat, -(*stat_damage)) {
                            do_stat_dependences(&mut stats, stat, new_value, old_value);
                        }
                    }

                    for (_, attribute_damage) in buff_debuff_effect.change_attribute_cache.iter() {
                        charactor::change_attribute_points(&mut stats, &Damage::Health, -(*attribute_damage), true);
                    }

                    for (resist, resists_damage) in buff_debuff_effect.change_resist.iter() {
                        charactor::change_resist(&mut stats, resist, -(*resists_damage));
                    }

                    for (ability, ability_damage) in buff_debuff_effect.change_ability .iter(){
                        charactor::change_ability(&mut stats, &ability, -(*ability_damage));
                    }
                }

                for effect_status in effect.effect_status.iter() {
                    effect_status_to_remove.push(effect_status.clone());
                }
                effects_to_remove.push(effect.effect_type.clone());
            } else {
                effect.time_duration += delta_time;
                for over_time_effect in effect.over_time_effect.iter_mut() {
                    over_time_effect.time_duration += delta_time;
                        if over_time_effect.time_duration < over_time_effect.trigger_time_effect {
                            continue;
                        } else {
                            over_time_effect.time_duration -= over_time_effect.trigger_time_effect;
                        }

                        let mut damage: TakenDamage = Default::default();
                        damage.damage.insert(over_time_effect.effect_damage_type.clone(), -over_time_effect.effect_damage_value);
                        damage_taken.damage.push(damage);
                }
            }                   
        }

        setup_base_skill(
            &deploy,
            &mut skills_and_effects.base_skill,
            &stats, 
            inventory.stuff_wear.get(&StuffWearSlot::PrimaryHand).unwrap()
        );

        for effect_type in effects_to_remove.iter() {
            skills_and_effects.effects.remove(effect_type);
        }

        for effect_status in effect_status_to_remove.iter() {
            if let Some(index) = skills_and_effects.effect_status.iter().position(|x| x == effect_status) {
                skills_and_effects.effect_status.remove(index);
            }
        }
    }
}

