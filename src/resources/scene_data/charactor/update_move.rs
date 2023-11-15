use bevy::prelude::*;

use crate::components::{PositionComponent, IdenteficationComponent};
use crate::config::TILE_SIZE;
use crate::components::charactor_component::{CharactorComponent, AbilityComponent, DestinationComponent};
use crate::resources::scene_data::charactor::CharactorStatus;
use crate::scenes::game_scenes::game_scene::GameScene;
use crate::scenes::game_scenes::tilemap::tile::{Position, TilePermissions};
use crate::scenes::game_scenes::tilemap::tile::Tile;
use crate::resources::scene_manager::SceneManager;

use super::CharactorType;
use super::abilities::AbilityType;

//use crate::plugins::camera::Orthographic2DCamera;

//use super::CharactorType;

const DEFAULT_MOVEMENT_SPEED: f32 = 100.0;

pub fn move_charactor(
    time: Res<Time>,
    mut charactor_query: Query<(&IdenteficationComponent, &mut CharactorComponent, &mut PositionComponent, &mut DestinationComponent, &AbilityComponent, &mut Transform, &mut TextureAtlasSprite)>,
    //mut camera: Query<(&mut Transform, &mut Orthographic2DCamera, &OrthographicProjection), With<Orthographic2DCamera>>,
    scene_manager: Res<SceneManager>,
){
    let delta = time.delta_seconds();
    let scene = scene_manager.get_current_game_scene();
    for (
        identification_component,
        mut charactor, 
        mut position, 
        mut destination,
        ability,
        mut transform, 
        mut sprite
    ) in charactor_query.iter_mut(){
        match destination.destination_point {
            Some(v) => {
                try_move(
                    identification_component,
                    &mut charactor, 
                    &mut position,
                    &mut destination,
                    ability,
                    &mut transform.translation,
                    delta,
                    scene
                );               // check for moving and create path;
            },
            None => continue,                                                                   //skip
        }
        if charactor.charactor_type == CharactorType::Player {                                  // this feature function for camera to stalk player
            //let mut move_camera_on_player: bool = false;
            //let (mut camera_transform, cam, projection) = camera.single_mut();
            //if component.charactor_type == CharactorType::Player && cam.camera_on_charator {move_camera_on_player = true;};

            //if move_camera_on_player {
            //    let projection_scale = projection.scale;
            //    let cam_x = new_x * projection_scale;
            //    let cam_y = new_y * projection_scale;
        
            //    camera_transform.translation.x += new_x;
            //    camera_transform.translation.y += new_y;
            //}
        };
    }
}

//first click on ground;
pub fn try_move(
    identification_component: &IdenteficationComponent, 
    charactor: &mut CharactorComponent, 
    position: &mut PositionComponent, 
    destination: &mut DestinationComponent, 
    ability: &AbilityComponent, 
    translation: &mut Vec3, 
    delta: f32,
    scene: &GameScene
) {
    if destination.destination_path.len() == 0 {                                        //first move after standing or reach his destination;
        try_path(position, destination, scene);                                   //light pathfinding;
    } else if destination.destination_path.len() == 1 {                                 //check for changing destination point;
        if destination.destination_path[0] != destination.destination_point.unwrap() {  //safe unwrap;
            try_path(position, destination, scene);
        }      
    } else {
        moving(
            identification_component,
            charactor, 
            position,
            destination,
            ability,
            translation,
            delta
        );
    }

}

pub fn moving(
    identification_component: &IdenteficationComponent, 
    charactor: &mut CharactorComponent, 
    position: &mut PositionComponent, 
    destination: &mut DestinationComponent, 
    ability: &AbilityComponent, 
    translation: &mut Vec3, 
    delta: f32
){
    //get charactor movement multiplier
    // if we have -100% charactor can't move at all;
    let movement_speed_multiplier = match ability.ability.get(&AbilityType::MovementSpeed){
        Some(v) => *v,
        None => {
            println!(
                "Can't get movement speed ability on charactor {:?}, {:?} . Use default 0",
                charactor.charactor_type,
                identification_component.id,
            );
            0
        }
    };

    let mut movement_speed = DEFAULT_MOVEMENT_SPEED + DEFAULT_MOVEMENT_SPEED * movement_speed_multiplier as f32 / 100.0;
    if movement_speed < 0.0 {
        movement_speed = 0.0;
    };

    let new_x = destination.destination_direction.x as f32 * movement_speed as f32 * delta;
    let new_y = destination.destination_direction.y as f32 * movement_speed as f32 * delta;

    translation.x += new_x;
    translation.y += new_y;

    change_moving_status_by_direction(charactor, &destination.destination_direction);

    try_grid_moving(charactor, position, destination, translation);
}

fn calculate_and_set_direction(path: &Vec<Position<i32>>, direction: &mut Position<i8>) {
    direction.x = (path[1].x - path[0].x) as i8;
    direction.y = (path[1].y - path[0].y) as i8;
}

