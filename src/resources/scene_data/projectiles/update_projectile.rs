use bevy::prelude::*;
use rand::Rng;
use crate::{components::{projectile_component::Projectile, IdentificationComponent, thing_component::ThingComponent, charactor_component::{CharactorComponent, EffectComponent, SkillComponent, AbilityComponent}, PositionComponent, tile_component::TileComponent, DamageTextComponent, AttributesComponent, ResistsComponent, ObjectType}, materials::material_manager::MaterialManager, scenes::game_scenes::tilemap::tile::Position, resources::{scene_data::{charactor::{skills::SkillDirectionType, effects::{EffectDeploy, Effect}, change_attribute_points, CharactorStatus}, stuff::{damage_type::DamageType, resists_types::{get_resist_from_damage_type, get_resist_from_effect_type}}, damage_text_informer::DamageTextInformer, Attribute, AbilityType}, deploy::Deploy}, config::TILE_SIZE};

pub fn update_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    deploy: Res<Deploy>,
    mut projectile_query: Query<(Entity, &Projectile, &mut Transform)>, 
    mut all_query: Query<(&PositionComponent, &IdentificationComponent), Without<TileComponent>>,
    mut things_query: Query<(&ThingComponent, &ResistsComponent, &mut AttributesComponent, &mut DamageTextComponent), With<ThingComponent>>,
    mut charactors_query: Query<(&mut AttributesComponent, &ResistsComponent, &mut EffectComponent, &mut SkillComponent, &AbilityComponent, &mut DamageTextComponent), With<CharactorComponent>>,
) {
    let delta = time.delta_seconds();
    for(projectile_entity, projectile, mut transfrom) in projectile_query.iter_mut() {
        transfrom.translation.x += projectile.motion_coefficient.x * projectile.velocity as f32 * delta;
        transfrom.translation.y += projectile.motion_coefficient.y * projectile.velocity as f32 * delta;
        if try_grid_move(transfrom.translation.x, transfrom.translation.y, &mut projectile.current_position) {
            let object_type = match check_for_collision(transfrom.translation.x, transfrom.translation.y, &mut all_query){
                Some(v) => {
                    match v {
                        ObjectType::Charactor(_) => todo!(),
                        ObjectType::Stuff => todo!(),
                        ObjectType::Thing => todo!(),
                        ObjectType::Projectile => todo!(),
                        ObjectType::Tile => todo!(),
                    }
                },
                None => continue,
            }
        }
    }
}

pub fn check_for_collision(
    projectile_x: i32,
    projectile_y: i32,
    all_query: &mut Query<(&PositionComponent, &IdentificationComponent), Without<TileComponent>>,
) -> Option<ObjectType> {
    let mut random = rand::thread_rng();
    for (position, identification) in all_query.iter() {
        let target_x = position.position.x;
        let target_y = position.position.y;
        if projectile_x != target_x || projectile_y != target_y {               //check for position and target;
            continue;
        }

        match identification.object_type {
            crate::components::ObjectType::Charactor(_) => {
                for(
                    mut charactor_attributes, 
                    charactor_resists,
                    mut charactor_effects, 
                    mut charactor_skills, 
                    charactor_abilities, 
                    mut text_component
                ) in charactors_query.iter_mut() {
                    match charactor_abilities.ability.get(&AbilityType::Evasion) {           //check for evade
                        Some(v) => {
                            let random_evade_chance: i16 = random.gen_range(0..=99);
                            if *v > random_evade_chance {                                    //evaded
                                text_component.text_upper.push(DamageTextInformer::new("Evaded".to_string(), false, None));
                                commands.entity(projectile_entity).despawn_recursive();
                                return;
                            }
                        },
                        None => {},
                    }

                    let block_amount_percent = match charactor_abilities.ability.get(&AbilityType::BlockChance) {      //check for block and amount;
                        Some(v) => {
                            let random_block_chance: i16 = random.gen_range(0..=99);
                            if *v > random_block_chance {                                   //blocked some damage;
                                match charactor_abilities.ability.get(&AbilityType::BlockAmount) {
                                    Some(value) => *value,
                                    None => 0,
                                }
                            } else {
                                0
                            }
                        },
                        None => 0,
                    };

                    for (damage_type, damage) in projectile.damage.iter() {
                        let charactor_resist = match charactor_resists.resists.get(&get_resist_from_damage_type(damage_type)) {
                            Some(v) => *v,
                            None => 0,
                        };
                        let total_damage = damage - damage * charactor_resist / 100 - damage * block_amount_percent / 100;
                        
                        let attribute = if *damage_type == DamageType::Stamina {
                            Attribute::Stamina
                        } else {
                            Attribute::Health
                        };

                        change_attribute_points(&mut charactor_attributes, &attribute, total_damage, false);
                    }

                    for effect_type in projectile.effects.iter(){
                        let effect_config: &EffectDeploy = deploy.charactor_deploy.effects_deploy.get_effect_config(effect_type);
                        let mut effect = Effect::new(effect_config);
                        let charactor_resist = match charactor_resists.resists.get(&get_resist_from_effect_type(effect_type)) {
                            Some(v) => *v,
                            None => 0,
                        };

                        effect.duration -= effect.duration * charactor_resist as f32 / 100.0;
                        charactor_effects.effects.entry(effect_type.clone()).and_modify(|x| x.duration += effect.duration).or_insert(effect);
                    }

                    for skill in projectile.passive_skills.iter() {
                        let mut new_skill = skill.clone();
                        match charactor_skills.passive_skills.get_mut(&skill.skill_type) {
                            Some(v) => {
                                new_skill.life_time += v.life_time;                       // prolong time duration;
                                *v = new_skill;
                            },
                            None => {
                                charactor_skills.passive_skills.insert(skill.skill_type.clone(), new_skill);
                            },
                        }
                    }
                    commands.entity(projectile_entity).despawn_recursive();
                    break;
                }
            },
            crate::components::ObjectType::Thing => {
                for(thing_cmponent, mut resists_component, mut attributes_component, mut text_informer) in things_query.iter_mut() {
                    for (damage_type, damage) in projectile.damage.iter() {
                        let thing_resits = match resists_component.resists.get(&get_resist_from_damage_type(damage_type)){
                            Some(v) => *v,
                            None => 0,
                        };
                        if *damage_type == DamageType::Stamina {
                            continue;
                        };

                        let damage_with_resist = damage - damage * thing_resits / 100;
                        match attributes_component.attributes.get_mut(&Attribute::Health) {
                            Some(v) => *v -= damage_with_resist,
                            None => {},
                        }
                        let text_damage = DamageTextInformer::new(damage_with_resist.to_string(), false, Some(damage_type));
                        text_informer.text_upper.push(text_damage);
                    }
                    commands.entity(projectile_entity).despawn_recursive();
                    return;
                }
            },
            _ => return,
        }
        /*
        if projectile.can_pierce {
            let projectile_piercing_chance = projectile.pierce_chance;
            let random_priecing_chance: u8 = random.gen_range(0..=99);
            if projectile_piercing_chance > random_priecing_chance {
                continue;
            }
        }
        */        
    }
}

