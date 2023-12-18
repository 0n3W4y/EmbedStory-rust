use bevy::prelude::*;
use rand::Rng;

use crate::components::{PositionComponent, IdentificationComponent, DamageTextComponent, ResistsComponent, AttributesComponent};
use crate::components::charactor_component::{
    AbilityComponent, ActionType, CharactorComponent, EffectComponent,
    SkillComponent, CharactorTargetComponent,
};

use crate::components::projectile_component::Projectile;
use crate::materials::material_manager::MaterialManager;
use crate::resources::deploy::Deploy;
use crate::resources::scene_data::{AbilityType, get_resist_from_damage_type};
use crate::resources::scene_data::charactor::SkillSlot;
use crate::resources::scene_data::damage_text_informer::DamageTextInformer;
use crate::resources::scene_data::projectiles::update_projectile::create_projectile;

use super::effects::{Effect, EffectStatus};
use super::{get_ability_type_from_damage_type, change_attribute_points, get_attribute_from_damage_type};
use super::update_move::calculate_direction;
use super::{skills::Skill, CharactorStatus};

pub fn update_attack_from_basic_skill(
    mut commands: Commands,
    material_manager: Res<MaterialManager>,
    deploy: Res<Deploy>,
    mut charactor_query: Query<(
        &mut CharactorComponent,
        &PositionComponent,
        &mut SkillComponent,
        &CharactorTargetComponent,
        &AbilityComponent,
        &EffectComponent,
    )>,

    mut target_query: Query<(
        &IdentificationComponent,
        &mut AttributesComponent,
        &ResistsComponent,
        &mut EffectComponent,
        &PositionComponent,
        &AbilityComponent,
        &mut DamageTextComponent,
        &mut SkillComponent,
    )>,
) {
    

    for (
        mut charactor, 
        charactor_position,
        mut charactor_skill, 
        charactor_target, 
        charactor_ability,
        charactor_effect
    ) in charactor_query.iter_mut() {
        if charactor_target.action != ActionType::Attack {              //check for target status; And skip all if not attacking;
            continue;
        }

        match charactor_effect.effect_status.iter().find(|&x| *x == EffectStatus::CanNotAttack) {
            Some(_) => continue,
            None => {},
        }

        let target_id = match charactor_target.target {             //checking for target id;
            Some(v) => v,
            None => {
                println!(
                    "Can not attack, because charactor '{:?}, {:?}, {:?}' have Attack action, but doesnt have a target", 
                    charactor.charactor_type, 
                    charactor.gender_type, 
                    charactor.race_type);
                continue;
            }
        };
        
        match charactor_skill.skills.get_mut(&SkillSlot::Base) {                //get base attack skill
            Some(skill) => {
                if skill.on_cooldown {                                          //check for colldown
                    continue;                                                   //go to next charactor;
                }

                for (
                    target_identification,
                    mut target_attributes,
                    target_resists,
                    mut target_effects,
                    target_position,
                    target_abilities,
                    mut target_text,
                    mut target_skills,
                ) in target_query.iter_mut() {
                    if target_identification.id == target_id {                              //check for target
                        if try_to_attack(charactor_position, target_position, skill) {      //try to attack;
                            attack(
                                &mut commands,
                                skill,
                                &mut charactor,
                                charactor_ability,
                                charactor_position,
                                target_position,
                                &mut target_text,
                                &mut target_attributes,
                                target_resists, 
                                &mut target_effects, 
                                target_abilities, 
                                &mut target_skills,
                                &deploy,
                                &material_manager,
                            );
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
    let skill_range = skill.range;                                          //check for target position;
    let diff_x = (position.position.x - target_position.position.x).abs(); // always positive value;
    let diff_y = (position.position.y - target_position.position.y).abs(); // always positive value;
    let diff = diff_x.max(diff_y);

    if skill_range as i32 >= diff {                                             //check for skill range;
        return true;
    } else {
        return false;
    }
}

fn attack(
    mut commands: &mut Commands,
    skill: &mut Skill,
    charactor: &mut CharactorComponent,
    charactor_ability: &AbilityComponent,
    charactor_position: &PositionComponent,
    target_position: &PositionComponent,
    target_text_component: &mut DamageTextComponent,
    target_attributes: &mut AttributesComponent,
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

    let critical_hit_random_number = rng.gen_range(0..=99);
    let critical_hit_multiplier = if skill.crit_chance > critical_hit_random_number {           //calculating critical multiplier;
        skill.crit_multiplier
    } else {
        100
    };

    //need more calculating for use properly angle from Y;
    let direction = calculate_direction(                    //get direction to set ccorrectly sprite position for sprite;
        charactor_position.position.x, 
        charactor_position.position.y, 
        target_position.position.x,
        target_position.position.y
    );

    let charactor_status = if direction.x < 0 {
        CharactorStatus::AttackingLeft
    } else if direction.x > 0 {
        CharactorStatus::AttackingRight
    } else if direction.y < 0 {
        CharactorStatus::AttackingUp
    } else {
        CharactorStatus::AttackingDown
    };

    charactor.status = charactor_status;

    if skill.projectiles > 0 {
        let mut projectile_component = Projectile {             //create default projectile component;
            projectile_type: skill.projectile_type.clone(),
            current_position: charactor_position.position.clone(),
            is_critical_hit: if critical_hit_multiplier > 100 {true}else{false},
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

            for (damage_type, damage) in skill.damage.iter_mut() {
                let ability_type = get_ability_type_from_damage_type(damage_type);
                let damage_multiplier = match charactor_ability.ability.get(&ability_type) {
                    Some(v) => *v,
                    None => 0,
                };
                let multiplier_damage = *damage * damage_multiplier / 100;
                let total_damage = multiplier_damage * critical_hit_multiplier / 100;
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
            target_text_component.text_upper.push(DamageTextInformer::new(0, Some("MISSED".to_string()), false, None));
            return;
        }

        match target_abilities.ability.get(&AbilityType::Evasion) {             //check for target evasion;
            Some(v) => {
                if *v > 0 {
                    let random_evasion_number: u8 = rng.gen_range(0..=99);
                    if *v >= random_evasion_number as i16 {                     //target evaded shot, put text into target and return from the function;
                        target_text_component.text_upper.push(DamageTextInformer::new(0, Some("EVADED".to_string()), false, None)); 
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
        
        for (damage_type, value) in skill.damage.iter() {                       //create and apply damage to target;
            let resist_type = get_resist_from_damage_type(damage_type);
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
            
            if damage <= 0 {                                                                    //check for negative damage (excluding healing by damage xD);
                continue;
            }

            let attribute = get_attribute_from_damage_type(damage_type);
            change_attribute_points(                                        //do damage;
                target_attributes,
                &attribute,
                damage,
                false,
            );

            let bold: bool = if critical_hit_multiplier > 100 {                                 //check for bigger text or not for inform to ui;
                true
            } else {
                false
            };

            target_text_component.text_upper.push(DamageTextInformer::new(damage, None, bold, Some(damage_type)));  //set damage to target informer;
        }

        for (effect_type, trigger_chance) in skill.effect.iter_mut() {          //set effects on target, if triggered;
            let trigger_chance_random_number: u8 = rng.gen_range(0..=99);
            if *trigger_chance < trigger_chance_random_number {
                continue;                                                                              //skip effect, because not triggered;
            };

            let effect_config = effects_deploy.get_effect_config(effect_type);          //create default effect;
            let mut effect = Effect::new(effect_config);
            let effect_time_reducing = match target_abilities.ability.get(&AbilityType::ReducingEffectTime) {
                Some(v) => *v,
                None => 0,
            };
            
            if effect_time_reducing > 100 {                                                        //check target resist; if it > 100% just ignore this effect;
                continue;
            }

            //calculate new effect duration by target resist;
            effect.effect_duration -= effect.effect_duration * effect_time_reducing as f32 / 100.0;

            target_effects.effects.entry(effect_type.clone()).and_modify(|x| x.effect_duration += effect.effect_duration).or_insert(effect);

        }
        
        //check for passivly skills on damage
        for (skill_type, trigger_chance) in skill.extra_skill.iter() {
            let skill_trigger_chance_random_number: u8 = rng.gen_range(0..=99);
            if *trigger_chance < skill_trigger_chance_random_number {
                continue;                                                               // skip passive skill, not triggered;
            }

            let mut skill = Skill::new(skills_deploy.get_skill_deploy(skill_type));         //create new default skill;
            for (damage_type, value) in skill.damage.iter_mut() {
                let resist_type = get_resist_from_damage_type(damage_type);
                let target_resist_multiplier = match target_resists.resists.get(&resist_type) {
                    Some(v) => *v,
                    None => 0,
                };

                let ability_type = get_ability_type_from_damage_type(damage_type);
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
