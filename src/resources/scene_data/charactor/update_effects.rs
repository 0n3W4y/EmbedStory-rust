use bevy::prelude::*;

use crate::components::{StatsComponent, ResistsComponent, AttributesComponent};
use crate::components::charactor_component::{
    AbilityComponent, CharactorComponent, EffectComponent,
    SkillComponent, InventoryComponent,
};
use crate::resources::scene_data::charactor::{self, skills};
use super::effects::EffectType;
use super::{CharactorStatus, SkillSlot, change_stat_points};

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
            & InventoryComponent,
        ),
        With<CharactorComponent>,
    >,
) {
    //this function running with criteria triggered by 0.1 sec;
    let delta_time: f32 = 0.1;
    for (
        charactor_component,
        mut effects, 
        mut stats, 
        mut attributes,
        mut resists, 
        mut abilities, 
        mut skills,
        inventory,
    ) in charactors_query.iter_mut() {
        if charactor_component.status == CharactorStatus::Dead {                            //check for dead
            continue;                                                                       //do nothing with dead charactors;
        };

        let mut effects_to_remove:Vec<EffectType> = vec![];                                     //create vec of effects for deleting, which one ends at this moment;

        for (effect_type, effect) in effects.effects.iter_mut() {                  //update  effects;
            if effect.current_time_duration == 0.0 {                                                             //first run;
                for (stat, stat_damage) in effect.change_stat.iter() {
                    change_stat_points(                    
                        &mut stats,
                        &mut resists.resists,
                        &mut abilities.ability,
                        &mut attributes,
                        stat,
                        *stat_damage,
                    );
                }

                for (attribute, attribute_damage) in effect.change_attribute.iter() {
                    charactor::change_attribute_points(&mut attributes, attribute, *attribute_damage, false);
                }

                for (attribute, attribute_damage) in effect.change_attribute_cache.iter() {
                    charactor::change_attribute_points(&mut attributes, attribute, *attribute_damage, true);
                }
                
                for (resist, resists_damage) in effect.change_resist.iter() {                   //change resists;
                    charactor::change_resist(&mut resists.resists, resist, *resists_damage);
                }

                for (ability, ability_damage) in effect.change_ability .iter(){                 //change abilities;
                    charactor::change_ability(&mut abilities.ability, &ability, *ability_damage);
                }

                
                skills::update_basic_skill_by_changes_in_ability(                        //update base skill by changes in abilities and stats;
                    skills.skills.get_mut(&SkillSlot::Base),
                     &abilities.ability, 
                     &inventory.stuff_wear
                    );
                effect.current_time_duration += delta_time; 
            } else if effect.current_time_duration > effect.effect_duration || effect.effect_duration < 0.0 {                                 //effect is end; revert changes and remove effect
                for (stat, stat_damage) in effect.change_stat.iter() {
                    change_stat_points(
                        &mut stats,
                        &mut resists.resists,
                        &mut abilities.ability,
                        &mut attributes,
                        stat,
                        -stat_damage,                                                                           // WARNING use "-" to revert changes if it be "+" so we have "-", and if it "-" so we "+" stat;
                    );
                }

                for (attribute, attribute_damage) in effect.change_attribute.iter() {
                    charactor::change_attribute_points(&mut attributes, attribute, -attribute_damage, false);
                }

                for (attribute, attribute_damage) in effect.change_attribute_cache.iter() {
                    charactor::change_attribute_points(&mut attributes, attribute, -attribute_damage, true);
                }

                for (effect_resist, resist_damage) in effect.change_resist.iter() {
                    charactor::change_resist(&mut resists.resists, effect_resist, -resist_damage);  // WARNING use "-" to revert changes if it be "+" so we have "-", and if it "-" so we "+" stat;
                }
            
                for (ability, ability_damage) in effect.change_ability.iter() {
                    charactor::change_ability(&mut abilities.ability, ability, -ability_damage);    // WARNING use "-" to revert changes if it be "+" so we have "-", and if it "-" so we "+" stat;
                }

                skills::update_basic_skill_by_changes_in_ability(skills.skills.get_mut(&SkillSlot::Base), &abilities.ability, &inventory.stuff_wear);  

                effects_to_remove.push(effect_type.clone());                                                            //fill vec for deleting effects ended by duration;
            } else {
                effect.current_time_duration += delta_time;                                                                           //add time to effect duration;
                //remove this;
                //println!("From effect update. current duration: {:?}, delta: {:?}", effect.current_duration, delta);
            }                
        }

        for effect_type in effects_to_remove.iter() {
            effects.effects.remove(effect_type);
        }
        effects_to_remove.clear();
    }
}

