use bevy::prelude::*;
use rand::Rng;

use crate::components::charactor_component::{
    AbilityComponent, ActionType, CharactorComponent, CompanionComponent, EffectComponent,
    ExtraStatsComponent, InventoryComponent, MonsterComponent, PlayerComponent, PositionComponent,
    ResistsComponent, SkillComponent,
};

use crate::resources::deploy::Deploy;
use crate::resources::deploy::charactor_deploy::EffectsDeploy;
use crate::resources::scene_data::charactor;

use super::effects::Effect;
use super::{abilities::Ability, skills::Skill, stats::ExtraStat, CharactorStatus};

pub fn player_attacking(
    mut player_query: Query<
        (
            &CharactorComponent,
            &mut SkillComponent,
            &AbilityComponent,
            &InventoryComponent,
            &PositionComponent,
        ),
        With<PlayerComponent>,
    >,
    mut monsters_query: Query<
        (
            &CharactorComponent,
            &mut ExtraStatsComponent,
            &ResistsComponent,
            &mut EffectComponent,
            &PositionComponent,
            &AbilityComponent,
        ),
        With<MonsterComponent>,
    >,
    deploy: Res<Deploy>,
) {
    let (player, mut player_skill, player_ability, player_inventory, player_position) =
        player_query.single_mut();

    if player.action != ActionType::Attack {
        return;
    };

    let player_target_id: usize = match player.target {
        Some(v) => v,
        _ => {
            println!("Player has no target, but status Attacking!");
            0
        }
    };

    let effects_deploy = &deploy.charactor_deploy.effects_deploy;

    for (
        monster,
        mut monster_extra_stats,
        monster_resist,
        mut monster_effect,
        monster_position,
        monster_ability,
    ) in monsters_query.iter_mut()
    {
        if monster.id == player_target_id {
            try_to_attack(
                player_position,
                monster_position,
                &mut player_skill,
                &player_ability,
                &player_inventory,
                &monster_resist,
                &mut monster_extra_stats,
                &mut monster_effect,
                monster_ability,
                effects_deploy
            );
            break;
        };
    }
}

pub fn companion_attacking(
    mut companion_query: Query<
        (
            &CharactorComponent,
            &SkillComponent,
            &mut ExtraStatsComponent,
            &ResistsComponent,
            &mut EffectComponent,
        ),
        With<CompanionComponent>,
    >,
    mut monsters_query: Query<
        (
            &CharactorComponent,
            &SkillComponent,
            &mut ExtraStatsComponent,
            &ResistsComponent,
            &mut EffectComponent,
        ),
        With<MonsterComponent>,
    >,
) {
    let (
        companion,
        companion_skill,
        mut companion_extra_stats,
        companion_resist,
        mut companion_effect,
    ) = companion_query.single_mut();

    let companion_target_id: usize = if companion.status == CharactorStatus::Attacking {
        match companion.target {
            Some(v) => v,
            _ => {
                println!("Companion has no target, but status Attacking!");
                0
            }
        }
    } else {
        0
    };
}
pub fn monster_attacking(
    mut monsters_query: Query<
        (
            &CharactorComponent,
            &SkillComponent,
            &mut ExtraStatsComponent,
            &ResistsComponent,
            &mut EffectComponent,
        ),
        With<MonsterComponent>,
    >,
    mut companion_query: Query<
        (
            &CharactorComponent,
            &SkillComponent,
            &mut ExtraStatsComponent,
            &ResistsComponent,
            &mut EffectComponent,
        ),
        With<CompanionComponent>,
    >,
    mut player_query: Query<
        (
            &CharactorComponent,
            &SkillComponent,
            &mut ExtraStatsComponent,
            &ResistsComponent,
            &mut EffectComponent,
        ),
        With<PlayerComponent>,
    >,
) {
}

