use crate::{
    components::{
        charactor_component::{AbilityComponent, EffectComponent, SkillComponent},
        projectile_component::Projectile,
        tile_component::TileComponent,
        AttributesComponent, DamageTextComponent, IdentificationComponent, ObjectType,
        PositionComponent, ResistsComponent,
    },
    config::TILE_SIZE,
    materials::material_manager::MaterialManager,
    resources::
        scene_data::{
            charactor::{
                change_attribute_points,
                skills::SkillDirectionType,
            },
            damage_text_informer::DamageTextInformer,
            stuff::
                damage_type::DamageType,
            AbilityType, Attribute, get_resist_from_damage_type,
        },
    scenes::game_scenes::tilemap::tile::Position,
};
use bevy::prelude::*;
use rand::Rng;

const Z_POSITION: f32 = 4.0; // fourth layer;

pub fn update_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut projectile_query: Query<(Entity, &mut Projectile, &mut Transform)>,
    mut all_query: Query<
        (
            &PositionComponent,
            &IdentificationComponent,
            &mut AttributesComponent,
            &ResistsComponent,
            &mut EffectComponent,
            &mut SkillComponent,
            &AbilityComponent,
            &mut DamageTextComponent,
            //&mut Projectile,
        ),
        Without<TileComponent>,
    >,
) {
    let delta = time.delta_seconds();
    for (projectile_entity, mut projectile, mut transfrom) in projectile_query.iter_mut() {
        transfrom.translation.x +=
            projectile.motion_coefficient.x * projectile.velocity as f32 * delta;
        transfrom.translation.y +=
            projectile.motion_coefficient.y * projectile.velocity as f32 * delta;
        if try_grid_move(
            transfrom.translation.x,
            transfrom.translation.y,
            &mut projectile.current_position,
        ) {
            check_for_collision(
                &mut commands,
                &mut projectile,
                projectile_entity,
                &mut all_query,
            );
        }
    }
}

pub fn check_for_collision(
    commands: &mut Commands,
    projectile: &mut Projectile,
    projectile_entity: Entity,
    all_query: &mut Query<
        (
            &PositionComponent,
            &IdentificationComponent,
            &mut AttributesComponent,
            &ResistsComponent,
            &mut EffectComponent,
            &mut SkillComponent,
            &AbilityComponent,
            &mut DamageTextComponent,
            //&mut Projectile,
        ),
        Without<TileComponent>,
    >,
) -> Option<(ObjectType, usize)> {
    for (
        position,
        identification,
        mut attributes_component,
        resists_component,
        mut effects_component,
        mut skills_component,
        abilities_component,
        mut damage_text,
        //mut other_projectile
    ) in all_query.iter_mut()
    {
        let target_x = position.position.x;
        let target_y = position.position.y;
        if projectile.current_position.x == target_x && projectile.current_position.y == target_y {
            //check for position and target;
            match identification.object_type {
                ObjectType::Charactor(_) => {
                    collision_with_charactor(
                        commands,
                        projectile,
                        projectile_entity,
                        &mut attributes_component,
                        &resists_component,
                        &mut effects_component,
                        &mut skills_component,
                        &abilities_component,
                        &mut damage_text,
                    );
                }
                ObjectType::Thing => {
                    collision_with_thing(
                        commands,
                        projectile_entity,
                        projectile,
                        &mut attributes_component,
                        &resists_component,
                        &mut damage_text,
                    );
                }
                ObjectType::Projectile => {
                    collision_with_projectile();
                }
                _ => continue,
            }
            break;
        } else {
            continue;
        }
    }
    None
}

fn collision_with_charactor(
    commands: &mut Commands,
    projectile: &Projectile,
    projectile_entity: Entity,
    attributes: &mut AttributesComponent,
    resists: &ResistsComponent,
    effects: &mut EffectComponent,
    skills: &mut SkillComponent,
    abilities: &AbilityComponent,
    damage_text: &mut DamageTextComponent,
) {
    let mut random = rand::thread_rng();
    match abilities.ability.get(&AbilityType::Evasion) {
        //check for evade
        Some(v) => {
            let random_evade_chance: i16 = random.gen_range(0..=99);
            if *v > random_evade_chance {
                //evaded
                damage_text.text_upper.push(DamageTextInformer::new(
                    "Evaded".to_string(),
                    false,
                    None,
                ));
                commands.entity(projectile_entity).despawn_recursive();
                return;
            }
        }
        None => {}
    }

    let block_amount_percent = match abilities.ability.get(&AbilityType::BlockChance) {
        //check for block and amount;
        Some(v) => {
            let random_block_chance: i16 = random.gen_range(0..=99);
            if *v > random_block_chance {
                //blocked some damage;
                match abilities.ability.get(&AbilityType::BlockAmount) {
                    Some(value) => *value,
                    None => 0,
                }
            } else {
                0
            }
        }
        None => 0,
    };

    for (damage_type, damage) in projectile.damage.iter() {
        let charactor_resist = match resists
            .resists
            .get(&get_resist_from_damage_type(damage_type))
        {
            Some(v) => *v,
            None => 0,
        };
        let total_damage =
            damage - damage * charactor_resist / 100 - damage * block_amount_percent / 100;

        let attribute = if *damage_type == DamageType::Stamina {
            Attribute::Stamina
        } else {
            Attribute::Health
        };

        change_attribute_points(attributes,  &attribute, total_damage, false);
    }

    for effect_type in projectile.effects.iter() {
        effects.added_effect.push(effect_type.clone());   
    }

    for skill in projectile.passive_skills.iter() {
        let mut new_skill = skill.clone();
        match skills.passive_skills.get_mut(&skill.skill_type) {
            Some(v) => {
                new_skill.life_time += v.life_time; // prolong time duration;
                *v = new_skill;
            }
            None => {
                skills
                    .passive_skills
                    .insert(skill.skill_type.clone(), new_skill);
            }
        }
    }
    commands.entity(projectile_entity).despawn_recursive();
}

