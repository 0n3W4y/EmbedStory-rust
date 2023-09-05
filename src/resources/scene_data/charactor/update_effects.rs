use bevy::prelude::*;

use crate::components::charactor_component::{
    AbilityComponent, CharactorComponent, EffectComponent, ExtraStatsComponent, ResistsComponent,
    SkillComponent, StatsComponent, InventoryComponent,
};
use crate::resources::scene_data::charactor::{self, skills};

use super::{CharactorStatus, SkillSlot};

pub fn update_effects(
    mut charactors_query: Query<
        (
            &CharactorComponent,
            &mut EffectComponent,
            &mut StatsComponent,
            &mut ExtraStatsComponent,
            &mut ResistsComponent,
            &mut AbilityComponent,
            &mut SkillComponent,
            & InventoryComponent,
        ),
        With<CharactorComponent>,
    >,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();
    for (
        charactor_component,
        mut effects, 
        mut stats, 
        mut extra_stats, 
        mut resists, 
        mut abilities, 
        mut skills,
        inventory,
    ) in charactors_query.iter_mut() {
        //check for dead
        if charactor_component.status == CharactorStatus::Dead {
            continue;
        };

        //update temporary effects;
        for (_, effect) in effects.temporary_effect.iter_mut() {
            //first run;
            if effect.current_duration == 0.0 {
                for (stat, stat_damage) in effect.change_stat {
                    charactor::change_stat(
                        &mut stats.stats,
                        &mut stats.stats_cache,
                        &mut extra_stats.extra_stats,
                        &mut extra_stats.extra_stats_cache,
                        &mut resists.effect_resists,
                        &mut resists.damage_resists,
                        &mut abilities.ability,
                        &stat,
                        stat_damage,
                        &effect.change_stat_damage_type,
                        stats.stats_min_value,
                    );
                }

                //change extra_stat_cache;
                //all effects change cache like buff or debuff health;
                for (extra_stat, damage_value) in effect.change_extra_stat {
                    charactor::change_extra_stat_cache(
                        &mut extra_stats.extra_stats, 
                        &mut extra_stats.extra_stats_cache,
                        &extra_stat, 
                        damage_value,
                        &effect.change_extra_stat_damage_type,
                    );
                };

                //change damage resists;
                for (damage_resist, damage_resists_value) in effect.change_damage_resist {
                    charactor::change_damage_resist(
                        &mut resists.damage_resists,
                        &damage_resist,
                        damage_resists_value,
                    );
                }

                //change effects resists;
                for (effect_resist, effect_resist_value) in effect.change_effect_resist {
                    charactor::change_effect_resist(
                        &mut resists.effect_resists,
                        &effect_resist,
                        effect_resist_value,
                    );
                }

                //change abilities;
                for (ability, ability_value) in effect.change_ability {
                    charactor::change_ability(&mut abilities.ability, &ability, ability_value as i16);
                }

                //update base skill by changes in abilities and stats;
                skills::update_basic_skill_by_changes_in_ability(skills.skills.get_mut(&SkillSlot::Base), &abilities.ability, &inventory.stuff_wear);

            } else if effect.current_duration >= effect.duration {
                // effect is end; revert changes and remove effect
                for (stat, stat_damage) in effect.change_stat {
                    charactor::change_stat(
                        &mut stats.stats,
                        &mut stats.stats_cache,
                        &mut extra_stats.extra_stats,
                        &mut extra_stats.extra_stats_cache,
                        &mut resists.effect_resists,
                        &mut resists.damage_resists,
                        &mut abilities.ability,
                        &stat,
                        -stat_damage, // WARNING use "-" to revert changes if it be "+" so we have "-", and if it "-" so we "+" stat;
                        &effect.change_stat_damage_type,
                        stats.stats_min_value,
                    );
                }
            

                for (extra_stat, damage_value) in effect.change_extra_stat {
                    charactor::change_extra_stat_current(
                        &mut extra_stats.extra_stats, 
                        &mut extra_stats.extra_stats_cache, 
                        &extra_stat, 
                        damage_value,
                        &effect.change_extra_stat_damage_type,
                    );
                };

            
                for (damage_resist, damage_resists_value) in effect.change_damage_resist {
                    charactor::change_damage_resist(
                        &mut resists.damage_resists,
                        &damage_resist,
                        -damage_resists_value, // WARNING use "-" to revert changes if it be "+" so we have "-", and if it "-" so we "+" stat;
                    );
                }
            
                for (effect_resist, effect_resist_value) in effect.change_effect_resist {
                    charactor::change_effect_resist(
                        &mut resists.effect_resists,
                        &effect_resist,
                        -effect_resist_value, // WARNING use "-" to revert changes if it be "+" so we have "-", and if it "-" so we "+" stat;
                    );
                }
            
                for (ability, ability_value) in effect.change_ability {
                    charactor::change_ability(&mut abilities.ability, &ability, -ability_value);
                    // WARNING use "-" to revert changes if it be "+" so we have "-", and if it "-" so we "+" stat;
                }

                skills::update_basic_skill_by_changes_in_ability(skills.skills.get_mut(&SkillSlot::Base), &abilities.ability, &inventory.stuff_wear);  

                effects.temporary_effect.remove(&effect.effect_type);
            } else {
                //add time to effect duration;
                effect.current_duration += delta;
                //remove this;
                println!("From effect update. current duration: {:?}, delta: {:?}", effect.current_duration, delta);
            }                
        }

        //update endless effect;
        for(_, endless_effect) in effects.endless_effect.iter_mut(){
            //check for remove endless_effect
            //we remove endless effect if duration changes to negative value;
            if endless_effect.current_duration < 0.0 {
                //remove endless effect;
                for (stat, stat_damage) in endless_effect.change_stat {
                    charactor::change_stat(
                        &mut stats.stats,
                        &mut stats.stats_cache,
                        &mut extra_stats.extra_stats,
                        &mut extra_stats.extra_stats_cache,
                        &mut resists.effect_resists,
                        &mut resists.damage_resists,
                        &mut abilities.ability,
                        &stat,
                        -stat_damage, // WARNING use "-" to revert changes if it be "+" so we have "-", and if it "-" so we "+" stat;
                        &endless_effect.change_stat_damage_type,
                        stats.stats_min_value,
                    );
                }
            

                for (extra_stat, damage_value) in endless_effect.change_extra_stat {
                    charactor::change_extra_stat_current(
                        &mut extra_stats.extra_stats, 
                        &mut extra_stats.extra_stats_cache, 
                        &extra_stat, 
                        damage_value,
                        &endless_effect.change_extra_stat_damage_type,
                    );
                };

            
                for (damage_resist, damage_resists_value) in endless_effect.change_damage_resist {
                    charactor::change_damage_resist(
                        &mut resists.damage_resists,
                        &damage_resist,
                        -damage_resists_value, // WARNING use "-" to revert changes if it be "+" so we have "-", and if it "-" so we "+" stat;
                    );
                }
            
                for (effect_resist, effect_resist_value) in endless_effect.change_effect_resist {
                    charactor::change_effect_resist(
                        &mut resists.effect_resists,
                        &effect_resist,
                        -effect_resist_value, // WARNING use "-" to revert changes if it be "+" so we have "-", and if it "-" so we "+" stat;
                    );
                }
            
                for (ability, ability_value) in endless_effect.change_ability {
                    charactor::change_ability(&mut abilities.ability, &ability, -ability_value);
                    // WARNING use "-" to revert changes if it be "+" so we have "-", and if it "-" so we "+" stat;
                }

                skills::update_basic_skill_by_changes_in_ability(skills.skills.get_mut(&SkillSlot::Base), &abilities.ability, &inventory.stuff_wear);

                effects.endless_effect.remove(&endless_effect.effect_type);

            //first run
            } else if endless_effect.current_duration == 0.0 {
                for (stat, stat_damage) in endless_effect.change_stat {
                    charactor::change_stat(
                        &mut stats.stats,
                        &mut stats.stats_cache,
                        &mut extra_stats.extra_stats,
                        &mut extra_stats.extra_stats_cache,
                        &mut resists.effect_resists,
                        &mut resists.damage_resists,
                        &mut abilities.ability,
                        &stat,
                        stat_damage,
                        &endless_effect.change_stat_damage_type,
                        stats.stats_min_value,
                    );
                }

                //change extra_stat_cache;
                //all effects change cache like buff or debuff health;
                for (extra_stat, damage_value) in endless_effect.change_extra_stat {
                    charactor::change_extra_stat_cache(
                        &mut extra_stats.extra_stats, 
                        &mut extra_stats.extra_stats_cache,
                        &extra_stat, 
                        damage_value,
                        &endless_effect.change_extra_stat_damage_type,
                    );
                };

                //change damage resists;
                for (damage_resist, damage_resists_value) in endless_effect.change_damage_resist {
                    charactor::change_damage_resist(
                        &mut resists.damage_resists,
                        &damage_resist,
                        damage_resists_value,
                    );
                }

                //change effects resists;
                for (effect_resist, effect_resist_value) in endless_effect.change_effect_resist {
                    charactor::change_effect_resist(
                        &mut resists.effect_resists,
                        &effect_resist,
                        effect_resist_value,
                    );
                }

                //change abilities;
                for (ability, ability_value) in endless_effect.change_ability {
                    charactor::change_ability(&mut abilities.ability, &ability, ability_value as i16);
                }

                skills::update_basic_skill_by_changes_in_ability(skills.skills.get_mut(&SkillSlot::Base), &abilities.ability, &inventory.stuff_wear);
            } else {
                endless_effect.current_duration += delta;
            }            
        }
    }
}