fn collision_with_charactor(
    commands: &mut Commands,
    deploy: &Deploy,
    projectile: &Projectile,
    projectile_entity: Entity,
    mut charactors_query: Query<(&mut AttributesComponent, &ResistsComponent, &mut EffectComponent, &mut SkillComponent, &AbilityComponent, &mut DamageTextComponent), With<CharactorComponent>>,

){}
fn collision_with_thing(){}
fn try_grid_move( x: f32, y: f32, position: &mut Position<i32>) -> bool {

}

pub fn create_projectile(
    commands: &mut Commands,
    material_manager: &MaterialManager,
    mut projectile: Projectile,      
    target_position: Position<i32>,
    projectiles_value: u8, 
    skill_direction: &SkillDirectionType
) {
    let arc: f32 = match *skill_direction {
        SkillDirectionType::Line => 0.0,
        SkillDirectionType::Arc15 => 15.0,
        SkillDirectionType::Arc30 => 30.0,
        SkillDirectionType::Arc45 => 45.0,
        SkillDirectionType::Arc60 => 60.0,
        SkillDirectionType::Arc90 => 90.0,
        SkillDirectionType::Arc180 => 180.0,
        SkillDirectionType::Arc360 => 360.0,
        SkillDirectionType::Point => {
            println!("Can not create projectiles where skill direction type is 'POINT'");
            return;
        },
    }; 
    let starting_point_x = projectile.current_position.x;
    let starting_point_y = projectile.current_position.y;

    let texture_atlas = material_manager.game_scene.projectiles.get_texture_atlas(&projectile.projectile_type);

    let half_arc_angle = arc / 2.0;
    let angle_coefficient = if projectiles_value == 1 {                                             //each angle to cast projectile;
        0.0
    } else {
        arc / projectiles_value as f32
    };                                     
    let delta_x = starting_point_x as f32 - target_position.x as f32;                //difference between target position and starting position;
    let delta_y = starting_point_y as f32 - target_position.y as f32;
    let angle_between_ab_and_y = (delta_x.atan2(delta_y)).to_degrees();                             //angle between Y and line cast to target position;
    let radius = (delta_x*delta_x + delta_y*delta_y).sqrt();

    for i in 0..projectiles_value {
        let x = starting_point_x as f32 + radius * (angle_between_ab_and_y - half_arc_angle + angle_coefficient * i as f32).to_radians().sin();
        let y = starting_point_y as f32 + radius * (angle_between_ab_and_y - half_arc_angle + angle_coefficient * i as f32).to_radians().cos();
        let new_delta_x = starting_point_x as f32 - x;
        let new_delta_y = starting_point_y as f32 - y;
        let distance = (new_delta_x.powf(2.0) + new_delta_y.powf(2.0)).sqrt();
        projectile.motion_coefficient.x = new_delta_x / distance;
        projectile.motion_coefficient.y = new_delta_y / distance;
        
    }
}