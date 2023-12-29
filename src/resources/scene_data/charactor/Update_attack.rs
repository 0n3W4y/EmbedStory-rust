use bevy::prelude::*;
use rand::Rng;

use crate::components::{PositionComponent, IdentificationComponent, TakenDamageComponent, TakenDamage, ObjectType, StatsComponent};
use crate::components::charactor_component::{
    ActionType, CharactorComponent, SkillAndEffectComponent, CharactorTargetComponent,
};

use crate::components::projectile_component::Projectile;
use crate::materials::material_manager::MaterialManager;
use crate::resources::deploy::Deploy;
use crate::resources::scene_data::Ability;
use crate::resources::scene_data::damage_text_informer::DamageIgnored;
use crate::resources::scene_data::projectiles::ProjectileType;
use crate::resources::scene_data::projectiles::update_projectile::create_projectile;
use crate::scenes::game_scenes::tilemap::tile::Position;

use super::effects::EffectStatus;
use super::skills::ActiveSkill;
use super::CharactorType;
use super::update_move::calculate_direction;
use super::CharactorStatus;

pub fn update_attack_from_basic_skill(
    mut commands: Commands,
    material_manager: Res<MaterialManager>,
    deploy: Res<Deploy>,
    mut charactor_query: Query<(
        &mut CharactorComponent,
        &PositionComponent,
        &mut SkillAndEffectComponent,
        &CharactorTargetComponent,
        &StatsComponent,
    )>,

    mut target_query: Query<(
        &IdentificationComponent,
        &PositionComponent,
        &mut TakenDamageComponent,
    )>,
) {
    for (
        mut charactor, 
        charactor_position,
        mut charactor_skill, 
        charactor_target, 
        charactor_stats,
    ) in charactor_query.iter_mut() {
        if charactor_target.action != ActionType::Attack {                                                      //check for target status; And skip all if not attacking;
            continue;
        }

        match charactor_skill.effect_status.iter().find(|&x| *x == EffectStatus::CanNotAttack) {
            Some(_) => continue,
            None => {},
        }

        let target_id = match charactor_target.target {                                                    //checking for target id;
            Some(v) => v,
            None => {
                println!(
                    "Can not attack, because charactor '{:?}, {:?}, {:?}' have Attack action, but doesnt have a target", 
                    charactor.charactor_type, 
                    charactor.gender_type, 
                    charactor.race_type
                );
                continue;
            }
        };
        
        let skill = &mut charactor_skill.base_skill;
        if skill.on_cooldown {                                          //check for cooldown
            continue;                                                   //go to next charactor;
        }

        let mut elapsed_target_quantity = skill.target_quantity;    //for multiply targets;
        for (
            target_identification,
            target_position,
            mut target_taken_damage,
        ) in target_query.iter_mut() {
            todo!();
            /*
            if skill.target_quantity > 1 {
                if try_to_attack(&charactor_position.position, &target_position.position, skill.skill_range) {
                    match target_identification.object_type {
                        ObjectType::Charactor(charactor_type, id) => {
                            if id == target_id {
                                targets_in_skill_range.insert(0, (&target_position, &mut target_taken_damage));
                                continue;
                            } else {
                                match charactor_type {
                                    CharactorType::Player => {
                                        if charactor.charactor_type == CharactorType::Monster {
                                            targets_in_skill_range.push((&target_position, &mut target_taken_damage));
                                        }
                                    },
                                    CharactorType::NPC => {},
                                    CharactorType::Monster => {
                                        if charactor.charactor_type != CharactorType::Monster {
                                            targets_in_skill_range.push((&target_position, &mut target_taken_damage));
                                        }
                                    },
                                    CharactorType::Companion => {
                                        if charactor.charactor_type == CharactorType::Monster {
                                            targets_in_skill_range.push((&target_position, &mut target_taken_damage));
                                        }
                                    },
                                }
                            }
                        },
                        ObjectType::Thing(_) => {

                        },
                        _ => {},
                    }
                };
            } else {
                match target_identification.object_type  {
                    ObjectType::Charactor(_, id) => {
                        if id == target_id {
                            if try_to_attack(&charactor_position.position, &target_position.position, skill.skill_range) {      //try to attack;
                                attack_single_target(
                                    &mut commands,
                                    skill,
                                    &mut charactor,
                                    charactor_stats,
                                    charactor_position,
                                    target_position,
                                    &mut target_taken_damage,
                                    &deploy,
                                    &material_manager,
                                );
                                return;
                            }
                        }
                    },
                    ObjectType::Stuff(_) => todo!(),
                    ObjectType::Thing(_) => todo!(),
                    ObjectType::Projectile(_) => todo!(),
                    ObjectType::Tile(_) => todo!(),
                }
            }
            */
        }
    }
}

