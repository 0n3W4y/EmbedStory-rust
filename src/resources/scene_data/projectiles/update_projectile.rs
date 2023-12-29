use crate::{
    components::{
        projectile_component::Projectile, thing_component::ThingComponent, IdentificationComponent,
        ObjectType, PositionComponent, TakenDamage, TakenDamageComponent,
    },
    config::TILE_SIZE,
    materials::material_manager::MaterialManager,
    scenes::game_scenes::tilemap::tile::Position,
};
use bevy::prelude::*;
use rand::Rng;

const Z_POSITION: f32 = 4.0; // fourth layer;

pub fn update_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut projectile_query: Query<(Entity, &mut Projectile, &mut Transform)>,
    mut all_query: Query<(
        &PositionComponent,
        &IdentificationComponent,
        &mut TakenDamageComponent,
        Option<&ThingComponent>,
    )>,
) {
    let delta = time.delta_seconds();
    for (projectile_entity, mut projectile, mut transform) in projectile_query.iter_mut() {
        transform.translation.x += projectile.motion_coefficient.x
            * projectile.velocity as f32
            * delta
            * projectile.direction.x as f32;
        transform.translation.y += projectile.motion_coefficient.y
            * projectile.velocity as f32
            * delta
            * projectile.direction.y as f32;

        if try_grid_move(
            transform.translation.x,
            transform.translation.y,
            &mut projectile,
        ) {
            try_collision(
                &mut commands,
                &mut projectile,
                &mut all_query,
                projectile_entity,
            );
        }
    }
}

pub fn try_collision(
    commands: &mut Commands,
    projectile: &mut Projectile,
    all_query: &mut Query<(
        &PositionComponent,
        &IdentificationComponent,
        &mut TakenDamageComponent,
        Option<&ThingComponent>,
    )>,
    projectile_entity: Entity,
) {
    let mut random = rand::thread_rng();
    let projectile_x = projectile.current_position.x;
    let projectile_y = projectile.current_position.y;
    for (
        position, 
        identification, 
        mut damage, 
        option_thing_component
    ) in all_query.iter_mut() {
        let target_x = position.position.x;
        let target_y = position.position.y;

        if target_x == projectile_x && target_y == projectile_y {
            match identification.object_type {
                ObjectType::Thing(_) => {
                    if (projectile.starting_position.x - target_x).abs() <= 1
                        && (projectile.starting_position.y - target_y).abs() <= 1
                    {
                        return;
                    }


                    if let Some(thing_component) = option_thing_component {
                        let random_number_for_checking_collision: u8 =
                            random.gen_range(0..100);
                        if random_number_for_checking_collision
                            <= thing_component.thing_defense_type.collision_chance()
                        {
                            return;
                        }
                    }

                    collision_with_thing(projectile, &mut damage);
                    commands.entity(projectile_entity).despawn_recursive();
                    return;
                }
                ObjectType::Charactor(_, _) => {
                    collision_with_charactor(projectile, &mut damage);
                    commands.entity(projectile_entity).despawn_recursive();
                    return;
                }
                ObjectType::Projectile(_) => {
                    //do_damage;
                    return;
                }
                _ => {
                    return;
                }
            }
        }
    }
}


fn collision_with_charactor(projectile: &Projectile, damage: &mut TakenDamageComponent) {
    let mut damage_taken: TakenDamage = Default::default();
    damage_taken.is_critical_hit = projectile.is_critical_hit;
    damage_taken.area_of_impact = projectile.area_on_impact;

    for (damage_type, damage) in projectile.damage.iter() {
        damage_taken.damage.insert(damage_type.clone(), *damage);
    }

    for effect in projectile.effects.iter() {
        damage_taken.effects.push(effect.clone());
    }

    for skill in projectile.passive_skills.iter() {
        damage_taken.passive_skills.push(skill.clone());
    }
    damage.damage.push(damage_taken);
}

fn collision_with_thing(projectile: &Projectile, damage: &mut TakenDamageComponent) {
    let mut damage_taken: TakenDamage = Default::default();
    damage_taken.is_critical_hit = projectile.is_critical_hit;
    damage_taken.area_of_impact = projectile.area_on_impact;
    for (damage_type, damage) in projectile.damage.iter() {
        damage_taken.damage.insert(damage_type.clone(), *damage);
    }
    damage.damage.push(damage_taken);
}

fn try_grid_move(x: f32, y: f32, projectile: &mut Projectile) -> bool {
    let direction_x = projectile.direction.x;
    let direction_y = projectile.direction.y;
    let projectile_grid_x = x / TILE_SIZE as f32;
    let projectile_grid_y = y / TILE_SIZE as f32;

    let new_grid_x = if direction_x < 0 {
        projectile_grid_x.ceil() as i32
    } else {
        projectile_grid_x.floor() as i32
    };

    let new_grid_y = if direction_y < 0 {
        projectile_grid_y.ceil() as i32
    } else {
        projectile_grid_y.floor() as i32
    };

    let mut bool = false;
    if projectile.current_position.x != new_grid_x {
        projectile.current_position.x = new_grid_x;
        bool = true;
    };

    if projectile.current_position.y != new_grid_y {
        projectile.current_position.y = new_grid_y;
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
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas,
            transform,
            ..Default::default()
        },
        projectile,
    ));
}
