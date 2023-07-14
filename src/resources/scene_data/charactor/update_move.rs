use bevy::prelude::*;

use crate::config::TILE_SIZE;
use crate::components::charactor_component::{CharactorComponent, PositionComponent, AbilityComponent, PlayerComponent};
use crate::resources::scene_data::charactor::CharactorStatus;
use crate::scenes::game_scenes::game_scene::GameScene;
use crate::scenes::game_scenes::tilemap::tile::{Position, TilePermissions};
use crate::scenes::game_scenes::tilemap::tile::Tile;
use crate::resources::scene_manager::SceneManager;

use super::abilities::Ability;

//use crate::plugins::camera::Orthographic2DCamera;

//use super::CharactorType;

const DEFAULT_MOVEMENT_SPEED: f32 = 1000.0;

pub fn move_charactor(
    time: Res<Time>,
    mut charactor_query: Query<(&mut CharactorComponent, &mut PositionComponent, &AbilityComponent, &mut Transform, &mut TextureAtlasSprite), Without<PlayerComponent>>,
    //mut camera: Query<(&mut Transform, &mut Orthographic2DCamera, &OrthographicProjection), With<Orthographic2DCamera>>,
    scene_manager: Res<SceneManager>,
){
    let delta = time.delta_seconds();
    let scene = scene_manager.get_current_game_scene();
    for (
        mut charactor, 
        mut position, 
        ability,
        mut transform, 
        mut sprite
    ) in charactor_query.iter_mut(){
        //check for need to move;
        match position.destination_point {
            Some(_) => {charactor.status = CharactorStatus::Moving;},
            None => continue,
        }
        //let mut move_camera_on_player: bool = false;
        //let (mut camera_transform, cam, projection) = camera.single_mut();
        //if component.charactor_type == CharactorType::Player && cam.camera_on_charator {move_camera_on_player = true;};

        if position.destination_path.len() == 0 {
            try_path(&mut position, scene);
            // TODO: Pathfinding;
            if position.position.x == position.destination_point.unwrap().x //safe unwrap
            && position.position.y == position.destination_point.unwrap().y { //safe unwrap
                destination_reach(&mut charactor);
                continue;
            }
            change_sprite_by_direction(&mut sprite, &position.destination_direction);
        }

        let movement_speed: f32 = match ability.ability.get(&Ability::MovementSpeed){
            Some(v) => *v,
            None => {
                println!(
                    "Can't get movement speed ability from charactor with id: '{:?}', type: '{:?}'. Use default 1000", 
                    charactor.id, 
                    charactor.charactor_type
                );
                DEFAULT_MOVEMENT_SPEED
            }
        }; 

        let new_x = position.destination_direction.x as f32 * movement_speed as f32 * delta;
        let new_y = position.destination_direction.y as f32 * movement_speed as f32 * delta;

        transform.translation.x += new_x;
        transform.translation.y += new_y;

        //if move_camera_on_player {
        //    let projection_scale = projection.scale;
        //    let cam_x = new_x * projection_scale;
        //    let cam_y = new_y * projection_scale;
    
        //    camera_transform.translation.x += new_x;
        //    camera_transform.translation.y += new_y;
        //}
        
        try_grid_moving(&mut charactor, &mut position, &mut transform.translation, &mut sprite);
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

fn destination_reach(charactor: &mut CharactorComponent){
    charactor.status = CharactorStatus::Standing;
}

fn try_grid_moving(charactor: &mut CharactorComponent, position: &mut PositionComponent, translation: &mut Vec3, sprite: &mut Mut<TextureAtlasSprite>){
    let grid_x = translation.x / TILE_SIZE as f32;
    let grid_y = translation.y / TILE_SIZE as f32;

    let dir_x = position.destination_direction.x;
    let dir_y = position.destination_direction.y;

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

    if position.position.x != calculated_x {
        position.position.x = calculated_x;
        grid_move = true;
    };

    if position.position.y != calculated_y {
        position.position.y = calculated_y;
        grid_move = true;
    };

    if grid_move {
        position.destination_path.remove(0); // remove reached grid point;

        if position.destination_path.len() != 0 {
            let next_point_x = position.destination_path[0].x;
            let next_point_y = position.destination_path[0].y;
            let direction_xy = calculate_direction(calculated_x, calculated_y, next_point_x, next_point_y);
            position.destination_direction.x = direction_xy.x;
            position.destination_direction.y = direction_xy.y;
            change_sprite_by_direction(sprite, &position.destination_direction);
        }else{
            destination_reach(charactor);
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

fn try_path(position: &mut PositionComponent, scene: &GameScene) {
    let position_x = position.position.x;
    let position_y = position.position.y;  
    let destination_x = position.destination_point.unwrap().x; //safe
    let destination_y = position.destination_point.unwrap().y; //safe

    let path_tiles = ((destination_x - position_x).abs()).max((destination_y - position_y).abs());
    for _ in 0..path_tiles{
        let mut current_x = position_x;
        let mut current_y = position_y;
        let path_len = position.destination_path.len();
        if path_len != 0 { 
            //last index with position;
            current_x = position.destination_path[path_len -1].x;
            current_y = position.destination_path[path_len -1].y;
        };

        let direction_xy = calculate_direction(current_x, current_y, destination_x, destination_y);
        if position.destination_direction.x == 0 && position.destination_direction.y == 0 { // first circle run;
            position.destination_direction.x = direction_xy.x;
            position.destination_direction.y = direction_xy.y;
        };

        let tile = scene.tilemap.get_tile_by_position(current_x + direction_xy.x as i32, current_y + direction_xy.y as i32);

        if check_tile_for_moving(tile) {
            position.destination_path.push(Position {x: tile.position.x, y: tile.position.y});
        } else {
            return;
        }
    }
}



