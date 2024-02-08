use bevy::prelude::*;
use rand::Rng;

use crate::components::{IdentificationComponent, ObjectType, PositionComponent, StatsComponent, TakenDamage, TakenDamageComponent};
use crate::components::charactor_component::{
    ActionType, CharactorComponent, SkillAndEffectComponent, CharactorTargetComponent,
};


use crate::resources::deploy::Deploy;
use crate::resources::scene_data::Ability;
use crate::resources::scene_data::damage_text_informer::DamageIgnored;
use crate::resources::scene_data::projectiles::setup_projectile_with_active_skill;
use crate::resources::scene_manager::SceneManager;
use crate::scenes::game_scenes::game_scene::GameScene;
use crate::scenes::game_scenes::tilemap::tile::Position;

use super::effects::EffectStatus;
use super::skills::ActiveSkill;
use super::update_move::calculate_direction;
use super::CharactorStatus;

pub fn update_attack_from_basic_skill(
    mut scene_manager: ResMut<SceneManager>,
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
    let scene = scene_manager.get_current_game_scene_mut();
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
        
        for (
            target,
            target_position,
            mut target_taken_damage,
        ) in target_query.iter_mut() {
            match target.object_type {
                ObjectType::Charactor(_, id) => {
                    if id == target_id {
                        try_to_attack(
                            &mut charactor,
                            charactor_stats, 
                            &charactor_position.position, 
                            &target_position.position, 
                            skill,
                            &mut target_taken_damage,
                            &deploy,
                            scene,
                        );
                    } else {
                        continue;
                    }
                },
                _ => continue,
            }
        }
    }
}

fn try_to_attack(
    charactor: &mut CharactorComponent,
    charactor_ability: &StatsComponent,
    charactor_position: &Position<i32>,
    target_position: &Position<i32>,
    skill: &mut ActiveSkill,
    target_taken_damage: &mut TakenDamageComponent,
    deploy: &Deploy,
    scene: &mut GameScene,

) {                                                                     //check for target position;
    let diff_x = (target_position.x - charactor_position.x).abs();         // always positive value;
    let diff_y = (target_position.y - charactor_position.y).abs();         // always positive value;
    let diff = diff_x.max(diff_y);

    if skill.skill_range as i32 >= diff {                                             //check for skill range;
        attack(
            skill,
            charactor,
            charactor_ability,
            charactor_position,
            target_position,
            target_taken_damage,
            deploy,
            scene
        );
    } else {
        return;
    }
}

fn attack(
    skill: &mut ActiveSkill,
    charactor: &mut CharactorComponent,
    charactor_ability: &StatsComponent,
    charactor_position: &Position<i32>,
    target_position: &Position<i32>,
    target_taken_damage: &mut TakenDamageComponent,
    deploy: &Deploy,
    scene: &mut GameScene,
) {
    let mut random = rand::thread_rng();
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
        charactor_position.x, 
        charactor_position.y, 
        target_position.x,
        target_position.y
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
    let random_accuracy_number: u8 = random.gen_range(0..100);
    let is_missed = if accuracy <= 0 || accuracy < random_accuracy_number as i16 {
        true
    } else {
        false
    };

    match skill.projectile_type.as_ref() {
        Some(_) => {
            setup_projectile_with_active_skill(scene, skill, charactor_position, target_position, deploy, is_missed);
        },
        None => {
            do_direct_damage(is_missed, target_taken_damage, skill);
        }
    }
}

fn do_direct_damage(
    is_missed: bool,
    target_taken_damage: &mut TakenDamageComponent,
    skill: &mut ActiveSkill,
){
    if is_missed {
        let damage = TakenDamage {missed_or_evaded: Some(DamageIgnored::Missed),..Default::default()};
        target_taken_damage.damage.push(damage);
        return;
    };
    let mut random = rand::thread_rng();
    let critical_hit_random_number = random.gen_range(0..=99);
    let critical_hit_multiplier = if skill.crit_chance > critical_hit_random_number {                                                   //calculating critical multiplier;
        skill.crit_multiplier
    } else {
        100
    };

    let mut damage: TakenDamage = Default::default();
    for (damage_type, value) in skill.damage.iter() {                                                                          //create and apply damage to target;       
        let value_with_multiplier = *value * critical_hit_multiplier / 100;                                                              //calculating new damage value with multiplier from critical hit;

        let critical_hit: bool = if critical_hit_multiplier > 0 {                                                                             //check for bigger text or not for inform to ui;
            true
        } else {
            false
        };
        damage.damage.insert(damage_type.clone(), value_with_multiplier);
        damage.is_critical_hit = critical_hit;
    }

    for (_, (effect, trigger_chance)) in skill.effects.iter() {                                                                                      //set effects on target, if triggered;
        let trigger_chance_random_number: u8 = random.gen_range(0..=99);
        if *trigger_chance < trigger_chance_random_number {
            continue;                                                                                                                          //skip effect, because not triggered;
        };

        damage.effects.push(effect.clone());
    }

    for (_, (passive_skill, trigger_chance)) in skill.passive_skills.iter() {
        let skill_trigger_chance_random_number: u8 = random.gen_range(0..=99);
        if *trigger_chance < skill_trigger_chance_random_number {
            continue;                                                                                                                           // skip passive skill, not triggered;
        }
        damage.passive_skills.push(passive_skill.clone());
    }

    target_taken_damage.damage.push(damage);
}