use crate::{
    components::{
        charactor_component::{AbilityComponent, EffectComponent, SkillComponent},
        projectile_component::Projectile,
        tile_component::TileComponent,
        AttributesComponent, IdentificationComponent, ObjectType, PositionComponent,
        ResistsComponent, TakenDamage, TakenDamageComponent,
    },
    config::TILE_SIZE,
    materials::material_manager::MaterialManager,
    scenes::game_scenes::tilemap::tile::Position,
};
use bevy::prelude::*;

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
            &mut TakenDamageComponent,
            //&mut Projectile,
        ),
        Without<TileComponent>,
    >,
) {
    let delta = time.delta_seconds();
    for (projectile_entity, mut projectile, mut transfrom) in projectile_query.iter_mut() {
        transfrom.translation.x += projectile.motion_coefficient.x
            * projectile.velocity as f32
            * delta
            * projectile.direction.x as f32;
        transfrom.translation.y += projectile.motion_coefficient.y
            * projectile.velocity as f32
            * delta
            * projectile.direction.y as f32;
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
            &mut TakenDamageComponent,
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
        mut damage,
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
                        &mut damage,
                    );
                }
                ObjectType::Thing => {
                    let starting_pos_x = projectile.starting_position.x;
                    let starting_pos_y = projectile.starting_position.y;
                    let delta_x: i32 = target_x - starting_pos_x;
                    let delta_y: i32 = target_y - starting_pos_y;
                    let distance: i32 = (((delta_x as f32).powf(2.0) + (delta_y as f32).powf(2.0))
                        .sqrt())
                    .floor() as i32;

                    if distance == 1 {
                        //ignoring any object at +-1 grid position ( thinking, charactor shooting from defense)
                        continue;
                    }
                    collision_with_thing(
                        commands,
                        projectile_entity,
                        projectile,
                        &mut attributes_component,
                        &resists_component,
                        &mut damage,
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
    damage: &mut TakenDamageComponent,
) {
    let mut damage_taken: TakenDamage = Default::default();
    damage_taken.is_critical_hit = projectile.is_critical_hit;

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
    commands.entity(projectile_entity).despawn_recursive();
}

fn collision_with_thing(
    commands: &mut Commands,
    projectile_entity: Entity,
    projectile: &Projectile,
    attributes_component: &mut AttributesComponent,
    resists_component: &ResistsComponent,
    damage: &mut TakenDamageComponent,
) {
    let mut damage_taken: TakenDamage = Default::default();
    damage_taken.is_critical_hit = projectile.is_critical_hit;
    for (damage_type, damage) in projectile.damage.iter() {
        damage_taken.damage.insert(damage_type.clone(), *damage);
    }
    damage.damage.push(damage_taken);
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
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas,
            transform,
            ..Default::default()
        },
        projectile,
    ));
}