fn try_to_attack(
    position: &PositionComponent,
    target_position: &PositionComponent,
    skill_component: &mut SkillComponent,
    ability_component: &AbilityComponent,
    inventory_component: &InventoryComponent,
    target_resist: &ResistsComponent,
    target_extra_stats: &mut ExtraStatsComponent,
    target_effect: &mut EffectComponent,
    target_ability: &AbilityComponent,
    effects_deploy: &EffectsDeploy,
) -> bool {
    //get attacking skill;
    let skill = match skill_component.skills.get_mut(&1) {
        Some(v) => v,
        _ => &mut Skill {
            ..Default::default()
        },
    };

    // check for default skill
    if skill.base_cooldown == 0 {
        println!("can't attacking target, because autoattack skill not found in skills storage");
        return false;
    };

    //check for position;
    let skill_range = skill.range;
    let diff_x = (position.position.x.abs() - target_position.position.x.abs()).abs(); // always positive value;
    let diff_y = (position.position.y.abs() - target_position.position.y.abs()).abs(); // always positive value;
    let diff = diff_x.max(diff_y);
    if skill_range as i32 >= diff {
        if skill.current_duration == 0.0 {
            //if all finem we attack;
            attack(
                skill,
                ability_component,
                inventory_component,
                target_resist,
                target_extra_stats,
                target_effect,
                target_ability,
                effects_deploy
            );
        } else {
            return false;
        }
    } else {
        return false;
    }

    return true;
}

fn attack(
    skill: &mut Skill,
    ability_component: &AbilityComponent,
    inventory_component: &InventoryComponent,
    target_resists: &ResistsComponent,
    target_extra_stats: &mut ExtraStatsComponent,
    target_effect: &mut EffectComponent,
    target_ability: &AbilityComponent,
    effects_deploy: &EffectsDeploy,
) {
    // value to start cooldown of attack;
    let mut rng = rand::thread_rng();
    skill.current_duration = skill.current_cooldown;
    //check for melee or ranged+magic attack;
    if skill.projectiles > 0 {
        //TODO: create projectile;
    } else {
        // let check for aacuracy
        let accuracy = match ability_component.ability.get(&Ability::Accuracy) {
            Some(v) => *v,
            _ => {
                println!("Can't get Accuracy, use 0.0 instead, so 100% chance to miss");
                0.0
            }
        };

        if accuracy <= 0.0 {
            //TODO: take this value to target, interface ( sprite ) need to text it to user; "MISS";
            return;
        } else if accuracy >= 100.0 {
        } else {
            let random_accuracy_number: u8 = rng.gen_range(0..=99);
            if accuracy <= random_accuracy_number as f32 {
                //TODO: take this value to target, interface ( sprite ) need to text it to user; "MISS";
                return;
            }
        }

        // if we here, let chech the evasion of tagert;
        let target_evasion = match target_ability.ability.get(&Ability::Evasion) {
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
        let block_amount: f32 = match target_ability.ability.get(&Ability::BlockAmount) {
            Some(v) => *v,
            _ => {
                println!("Target has no block amount, i use 0 instead");
                0.0
            }
        };

        let block_percent: f32 = match target_ability.ability.get(&Ability::BlockChance) {
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
                    *value = damage_from_weapon - (damage_from_weapon * resist_damage_from_target / 100) ;
                };
            }

            

            //check for ednless effect or temporary
            if effect.duration == 0.0 {
                //endless
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

                //calculate new effect duration by target resist;
                let effect_duration = effect.duration * target_effect_resist as f32 / 100.0;
                effect.duration -= effect_duration;

                target_effect.temporary_effect.entry(effect_type.clone())
                .and_modify(|x| x.duration += effect.duration)
                .or_insert(effect);

                //TODO: change damge from min to max in 2 values;
                /*
                for (_, value) in target_effect.temporary_effect.get_mut(&effect_type).unwrap().change_extra_stat.iter_mut() {

                }
                */
            }
            
        }
    }
}
