use bevy::prelude::*;
use rand::Rng;

use crate::components::tile_component::TileComponent;
use crate::components::{PositionComponent, IdentificationComponent, DamageTextComponent, ResistsComponent, AttributesComponent};
use crate::components::charactor_component::{
    AbilityComponent, ActionType, CharactorComponent, EffectComponent,
    SkillComponent, CharactorTargetComponent,
};

use crate::components::projectile_component::Projectile;
use crate::materials::material_manager::MaterialManager;
use crate::resources::deploy::Deploy;
use crate::resources::scene_data::{get_resist_from_damage_type, Ability, Attribute};
use crate::resources::scene_data::damage_text_informer::{DamageTextInformer, TextDamageType};
use crate::resources::scene_data::projectiles::update_projectile::create_projectile;
use crate::scenes::game_scenes::tilemap::tile::Position;

use super::effects::EffectStatus;
use super::skills::{ActiveSkill, TargetType};
use super::{change_attribute_points, CharactorType};
use super::update_move::calculate_direction;
use super::CharactorStatus;

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
    ), With<CharactorComponent>>,

    mut target_query: Query<(
        &CharactorComponent,
        &IdentificationComponent,
        &mut AttributesComponent,
        &ResistsComponent,
        &mut EffectComponent,
        &PositionComponent,
        &AbilityComponent,
        &mut DamageTextComponent,
        &mut SkillComponent,
    ), Without<TileComponent>>,
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
        
        let skill = &mut charactor_skill.base_skill;
        if skill.on_cooldown {                                          //check for colldown
            continue;                                                   //go to next charactor;
        }

        for (
            target_component,
            target_identification,
            mut target_attributes,
            target_resists,
            mut target_effects,
            target_position,
            target_abilities,
            mut target_text,
            mut target_skills,
        ) in target_query.iter_mut() {
            let mut targets_position_in_skill_range: Vec<Position<i32>> = vec![];
            if skill.projectiles > 1 {
                let charactor_position_x = charactor_position.position.x;
                let charactor_position_y = charactor_position.position.y;
                let target_position_x = target_position.position.x;
                let target_position_y = target_position.position.y;

                let distance = (((target_position_x - charactor_position_x) as f32).powf(2.0) + ((target_position_y - charactor_position_y) as f32).powf(2.0)).sqrt() as i32;
                if distance <= skill.skill_range as i32{
                    match skill.target {
                        TargetType::Allies => {
                            match charactor.charactor_type {
                                CharactorType::Player => {
                                    if target_component.charactor_type == CharactorType::Companion {
                                        targets_position_in_skill_range.push(target_position.position.clone());
                                    }
                                },
                                CharactorType::NPC => {},
                                CharactorType::Monster => {
                                    if target_component.charactor_type == CharactorType::Monster {
                                        targets_position_in_skill_range.push(target_position.position.clone());
                                    }
                                },
                                CharactorType::Companion => {
                                    if target_component.charactor_type == CharactorType::Player {
                                        targets_position_in_skill_range.push(target_position.position.clone());
                                    }
                                },
                            }
                        },
                        TargetType::Enemies => {
                            match charactor.charactor_type {
                                CharactorType::Player => {
                                    if target_component.charactor_type == CharactorType::Monster {
                                        targets_position_in_skill_range.push(target_position.position.clone());
                                    }
                                },
                                CharactorType::NPC => {},
                                CharactorType::Monster => {
                                    if target_component.charactor_type == CharactorType::Player
                                    || target_component.charactor_type == CharactorType::Companion {
                                        targets_position_in_skill_range.push(target_position.position.clone());
                                    }
                                },
                                CharactorType::Companion => {
                                    if target_component.charactor_type == CharactorType::Monster {
                                        targets_position_in_skill_range.push(target_position.position.clone());
                                    }
                                },
                            }
                        },
                        TargetType::All => {
                            if target_component.charactor_type != CharactorType::NPC {
                                targets_position_in_skill_range.push(target_position.position.clone());
                            }
                        },
                    }
                }
            }
            
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
                        targets_position_in_skill_range,
                    );
                    break;
                }
            }
        }
    }
}