fn try_to_attack(
    charactor_position: &Position<i32>,
    target_position: &Position<i32>,
    skill_range: u8,
) -> bool {                                                                     //check for target position;
    let diff_x = (target_position.x - charactor_position.x).abs();         // always positive value;
    let diff_y = (target_position.y - charactor_position.y).abs();         // always positive value;
    let diff = diff_x.max(diff_y);

    if skill_range as i32 >= diff {                                             //check for skill range;
        return true;
    } else {
        return false;
    }
}

fn attack_single_target(
    mut commands: &mut Commands,
    skill: &mut ActiveSkill,
    charactor: &mut CharactorComponent,
    charactor_ability: &StatsComponent,
    charactor_position: &PositionComponent,
    target_position: &PositionComponent,
    target_taken_damage: &mut TakenDamageComponent,
    deploy: &Deploy,
    material_manager: &MaterialManager,
) {
    let mut rng = rand::thread_rng();
    skill.on_cooldown = true;                                                                    //set skill on cooldown;
    let accuracy = match charactor_ability.ability.get(&Ability::Accuracy) {                //get accuracy
        Some(v) => *v,
        _ => {
            println!("Can't get Accuracy, use 0 instead, so 100% chance to miss");
            0
        }
    };
    //need more calculating for use properly angle from Y;
    let direction = calculate_direction(                                            //get direction to set ccorrectly sprite position for sprite;
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

    if skill.projectile_type == ProjectileType::None {                                             //if None = melee, or ranged without projectile => instant direct damage;
        let random_accuracy_number: u8 = rng.gen_range(0..=99);
        if accuracy <= 0 || accuracy < random_accuracy_number as i16 {
            let damage = TakenDamage {missed_or_evaded: Some(DamageIgnored::Missed),..Default::default()};
            target_taken_damage.damage.push(damage);
            return;
        };

        let critical_hit_random_number = rng.gen_range(0..=99);
        let critical_hit_multiplier = if skill.crit_chance > critical_hit_random_number {                                                       //calculating critical multiplier;
            skill.crit_multiplier
        } else {
            0
        };

        let mut damage: TakenDamage = Default::default();
        for (damage_type, value) in skill.damage.iter() {                                                                           //create and apply damage to target;       
            let value_with_multiplier = *value +  *value * critical_hit_multiplier / 100;                                                               //calculating new damage value with multiplier from critical hit;

            let critical_hit: bool = if critical_hit_multiplier > 0 {                                                                                               //check for bigger text or not for inform to ui;
                true
            } else {
                false
            };
            damage.damage.insert(damage_type.clone(), value_with_multiplier);
            damage.is_critical_hit = critical_hit;
        }

        for (effect_type, (effect, trigger_chance)) in skill.effects.iter() {                                                                                      //set effects on target, if triggered;
            let trigger_chance_random_number: u8 = rng.gen_range(0..=99);
            if *trigger_chance < trigger_chance_random_number {
                continue;                                                                                                                                   //skip effect, because not triggered;
            };

            damage.effects.push(effect.clone());
        }

        for (skill_type, (passive_skill, trigger_chance)) in skill.passive_skills.iter() {
            let skill_trigger_chance_random_number: u8 = rng.gen_range(0..=99);
            if *trigger_chance < skill_trigger_chance_random_number {
                continue;                                                                                                                                       // skip passive skill, not triggered;
            }
            damage.passive_skills.push(passive_skill.clone());
        }

        target_taken_damage.damage.push(damage);
    } else {                                                                                        //if have a projectile type, create projectile
        let mut projectile_component = Projectile::new(&skill.projectile_type);         //create default projectile component;
        projectile_component.starting_position = charactor_position.position.clone();
        projectile_component.range = skill.skill_range;

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
            projectile_component.clone(),
            target_position.position.clone(),
        );
    }
}


fn setup_projectile(projectile_component: &mut Projectile, skill: &mut ActiveSkill){
    let mut rng = rand::thread_rng();
    let critical_hit_random_number = rng.gen_range(0..=99);
    let critical_hit_multiplier = if skill.crit_chance > critical_hit_random_number {                                                                           //calculating critical multiplier;
        skill.crit_multiplier
    } else {
        0
    };

    for (damage, value) in skill.damage.iter() {
        let new_value = *value + *value * critical_hit_multiplier / 100;
        projectile_component.damage.insert(damage.clone(), new_value);
    }

    for (effect_type, (effect, trigger_chance)) in skill.effects.iter() {                                                   //check for triggered effects;
        let trigger_chance_random_number: u8 = rng.gen_range(0..=99);
        if *trigger_chance < trigger_chance_random_number {
            continue;                                                                                                                                   //not triggered;
        }
        projectile_component.effects.push(effect.clone());                                                                                              //store triggered effects to projectile;
    }

    for (skill_type, (passive_skill, trigger_chance)) in skill.passive_skills.iter() {
        let trigger_chance_random_number: u8 = rng.gen_range(0..=99);
        if *trigger_chance < trigger_chance_random_number {
            continue;                                                                                                                                   //not triggered;
        }

        projectile_component.passive_skills.push(passive_skill.clone());
    }
}