fn collision_with_thing(
    commands: &mut Commands,
    projectile_entity: Entity,
    projectile: &Projectile,
    attributes_component: &mut AttributesComponent,
    resists_component: &ResistsComponent,
    damage_text: &mut DamageTextComponent,
) {
    for (damage_type, damage) in projectile.damage.iter() {
        let thing_resits = match resists_component
            .resists
            .get(&get_resist_from_damage_type(damage_type))
        {
            Some(v) => *v,
            None => 0,
        };
        if *damage_type == DamageType::Stamina {
            continue;
        };

        let damage_with_resist = damage - damage * thing_resits / 100;
        match attributes_component.attributes.get_mut(&Attribute::Health) {
            Some(v) => *v -= damage_with_resist,
            None => {}
        }
        let text_damage =
            DamageTextInformer::new(damage_with_resist.to_string(), false, Some(damage_type));
        damage_text.text_upper.push(text_damage);
    }
    commands.entity(projectile_entity).despawn_recursive();
    return;
}

fn collision_with_projectile() {}

fn try_grid_move(x: f32, y: f32, position: &mut Position<i32>) -> bool {
    let projectile_grid_x = (x / TILE_SIZE as f32).round() as i32;
    let projectile_grid_y = (y / TILE_SIZE as f32).round() as i32;

    let mut bool = false;
    if position.x != projectile_grid_x {
        position.x = projectile_grid_x;
        bool = true;
    };

    if position.y != projectile_grid_y {
        position.y = projectile_grid_y;
        bool = true;
    }

    return bool;
}

pub fn create_projectile(
    commands: &mut Commands,
    material_manager: &MaterialManager,
    projectile: Projectile,
    target_position: Position<i32>,
    projectiles_value: u8,
    skill_direction: &SkillDirectionType,
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
        }
    };
    let starting_point_x = projectile.current_position.x;
    let starting_point_y = projectile.current_position.y;

    let half_arc_angle = arc / 2.0;
    let angle_coefficient = if projectiles_value == 1 {
        //each angle to cast projectile;
        0.0
    } else {
        arc / projectiles_value as f32
    };
    let delta_x = starting_point_x as f32 - target_position.x as f32; //difference between target position and starting position;
    let delta_y = starting_point_y as f32 - target_position.y as f32;
    let angle_between_ab_and_y = (delta_x.atan2(delta_y)).to_degrees(); //angle between Y and line cast to target position;
    let radius = (delta_x * delta_x + delta_y * delta_y).sqrt();

    for i in 0..projectiles_value {
        let mut new_projectile_component = projectile.clone();
        let projectile_type = &new_projectile_component.projectile_type;

        let x = starting_point_x as f32
            + radius
                * (angle_between_ab_and_y - half_arc_angle + angle_coefficient * i as f32)
                    .to_radians()
                    .sin();
        let y = starting_point_y as f32
            + radius
                * (angle_between_ab_and_y - half_arc_angle + angle_coefficient * i as f32)
                    .to_radians()
                    .cos();
        let new_delta_x = starting_point_x as f32 - x;
        let new_delta_y = starting_point_y as f32 - y;
        let distance = (new_delta_x.powf(2.0) + new_delta_y.powf(2.0)).sqrt();
        new_projectile_component.motion_coefficient.x = new_delta_x / distance;
        new_projectile_component.motion_coefficient.y = new_delta_y / distance;
        

        let new_z_position = Z_POSITION;
        let transform = Transform::from_xyz(x, y, new_z_position);
        let texture_atlas = material_manager
            .game_scene
            .projectiles
            .get_texture_atlas(projectile_type);
        commands.spawn((SpriteSheetBundle {
            texture_atlas,
            transform,
            ..Default::default()
        },
        new_projectile_component,
        ));
    }
}