fn change_moving_status_by_direction(charactor: &mut CharactorComponent, direction: &Position<i8>){
    if direction.x == 0 && direction.y > 0 {                //up 
        charactor.status = CharactorStatus::MovingUp;
    } else if direction.x == 0 && direction.y < 0 {          // down and by default; when standing; 
        charactor.status = CharactorStatus::MovingDown;
    } else if direction.x < 0 && (direction.y < 0 || direction.y > 0) {            //left
        charactor.status = CharactorStatus::MovingLeft;
    } else {                                                //right
        charactor.status = CharactorStatus::MovingRight;
    };
}

/*                                  //this function must be on animation_handler;
fn change_sprite_by_direction(sprite: &mut Mut<TextureAtlasSprite>, direction: &Position<i8>){
    if direction.x >= 0 && direction.y > 0
    || direction.x <= 0 && direction.y > 0 {                //up 
        sprite.index = 1;
        sprite.flip_x = false;
    } else if direction.x >= 0 && direction.y < 0           // down and by default; when standing;
    || direction.x <= 0 && direction.y < 0
    || direction.x == 0 && direction.y == 0 {
        sprite.index = 0;
        sprite.flip_x = false;
    } else if direction.x == 0 && direction.y < 0 {            //left
        sprite.index = 2;
        sprite.flip_x = false;
    } else {                                                //right
        sprite.index = 2;
        sprite.flip_x = true;
    };
}
*/
fn destination_reach(charactor: &mut CharactorComponent, destination: &mut DestinationComponent){
    charactor.status = CharactorStatus::Standing;
    destination.destination_direction = Position {x: 0 as i8, y: 0 as i8};
    destination.destination_point = None;
    destination.destination_path.clear();
}

fn try_grid_moving(charactor: &mut CharactorComponent, position: &mut PositionComponent, destination: &mut DestinationComponent, translation: &mut Vec3){
    let grid_x = translation.x / TILE_SIZE as f32;
    let grid_y = translation.y / TILE_SIZE as f32;

    let dir_x = destination.destination_direction.x;
    let dir_y = destination.destination_direction.y;

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

    let current_destination_x = destination.destination_path[0].x;
    let current_destination_y = destination.destination_path[0].y;

    //reach destination point in path;
    if current_destination_x == calculated_x && current_destination_y == calculated_y {
        //set new grid position;
        position.position.x = current_destination_x;
        position.position.y = current_destination_y;
        //remove first path point;
        destination.destination_path.remove(0);

        //check for reach destination;
        if destination.destination_path.len() == 0 {
            destination_reach(charactor, destination);
            //centering sprite ;
            translation.x = grid_x * TILE_SIZE as f32;
            translation.y = grid_y * TILE_SIZE as f32;
        } else {
            calculate_and_set_direction(&destination.destination_path, &mut destination.destination_direction);
        }   
    }
}



fn try_path(position: &mut PositionComponent, destination: &mut DestinationComponent, scene: &GameScene) {
    //pathfinding need to be here;
    let starting_position_x = position.position.x;
    let starting_position_y = position.position.y;  
    let destination_x = destination.destination_point.unwrap().x; //safe
    let destination_y = destination.destination_point.unwrap().y; //safe

    //maximum tiles to reaach destination without pathfinding;
    let path_tiles = ((destination_x - starting_position_x).abs()).max((destination_y - starting_position_y).abs());
    for i in 0..path_tiles{
        let path_len = destination.destination_path.len();
        let current_position = if path_len == 0 {
            //get start point;
            Position{x: starting_position_x, y: starting_position_y}
        } else {
            //get last point;
            Position{
                x: destination.destination_path[path_len -1].x,
                y: destination.destination_path[path_len -1].y
            }
        };
        let dif_x = destination_x - current_position.x;
        let dif_y = destination_y - current_position.y;

        let direction_x = if dif_x > 0 {
            1
        } else if dif_x < 0 {
            -1
        } else {
            0
        };

        let direction_y = if dif_y > 0 {
            1
        } else if dif_y < 0 {
            -1 
        } else {
            0
        };

        let next_position = Position{x: current_position.x + direction_x, y: current_position.y + direction_y};
        let tile = scene.tilemap.get_tile_by_position(next_position.x, next_position.y);

        if check_tile_for_moving(tile) {
            destination.destination_path.push(next_position);
        } else {
            //break circle;
            //break if tile can't walked, without pathfinding;
            if path_len == 0 {
                destination.destination_point = None;
                return;
            } else {
                destination.destination_point = Some(destination.destination_path[path_len - 1]);
                return;
            }            
        }
    }
}

fn check_tile_for_moving(tile: Option<&Tile>) -> bool {
    match tile {
        Some(v) => {
            match v.permissions.iter().find(|x|{x == &&TilePermissions::Walk}){
                Some(_) => true,
                None => false,
            }
        },
        None => false,
    }
    
}