fn try_to_attack(
    position: &PositionComponent,
    target_position: &PositionComponent,
    skill: &mut ActiveSkill,
) -> bool {
    let skill_range = skill.skill_range;                                          //check for target position;
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
    skill: &mut ActiveSkill,
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
    mut targets_position_in_skill_range: Vec<Position<i32>>,
) {
    let mut rng = rand::thread_rng();

    skill.on_cooldown = true;                                                                    //set skill on cooldown;

    let accuracy = match charactor_ability.ability.get(&Ability::Accuracy) {            //get accuracy
        Some(v) => *v,
        _ => {
            println!("Can't get Accuracy, use 0 instead, so 100% chance to miss");
            0
        }
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
            starting_position: charactor_position.position.clone(),
            range: skill.skill_range,
            ..Default::default()
        };

        let random_accuracy_number: u8 = rng.gen_range(0..=99);
        if accuracy <= 0 || accuracy < random_accuracy_number as i16 {
            projectile_component.is_missed = true;
            create_projectile(
                &mut commands,
                material_manager,
                projectile_component,
                target_position.position.clone(),
            );
            return;
        };

        setup_projectile(&mut projectile_component, skill);
        create_projectile(
            &mut commands,
            material_manager,
            projectile_component,
            target_position.position.clone(),
        );

        if skill.projectiles == 1 {
            return;
        }
        
        for _ in 1..skill.projectiles {                                             //ignore 1-st projectile
            if targets_position_in_skill_range.len() == 0 {
                create_projectile(
                    &mut commands,
                    material_manager,
                    projectile_component.clone(),
                    target_position.position.clone(),
                );
            } else {
                create_projectile(
                    &mut commands,
                    material_manager,
                    projectile_component.clone(),
                    targets_position_in_skill_range[0].clone(),
                );
                targets_position_in_skill_range.remove(0);
            }
        } 

    } else {
        let random_accuracy_number: u8 = rng.gen_range(0..=99);
        if accuracy <= 0 || accuracy < random_accuracy_number as i16 {
            target_text_component.text_upper.push(DamageTextInformer::new(0, Some(TextDamageType::Missed), false, None));
            return;
        };

        match target_abilities.ability.get(&Ability::Evasion) {             //check for target evasion;
            Some(v) => {
                if *v > 0 {
                    let random_evasion_number: u8 = rng.gen_range(0..=99);
                    if *v >= random_evasion_number as i16 {                     //target evaded shot, put text into target and return from the function;
                        target_text_component.text_upper.push(DamageTextInformer::new(0, Some(TextDamageType::Evaded), false, None)); 
                        return;
                    }
                }
            },
            _ => {}
        };

        let block_amount: i16 = match target_abilities.ability.get(&Ability::BlockChance) {       //set block amount if block chance triggered, otherwise use 0;
            Some(v) => {
                let block_chance_random_number: u8 = rng.gen_range(0..=99);
                if *v >= block_chance_random_number as i16 {
                    match target_abilities.ability.get(&Ability::BlockAmount) {
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
        let critical_hit_multiplier = if skill.crit_chance > critical_hit_random_number {                                                       //calculating critical multiplier;
            skill.crit_multiplier
        } else {
            0
        };
        
        for (damage_type, value) in skill.damage.iter() {                                                                           //create and apply damage to target;
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

            let value_with_multiplier = *value +  *value * critical_hit_multiplier / 100;                                                               //calculating new damage value with multiplier from critical hit;
            let mut damage = value_with_multiplier - (value_with_multiplier * target_damage_resist / 100) - (value_with_multiplier * block_amount / 100 );  // calculating damage;
            
            if damage <= 0 {                                                                                                                                //check for negative damage (excluding healing by damage xD);
                damage = 0;
            }

            change_attribute_points(
                target_attributes,
                &Attribute::new(damage_type),
                damage,
                false,
            );

            let bold: bool = if critical_hit_multiplier > 0 {                                                                                               //check for bigger text or not for inform to ui;
                true
            } else {
                false
            };

            target_text_component.text_upper.push(DamageTextInformer::new(damage, None, bold, Some(damage_type)));          //set damage to target informer;
        }

        for (effect_type, (effect, trigger_chance)) in skill.effects.iter() {                                                                                      //set effects on target, if triggered;
            let trigger_chance_random_number: u8 = rng.gen_range(0..=99);
            if *trigger_chance < trigger_chance_random_number {
                continue;                                                                              //skip effect, because not triggered;
            };

            target_effects.added_effect.push(effect.clone());
        }

        for (skill_type, (passive_skill, trigger_chance)) in skill.passive_skills.iter() {
            let skill_trigger_chance_random_number: u8 = rng.gen_range(0..=99);
            if *trigger_chance < skill_trigger_chance_random_number {
                continue;                                                               // skip passive skill, not triggered;
            }
            target_skills.added_passive_skills.push(passive_skill.clone());
        } 
    }
}


fn setup_projectile(projectile_component: &mut Projectile, skill: &mut ActiveSkill){
    let mut rng = rand::thread_rng();
    let critical_hit_random_number = rng.gen_range(0..=99);
    let critical_hit_multiplier = if skill.crit_chance > critical_hit_random_number {                           //calculating critical multiplier;
        skill.crit_multiplier
    } else {
        0
    };

    for (damage, value) in skill.damage.iter() {
        let new_value = *value + *value * critical_hit_multiplier / 100;
        projectile_component.damage.insert(damage.clone(), new_value);
    }

    for (effect_type, (effect, trigger_chance)) in skill.effects.iter() {                //check for triggered effects;
        let trigger_chance_random_number: u8 = rng.gen_range(0..=99);
        if *trigger_chance < trigger_chance_random_number {
            continue;                                                                                           //not triggered;
        }
        projectile_component.effects.push(effect.clone());                                                       //store triggered effects to projectile;
    }

    for (skill_type, (passive_skill, trigger_chance)) in skill.passive_skills.iter() {
        let trigger_chance_random_number: u8 = rng.gen_range(0..=99);
        if *trigger_chance < trigger_chance_random_number {
            continue;                                                                                              //not triggered;
        }

        projectile_component.passive_skills.push(passive_skill.clone());
    }
}