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
    mut charactor_query: Query<(&mut CharactorComponent, &mut Transform, &mut TextureAtlasSprite), With<CharactorComponent>>,
    scene_manager: Res<SceneManager>,
){
    for (mut component, mut transform, mut sprite) in charactor_query.iter_mut(){
        if component.status != CharactorStatus::Moving {
            continue;
        };
        let scene = scene_manager.get_current_game_scene();

        if component.destination_path.len() == 0 {
            try_path(&mut component, scene);
            //after fucntion to create path we can't create a path - end moving, char reached his destination // TODO: Pathfinding;
            if component.destination_path.len() == 0 {
                destination_reach(&mut component);
                continue;
            }
            change_sprite_by_direction(&mut sprite, &component.destination_direction);
        }

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

        let d_x = component.destination_direction.x as f32 * movement_speed as f32;
        let d_y = component.destination_direction.y as f32 * movement_speed as f32;

        transform.translation.x += d_x * time.delta_seconds();
        transform.translation.y += d_y * time.delta_seconds();

        
        try_grid_moving(&mut component, &mut transform.translation, &mut sprite);
    }
}

fn calculate_direction(position_x: i32, position_y: i32, destination_x: i32, destination_y: i32) -> Position<i8> {
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

    return Position{x, y};
}

fn change_sprite_by_direction(sprite: &mut Mut<TextureAtlasSprite>, direction: &Position<i8>){
    if direction.x >= 0 && direction.y > 0
    || direction.x <= 0 && direction.y > 0 {
        //up 
        sprite.index = 1;
        sprite.flip_x = false;
    } else if direction.x >= 0 && direction.y < 0 
    || direction.x <= 0 && direction.y < 0
    || direction.x == 0 && direction.y == 0 {
        // down
        sprite.index = 0;
        sprite.flip_x = false;
    } else if direction.x == 0 && direction.y < 0 {
        // left
        sprite.index = 2;
        sprite.flip_x = false;
    } else {
        sprite.index = 2;
        sprite.flip_x = true;
    };
}

fn destination_reach(component: &mut CharactorComponent){
    component.status = CharactorStatus::Standing;
}

fn try_grid_moving(component: &mut CharactorComponent, translation: &mut Vec3, sprite: &mut Mut<TextureAtlasSprite>){
    let grid_x = translation.x / TILE_SIZE as f32;
    let grid_y = translation.y / TILE_SIZE as f32;

    let dir_x = component.destination_direction.x;
    let dir_y = component.destination_direction.y;

    let calculated_x = if dir_x > 0 {
        grid_x.floor() as i32
    } else {
        grid_x.ceil() as i32
    };

    let calculated_y = if dir_y > 0 {
        grid_y.floor() as i32
    } else {
        grid_x.ceil() as i32
    };

    let mut grid_move:bool = false;

    if component.position.x != calculated_x {
        component.position.x = calculated_x;
        grid_move = true;
    };

    if component.position.y != calculated_y {
        component.position.y = calculated_y;
        grid_move = true;
    };

    if grid_move {
        component.destination_path.remove(0); // remove reached grid point;

        if component.destination_path.len() != 0 {
            let next_point_x = component.destination_path[0].x;
            let next_point_y = component.destination_path[0].y;
            let direction_xy = calculate_direction(calculated_x, calculated_y, next_point_x, next_point_y);
            component.destination_direction.x = direction_xy.x;
            component.destination_direction.y = direction_xy.y;
            change_sprite_by_direction(sprite, &component.destination_direction);
        }else{
            destination_reach(component);
            translation.x = grid_x * TILE_SIZE as f32;
            translation.y = grid_y * TILE_SIZE as f32;
            sprite.index = 0;
            sprite.flip_x = false;
        }

    } 
}

fn check_tile_for_moving(tile: &Tile) -> bool {
    match tile.permissions.iter().find(|x|{x == &&TilePermissions::Walk}){
        Some(_) => true,
        None => false,
    }
}

fn try_path(component: &mut CharactorComponent, scene: &GameScene) {
    let position_x = component.position.x;
    let position_y = component.position.y;  
    let destination_x = component.destination_point.x;
    let destination_y = component.destination_point.y;

    let path_tiles = ((destination_x - position_x).abs()).max((destination_y - position_y).abs());
    for _ in 0..path_tiles{
        let mut current_x = position_x;
        let mut current_y = position_y;
        let path_len = component.destination_path.len();
        if path_len != 0 { 
            //last index with position;
            current_x = component.destination_path[path_len -1].x;
            current_y = component.destination_path[path_len -1].y;
        };

        let direction_xy = calculate_direction(current_x, current_y, destination_x, destination_y);
        if component.destination_direction.x == 0 && component.destination_direction.y == 0 { // first circle run;
            component.destination_direction.x = direction_xy.x;
            component.destination_direction.y = direction_xy.y;
        };

        let tile = scene.tilemap.get_tile_by_position(current_x + direction_xy.x as i32, current_y + direction_xy.y as i32);

        if check_tile_for_moving(tile) {
            component.destination_path.push(Position {x: tile.position.x, y: tile.position.y});
        } else {
            return;
        }
    }
}



