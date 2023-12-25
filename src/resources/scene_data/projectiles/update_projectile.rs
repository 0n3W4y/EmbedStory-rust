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
            charactor::
                change_attribute_points,
            damage_text_informer::{DamageTextInformer, TextDamageType}, Ability, get_resist_from_damage_type, Attribute,
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
            projectile.motion_coefficient.x * projectile.velocity as f32 * delta * projectile.direction.x as f32;
        transfrom.translation.y +=
            projectile.motion_coefficient.y * projectile.velocity as f32 * delta * projectile.direction.y as f32;
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
                    let starting_pos_x = projectile.starting_position.x;
                    let starting_pos_y = projectile.starting_position.y;
                    let delta_x: i32 = target_x - starting_pos_x;
                    let delta_y: i32 = target_y - starting_pos_y;
                    let distance: i32 = (((delta_x as f32).powf(2.0) + (delta_y as f32).powf(2.0)).sqrt()).floor() as i32;

                    if distance == 1 {                                                                          //ignoring any object at +-1 grid position ( thinking, charactor shooting from defense)
                        continue;
                    }
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
    match abilities.ability.get(&Ability::Evasion) {
        //check for evade
        Some(v) => {
            let random_evade_chance: i16 = random.gen_range(0..=99);
            if *v > random_evade_chance {
                //evaded
                damage_text.text_upper.push(DamageTextInformer::new(
                    0,
                    Some(TextDamageType::Evaded),
                    false,
                    None,
                ));
                commands.entity(projectile_entity).despawn_recursive();
                return;
            }
        }
        None => {}
    }

    let block_amount_percent = match abilities.ability.get(&Ability::BlockChance) {
        //check for block and amount;
        Some(v) => {
            let random_block_chance: i16 = random.gen_range(0..=99);
            if *v > random_block_chance {
                //blocked some damage;
                match abilities.ability.get(&Ability::BlockAmount) {
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

        change_attribute_points(attributes,  &Attribute::Health, total_damage, false);
    }

    for effect in projectile.effects.iter() {
        effects.added_effect.push(effect.clone());
    }

    for skill in projectile.passive_skills.iter() {
        skills.added_passive_skills.push(skill.clone());
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

        let damage_with_resist = damage - damage * thing_resits / 100;
        match attributes_component.attributes.get_mut(&Attribute::Health) {
            Some(v) => *v -= damage_with_resist,
            None => {}
        }
        let text_damage =
            DamageTextInformer::new(damage_with_resist, None, false, Some(damage_type));
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
    mut projectile: Projectile,
    target_position: Position<i32>,
) {
    let starting_point_x = projectile.starting_position.x;
    let starting_point_y = projectile.starting_position.y;

    let delta_x = target_position.x - starting_point_x;
    let delta_y = target_position.y - starting_point_y;

    projectile.direction.x = if delta_x < 0 {
        -1
    } else if delta_x > 0 {
        1
    } else {
        0
    };

    projectile.direction.y = if delta_y < 0 {
        -1
    } else if delta_y > 0 {
        1
    } else {
        0
    };

    let distance = ((delta_x as f32).powf(2.0) + (delta_y as f32).powf(2.0)).sqrt();
    projectile.motion_coefficient.x = delta_x as f32 / distance;
    projectile.motion_coefficient.y = delta_y as f32 / distance;
    let x = starting_point_x as f32 * TILE_SIZE as f32;
    let y = starting_point_y as f32 * TILE_SIZE as f32;
    let new_z_position = Z_POSITION;
    let transform = Transform::from_xyz(x, y, new_z_position);
    let texture_atlas = material_manager
        .game_scene
        .projectiles
        .get_texture_atlas(&projectile.projectile_type);
    commands.spawn((SpriteSheetBundle {
        texture_atlas,
        transform,
        ..Default::default()
    },
    projectile,
    ));
}
