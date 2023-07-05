use bevy::prelude::*;

use super::effects::EffectTimeType;
use crate::components::charactor_component::{
    AbilityComponent, CharactorComponent, EffectComponent, ExtraStatsComponent, ResistsComponent,
    SkillComponent, StatsComponent, InventoryComponent,
};
use crate::resources::scene_data::charactor;

pub fn update_effects(
    mut charactors_query: Query<
        (
            &mut EffectComponent,
            &mut StatsComponent,
            &mut ExtraStatsComponent,
            &mut ResistsComponent,
            &mut AbilityComponent,
            &mut SkillComponent,
            &mut InventoryComponent
        ),
        With<CharactorComponent>,
    >,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();
    println!("{delta}");
    for (
        mut effects, 
        mut stats, 
        mut extra_stats, 
        mut resists, 
        mut abilities, 
        mut skills,
        mut inventory
    ) in charactors_query.iter_mut() {
        for (_, effect) in effects.temporary_effect.iter_mut() {
            if (effect.current_duration - effect.trigger_time as f32 * effect.triggered as f32)
                >= effect.trigger_time as f32
            {
                effect.triggered += 1;
                //trigger pereodic effect;

                if effect.change_stat_time_effect == EffectTimeType::Pereodic {
                    for (stat, stat_damage) in effect.change_stat {
                        charactor::change_stat(
                            &mut stats.stats,
                            &mut stats.stats_cache,
                            &mut extra_stats.extra_stats_regen,
                            &mut extra_stats.extra_stats,
                            &mut extra_stats.extra_stats_cache,
                            &mut resists.effect_resists,
                            &mut resists.effect_resists_cache,
                            resists.effect_resists_min_value,
                            resists.effect_resists_max_value,
                            &mut resists.damage_resists,
                            &mut resists.damage_resists_cache,
                            resists.damage_resists_max_value,
                            resists.damage_resists_min_value,
                            &mut abilities.ability,
                            &stat,
                            stat_damage,
                            stats.stats_min_value,
                        );
                    }
                    //TODO: Update Weapon, skills and trinket by ability storage;
                };

                if effect.change_extra_stat_time_effect == EffectTimeType::Pereodic {
                    if effect.change_extra_stat_is_damage {
                        for (extra_stat, damage_value) in effect.change_extra_stat {
                            charactor::change_extra_stat_current(&mut extra_stats.extra_stats, &mut extra_stats.extra_stats_cache, &extra_stat, damage_value);
                        };
                    }else {
                        for (extra_stat, damage_value) in effect.change_extra_stat {
                            charactor::change_extra_stat_cache(&mut extra_stats.extra_stats, &mut extra_stats.extra_stats_cache, &extra_stat, damage_value);
                        };
                    }
                };

                if effect.change_damage_resist_time_effect == EffectTimeType::Pereodic {
                    for (damage_resist, damage_resists_value) in effect.change_damage_resist {
                        charactor::change_damage_resist(
                            &mut resists.damage_resists,
                            &mut resists.damage_resists_cache,
                            &damage_resist,
                            damage_resists_value,
                            resists.damage_resists_max_value,
                            resists.damage_resists_min_value,
                        );
                    }
                };

                if effect.change_ability_time_effect == EffectTimeType::Pereodic {
                    for (ability, ability_value) in effect.change_ability {
                        charactor::change_ability(&mut abilities.ability, &ability, ability_value as f32);
                    }
                    //update weapon, trinket, skill by ability storage;
                };
            } else if effect.current_duration == 0.0 {
               //do changes no matter instant or pereodic, just need to know - it start of effect do

               //change stat;
                for (stat, stat_damage) in effect.change_stat {
                    charactor::change_stat(
                        &mut stats.stats,
                        &mut stats.stats_cache,
                        &mut extra_stats.extra_stats_regen,
                        &mut extra_stats.extra_stats,
                        &mut extra_stats.extra_stats_cache,
                        &mut resists.effect_resists,
                        &mut resists.effect_resists_cache,
                        resists.effect_resists_min_value,
                        resists.effect_resists_max_value,
                        &mut resists.damage_resists,
                        &mut resists.damage_resists_cache,
                        resists.damage_resists_max_value,
                        resists.damage_resists_min_value,
                        &mut abilities.ability,
                        &stat,
                        stat_damage,
                        stats.stats_min_value,
                    );
                }

                //change extra_stat
                if effect.change_extra_stat_is_damage {
                    for (extra_stat, damage_value) in effect.change_extra_stat {
                        charactor::change_extra_stat_current(&mut extra_stats.extra_stats, &mut extra_stats.extra_stats_cache, &extra_stat, damage_value);
                    };
                }else {
                    for (extra_stat, damage_value) in effect.change_extra_stat {
                        charactor::change_extra_stat_cache(&mut extra_stats.extra_stats, &mut extra_stats.extra_stats_cache, &extra_stat, damage_value);
                    };
                }

                //change ... damage resists;
                for (damage_resist, damage_resists_value) in effect.change_damage_resist {
                    charactor::change_damage_resist(
                        &mut resists.damage_resists,
                        &mut resists.damage_resists_cache,
                        &damage_resist,
                        damage_resists_value,
                        resists.damage_resists_max_value,
                        resists.damage_resists_min_value,
                    );
                }

                //change abilities;
                for (ability, ability_value) in effect.change_ability {
                    charactor::change_ability(&mut abilities.ability, &ability, ability_value as f32);
                }

                //TODO: update weapon. trinket and skills by ability storage;
            }

            //add duration;
            effect.current_duration += delta;

            //check for effects end;
            if effect.current_duration >= effect.duration as f32 {
                // remove effect and remove passsive skill;
                if effect.change_stat_revert_changes {
                    for (stat, stat_damage) in effect.change_stat {
                        charactor::change_stat(
                            &mut stats.stats,
                            &mut stats.stats_cache,
                            &mut extra_stats.extra_stats_regen,
                            &mut extra_stats.extra_stats,
                            &mut extra_stats.extra_stats_cache,
                            &mut resists.effect_resists,
                            &mut resists.effect_resists_cache,
                            resists.effect_resists_min_value,
                            resists.effect_resists_max_value,
                            &mut resists.damage_resists,
                            &mut resists.damage_resists_cache,
                            resists.damage_resists_max_value,
                            resists.damage_resists_min_value,
                            &mut abilities.ability,
                            &stat,
                            -stat_damage, // WARNING use "-" to revert changes if it be "+" so we have "-", and if it "-" so we "+" stat;
                            stats.stats_min_value,
                        );
                    }
                    //todo: udapte weapon. trinket and skill

                }
            
                if effect.change_extra_stat_revert_changes {
                    if effect.change_extra_stat_is_damage {
                        for (extra_stat, damage_value) in effect.change_extra_stat {
                            charactor::change_extra_stat_current(&mut extra_stats.extra_stats, &mut extra_stats.extra_stats_cache, &extra_stat, damage_value);
                        };
                    } else {
                        for (extra_stat, damage_value) in effect.change_extra_stat {
                            charactor::change_extra_stat_cache(&mut extra_stats.extra_stats, &mut extra_stats.extra_stats_cache, &extra_stat, damage_value);
                        };
                    }
                }
            
                if effect.change_damage_resist_revert_changes {
                    for (damage_resist, damage_resists_value) in effect.change_damage_resist {
                        charactor::change_damage_resist(
                            &mut resists.damage_resists,
                            &mut resists.damage_resists_cache,
                            &damage_resist,
                            -damage_resists_value, // WARNING use "-" to revert changes if it be "+" so we have "-", and if it "-" so we "+" stat;
                            resists.damage_resists_max_value,
                            resists.damage_resists_min_value,
                        );
                    }
                }
            
                if effect.change_ability_revert_changes {
                    for (ability, ability_value) in effect.change_ability {
                        charactor::change_ability(&mut abilities.ability, &ability, -ability_value);
                        // WARNING use "-" to revert changes if it be "+" so we have "-", and if it "-" so we "+" stat;
                    }
                    //todo: udapte weapon. trinket and skill
                }            

                // remove passive skill if it present;
                match effect.extra_skill {
                    Some(v) => {
                        skills.passive_skills.remove(&v);
                    }
                    _ => {}
                }
                // remove effect from hashmap of effects;
                effects.temporary_effect.remove(&effect.effect_type);
            };
        }
    }
}
