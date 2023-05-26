use bevy::prelude::*;

use crate::config::TILE_SIZE;
use crate::components::charactor_component::CharactorComponent;
use crate::resources::scene_data::objects::charactor::CharactorStatus;
use crate::scenes::game_scenes::game_scene::GameScene;
use crate::scenes::game_scenes::tilemap::tile::{Position, TilePermissions};
use crate::resources::scene_data::objects::charactor::skills::Skill;
use crate::scenes::game_scenes::tilemap::tile::Tile;
use crate::resources::scene_manager::SceneManager;

const DEFAULT_MOVEMENT_SPEED: u16 = 1000;

pub fn move_charactor(
    time: Res<Time>,
    mut charactor_query: Query<(&mut CharactorComponent, &mut Transform), With<CharactorComponent>>,
    scene_manager: Res<SceneManager>,
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

        let position_x = component.position.x;
        let position_y = component.position.y;
        let destination_x = component.destination_point.x;
        let destination_y = component.destination_point.y;

        let scene = scene_manager.get_current_game_scene();

        if component.destination_path.len() == 0 {
            try_path(position_x, position_y, destination_x, destination_y, &mut component.destination_path, scene);
        }
        

        
        
        
        if check_next_grid_position_for_moving(tile) {

        };

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

fn check_tile_for_moving(tile: &Tile) -> bool {
    match tile.permissions.iter().find(|x|{x == &&TilePermissions::Walk}){
        Some(_) => true,
        None => false,
    }
}

fn try_path(x: i32, y: i32, dis_x: i32, dis_y: i32, path_vec: &mut Vec<Position<i32>>, scene: &GameScene) {
    let path_tiles = ((dis_x - x).abs()).max((dis_y - y).abs());
    for _ in 0..path_tiles{

    }
    let direction_xy = calculate_direction(x, y, dis_x, dis_y);
    let tile = scene.tilemap.get_tile_by_position(x + direction_xy.0, y + direction_xy.1);
    if check_tile_for_moving(tile) {
        path_vec.push(Position {x: tile.position.x, y: tile.position.y });
    } else {

    }

}



