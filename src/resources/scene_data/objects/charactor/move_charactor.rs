use bevy::prelude::*;

use crate::config::TILE_SIZE;
use crate::components::charactor_component::CharactorComponent;
use crate::resources::scene_data::objects::charactor::CharactorStatus;
use crate::scenes::game_scenes::tilemap::tile::Position;

const DEFAULT_MOVEMENT_SPEED: u16 = 1000;

pub fn move_charactor(
    time: Res<Time>,
    mut charactor_query: Query<(&mut CharactorComponent, &mut Transform), With<CharactorComponent>>,
){
    for (mut component, transform) in charactor_query.iter_mut(){
        if component.status != CharactorStatus::Moving {
            continue;
        };

        let movement_speed: u16 = match component.skills.get(&Skill::Movement){
            Some(v) => *v,
            None => {
                println!(
                    "Can't get movement speed skill from charactor with id: '{:?}', type: '{:?}', subtype: '{:?}'. Use default 1000", 
                    component.id, 
                    component.charactor_type,
                    component.charactor_subtype,
                );
                DEFAULT_MOVEMENT_SPEED
            }
        };

        let direction_xy = calculate_direction(component.position.x, component.position.y, component.destination_point.x, component.destination_point.y);

        let sprite_x = transform.translation.x + (direction_xy.0 as f32 * movement_speed as f32 * time.delta_seconds());
        let sprite_y = transform.translation.y + (direction_xy.1 as f32 * movement_speed as f32 * time.delta_seconds());

        try_grid_moving(&mut component.position, sprite_x, sprite_y, direction_xy);
        if check_destination_reach(&component.position, &component.destination_point) {
            component.status = CharactorStatus::Standing;
        }
    }
}

fn calculate_direction(position_x: i32, position_y: i32, destination_x: i32, destination_y: i32) -> (i8, i8) {
    let direction_x = destination_x - position_x;
    let direction_y = destination_y - position_y;

    let x = if direction_x > 0 {
        1
    } else if direction_x < 0 {
        -1
    } else {
        0
    };

    let y = if direction_y > 0 {
        1
    } else if direction_y < 0 {
        -1
    } else {
        0
    };

    return (x, y);
}

fn check_destination_reach(position: &Position<i32>, destination: &Position<i32>) -> bool {
    if position.x == destination.x && position.y == destination.y {
        true
    } else {
        false
    }
}

fn try_grid_moving(position: &mut Position<i32>, x: f32, y: f32, direction: (i8, i8)){
    let grid_x = x / TILE_SIZE as f32;
    let grid_y = y / TILE_SIZE as f32;

    let calculated_x = if direction.0 > 0 {
        grid_x.floor() as i32
    } else {
        grid_x.ceil() as i32
    };

    let calculated_y = if direction.1 > 0 {
        grid_y.floor() as i32
    } else {
        grid_x.ceil() as i32
    };

    if position.x != calculated_x {
        position.x = calculated_x;
    };

    if position.x != calculated_y {
        position.y = calculated_y;
    };
}



