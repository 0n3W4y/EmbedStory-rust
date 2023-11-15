use bevy::prelude::*;
use rand::Rng;

use crate::components::{PositionComponent, IdenteficationComponent};
use crate::components::charactor_component::{
    AbilityComponent, ActionType, CharactorComponent, EffectComponent,
    ResistsComponent, SkillComponent, CharactorTargetComponent, CharactorTextComponent, StatsComponent,
};

use crate::components::projectile_component::Projectile;
use crate::materials::material_manager::MaterialManager;
use crate::resources::deploy::Deploy;
use crate::resources::scene_data::charactor::{self, SkillSlot};
use crate::resources::scene_data::projectiles::update_projectile::create_projectile;
use crate::resources::scene_data::stuff::damage_type::DamageType;
use crate::resources::scene_data::stuff::resists_types;

use super::abilities;
use super::damage_text_informer::DamageTextInformer;
use super::effects::Effect;
use super::stats::Stat;
use super::{abilities::AbilityType, skills::Skill, CharactorStatus};

pub fn attacking_from_basic_skill(
    mut commands: Commands,
    mut charactor_query: Query<(
        &IdenteficationComponent,
        &CharactorComponent,
        &PositionComponent,
        &mut SkillComponent,
        &CharactorTargetComponent,
        &AbilityComponent,
    )>,

    mut target_query: Query<(
        &IdenteficationComponent,
        &CharactorComponent,
        &StatsComponent,
        &ResistsComponent,
        &mut EffectComponent,
        &PositionComponent,
        &AbilityComponent,
        &mut CharactorTextComponent,
        &mut SkillComponent,
    )>,

    deploy: Res<Deploy>,
    materail_manager: Res<MaterialManager>,
) {
    for (
        identification_component,
        charactor, 
        charactor_position,
        mut charactor_skill, 
        charactor_target, 
        charactor_ability, 
    ) in charactor_query.iter_mut() {
        //check for attack
        if charactor.status != CharactorStatus::CanAttack {
            continue;
        }

        //let's attack or create projectile to attack;

        let target_id = match charactor_target.target {
            Some(v) => v,
            None => {
                println!("Can not attack, because charactor '{:?}, {:?}, {:?}' have Attack action, but doesnt have a target", charactor.charactor_type, charactor.gender_type, charactor.race_type);
                continue;
            }
        };

        //safe, we r already check for skill previosly;
        let skill = charactor_skill.skills.get_mut(&SkillSlot::Base).unwrap();

        for (
            identification_target_component,
            target_component, 
            mut target_stats,
            target_resists, 
            mut target_effects, 
            target_position, 
            target_abilities,
            mut target_text_component,
            mut target_skills,
        ) in target_query.iter_mut() {
            if target_id == identification_target_component.id {
                attack(
                    commands,
                    skill,
                    charactor_ability,
                    charactor_position,
                    target_position,
                    &mut target_text_component,
                    &mut target_stats,
                    target_resists, 
                    &mut target_effects, 
                    target_abilities, 
                    &mut target_skills,
                    &deploy,
                    &materail_manager,
                );
                break;
            }
        }
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
        &IdenteficationComponent,
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

        if charactor.status != CharactorStatus::TryAttack {
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
                    target_identification,
                    target,
                    target_position,
                ) in target_query.iter_mut() {
                    //check for target
                    if target_identification.id == target_id {
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
    mut commands: Commands,
    skill: &mut Skill,
    charactor_ability: &AbilityComponent,
    charactor_position: &PositionComponent,
    target_position: &PositionComponent,
    target_text_component: &mut CharactorTextComponent,
    target_stats: &mut StatsComponent,
    target_resists: &ResistsComponent,
    target_effects: &mut EffectComponent,
    target_abilities: &AbilityComponent,
    target_skills: &mut SkillComponent,
    deploy: &Deploy,
    material_manager: &MaterialManager,
) {
    let effects_deploy = &deploy.charactor_deploy.effects_deploy;
    let skills_deploy = &deploy.charactor_deploy.skills_deploy;
    let mut rng = rand::thread_rng();

    skill.on_cooldown = true;                                                                    //set skill on cooldown;

    let accuracy = match charactor_ability.ability.get(&AbilityType::Accuracy) {            //get accuracy
        Some(v) => *v,
        _ => {
            println!("Can't get Accuracy, use 0 instead, so 100% chance to miss");
            0
        }
    };

    let random_accuracy_number: u8 = rng.gen_range(0..=99);
    let missed: bool = if accuracy <= 0 || accuracy < random_accuracy_number as i16 {
        true
    } else {
        false
    };

    if skill.projectiles > 0 {
        let mut projectile_component = Projectile {             //create default projectile component;
            projectile_type: skill.projectile_type.clone(),
            starting_position: charactor_position.position.clone(),
            ..Default::default()
        };

        if missed {
            projectile_component.is_missed = true;
            create_projectile(
                &mut commands,
                material_manager,
                projectile_component,
                target_position.position.clone(),
                skill.projectiles,
                &skill.skill_direction
            );
            return;                                                                         //return, because attack is missed;
        }
        
        projectile_component.damage = skill.damage.clone();                                 //clone damage values;

        for (effect_type, trigger_chance) in skill.effect.iter() {          //check for triggered effects;
            let trigger_chance_random_number: u8 = rng.gen_range(0..=99);
            if *trigger_chance < trigger_chance_random_number {
                continue;                                                                   //not triggered;
            }
            projectile_component.effects.push(effect_type.clone());                             //store triggered effects to projectile;
        }

        for (skill_type, trigger_chance) in skill.extra_skill.iter() {
            let trigger_chance_random_number: u8 = rng.gen_range(0..=99);
            if *trigger_chance < trigger_chance_random_number {
                continue;                                                                   //not triggered;
            }

            let skill_config = deploy.charactor_deploy.skills_deploy.get_skill_deploy(skill_type);
            let mut skill = Skill::new(skill_config);

            let critical_hit_random_number = rng.gen_range(0..=99);
            let critical_hit_multiplier = if skill.crit_chance > critical_hit_random_number {           //calculating critical multiplier;
                skill.crit_multiplier
            } else {
                100
            };

            for (damage_type, damage) in skill.damage.iter_mut() {
                let ability_type = abilities::get_ability_type_from_damage_type(damage_type);
                let damage_multiplier = match charactor_ability.ability.get(&ability_type) {
                    Some(v) => *v,
                    None => 0,
                };
                let multiplier_damage = *damage * damage_multiplier / 100;
                let total_damage = multiplier_damage * critical_hit_multiplier;
                *damage = total_damage;
            }

            projectile_component.passive_skills.push(skill);
        }
        create_projectile(
            &mut commands,
            material_manager,
            projectile_component,
            target_position.position.clone(),
            skill.projectiles,
            &skill.skill_direction
        );                       

    } else {
        if missed {                                                             // if missed we put text into target and return from the function - no need to do next;
            target_text_component.text_upper_charactor.push(DamageTextInformer::new("MISSED".to_string(), false, None));
            return;
        }

        match target_abilities.ability.get(&AbilityType::Evasion) {             //check for target evasion;
            Some(v) => {
                if *v > 0 {
                    let random_evasion_number: u8 = rng.gen_range(0..=99);
                    if *v >= random_evasion_number as i16 {                     //target evaded shot, put text into target and return from the function;
                        target_text_component.text_upper_charactor.push(DamageTextInformer::new("EVADED".to_string(), false, None)); 
                        return;
                    }
                }
            },
            _ => {}
        };

        let block_amount: i16 = match target_abilities.ability.get(&AbilityType::BlockChance) {       //set block amount if block chance triggered, otherwise use 0;
            Some(v) => {
                let block_chance_random_number: u8 = rng.gen_range(0..=99);
                if *v >= block_chance_random_number as i16 {
                    match target_abilities.ability.get(&AbilityType::BlockAmount) {
                        Some(v) => *v,
                        _ => {
                            println!("Target has no block amount and have block chance, i use 0 instead");
                            0
                        }
                    }
                } else {
                    0
                }
            },
            _ => {
                0
            }
        };

        let critical_hit_random_number = rng.gen_range(0..=99);
        let critical_hit_multiplier = if skill.crit_chance > critical_hit_random_number {           //calculating critical multiplier;
            skill.crit_multiplier
        } else {
            100
        };
        
        for (damage_type, value) in skill.damage.iter() {                       //create and apply damage to target;
            let resist_type = resists_types::get_resist_from_damage_type(damage_type);
            let target_damage_resist = match target_resists.resists.get(&resist_type) {
                Some(v) => *v,
                _ => {
                    println!(
                        "Target has no damage resist: '{:?}', I use 0 instead",
                        damage_type
                    );
                    0
                }
            };
            //do not need to use damage multiplier from ability, cause already applied from  ability when skill is created;

            let value_with_multiplier = value * critical_hit_multiplier / 100;                    //calculating new damage value with multiplier from critical hit;
            let damage = value_with_multiplier - (value_with_multiplier * target_damage_resist / 100) - (value_with_multiplier * block_amount / 100 );  // calculating damage;
            let stat = if *damage_type == DamageType::Stamina {                                 //which health or stamina points get damaged;
                Stat::StaminaPoints
            } else {
                Stat::HealthPoints
            };

            if damage <= 0 {                                                                    //check for negative damage (excluding healing by damage xD);
                continue;
            }

            charactor::change_health_stamina_points(                                        //do damage;
                &mut target_stats.stats,
                &mut target_stats.stats_cache,
                &stat,
                damage,
            );

            let bold: bool = if critical_hit_multiplier > 100 {                                 //check for bigger text or not for inform to ui;
                true
            } else {
                false
            };

            target_text_component.text_upper_charactor.push(DamageTextInformer::new(damage.to_string(), bold, Some(damage_type)));  //set damage to target informer;
        }

        for (effect_type, trigger_chance) in skill.effect.iter_mut() {          //set effects on target, if triggered;
            let trigger_chance_random_number: u8 = rng.gen_range(0..=99);
            if *trigger_chance < trigger_chance_random_number {
                continue;                                                                              //skip effect, because not triggered;
            };

            let effect_config = effects_deploy.get_effect_config(effect_type);          //create default effect;
            let mut effect = Effect::new(effect_config);
            let effect_resist = resists_types::get_resist_from_effect_type(effect_type);  //convert effect type to resist type;
            let target_effect_resist = match target_resists.resists.get(&effect_resist) {       //get resist from target on this effect to change duration;
                Some(v) => *v,
                _ => {
                    println!(
                        "Target has no effect resist: '{:?}', I use 0 instead",
                        effect_type
                    );
                    0
                }
            };
            
            if target_effect_resist > 100 {                                                        //check target resist; if it > 100% just ignore this effect;
                continue;
            }

            //calculate new effect duration by target resist;
            let effect_duration = effect.duration * target_effect_resist as f32 / 100.0;
            effect.duration -= effect_duration;

            target_effects.effects.entry(effect_type.clone()).and_modify(|x| x.duration += effect.duration).or_insert(effect);

        }
        
        //check for passivly skills on damage
        for (skill_type, trigger_chance) in skill.extra_skill.iter() {
            let skill_trigger_chance_random_number: u8 = rng.gen_range(0..=99);
            if *trigger_chance < skill_trigger_chance_random_number {
                continue;                                                               // skip passive skill, not triggered;
            }

            let mut skill = Skill::new(skills_deploy.get_skill_deploy(skill_type));         //create new default skill;
            for (damage_type, value) in skill.damage.iter_mut() {
                let resist_type = resists_types::get_resist_from_damage_type(damage_type);
                let target_resist_multiplier = match target_resists.resists.get(&resist_type) {
                    Some(v) => *v,
                    None => 0,
                };

                let ability_type = abilities::get_ability_type_from_damage_type(damage_type);
                let damage_multiplier = match charactor_ability.ability.get(&ability_type) {
                    Some(v) => *v,
                    None => 0
                };

                let damage_with_multiplier = *value + *value * damage_multiplier / 100;
                let temp_value = damage_with_multiplier - damage_with_multiplier * target_resist_multiplier / 100;

                if temp_value < 0 {
                    *value = 0;
                } else {
                    *value = temp_value;
                }                                    
            } 
            match target_skills.passive_skills.get_mut(skill_type) {
                Some(v) => {
                    skill.life_time += v.life_time;                       // prolong time duration;
                    *v = skill;
                },
                None => {
                    target_skills.passive_skills.insert(skill_type.clone(), skill);
                },
            }
        } 
    }
}
