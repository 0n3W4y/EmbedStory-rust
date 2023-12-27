use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::components::IdentificationComponent;
use crate::components::ObjectType;
use crate::components::PositionComponent;
use crate::components::charactor_component::ActionType;
use crate::components::charactor_component::CharactorTargetComponent;
use crate::components::charactor_component::PlayerComponent;
use crate::components::tile_component::TileComponent;
use crate::config::{MONITOR_HALF_HEIGHT, MONITOR_HALF_WIDTH, TILE_SIZE};
use crate::plugins::camera::Orthographic2DCamera;
//use crate::resources::scene_manager::SceneManager;
use crate::scenes::game_scenes::tilemap::tile::Position;

use super::CharactorType;

pub fn player_click(
    primary_query: Query<&Window, With<PrimaryWindow>>,
    mouse_button_input: Res<Input<MouseButton>>,
    //scene_manager: Res<SceneManager>,
    mut player_query: Query<
        (&mut CharactorTargetComponent, &mut PositionComponent),
        With<PlayerComponent>,
    >,
    target_query: Query<(&IdentificationComponent, &PositionComponent), Without<TileComponent>>,
    camera: Query<(&Transform, &OrthographicProjection), With<Orthographic2DCamera>>,
) {
    let (
        mut player_target, 
        mut destination_component
    ) = player_query.single_mut();
    
    let Ok(window) = primary_query.get_single() else {
        return;
    };
    //let scene = scene_manager.get_current_game_scene();

    if mouse_button_input.just_released(MouseButton::Left) {
        if let Some(_position) = window.cursor_position() {
            let x = _position.x;
            let y = _position.y;
            let (transform, projection) = camera.single();
            let cam_x = transform.translation.x;
            let cam_y = transform.translation.y;
            let camera_scale = projection.scale;

            //let position_x:i8 = ((( x - MONITOR_HALF_WIDTH as f32 + cam_x ) * camera_scale )  / TILE_SIZE as f32 ).round() as i8;
            //let position_y:i8 = ((( y - MONITOR_HALF_HEIGHT as f32 + cam_y ) * camera_scale )  / TILE_SIZE as f32 ).round() as i8;
            let position_x: i32 = ((x + cam_x / camera_scale - MONITOR_HALF_WIDTH as f32)
                / (TILE_SIZE as f32 / camera_scale))
                .round() as i32;
            let position_y: i32 = ((y + cam_y / camera_scale - MONITOR_HALF_HEIGHT as f32)
                / (TILE_SIZE as f32 / camera_scale))
                .round() as i32;

            for (target_identification, target_position) in target_query.iter(){
                let target_x = target_position.position.x;
                let target_y = target_position.position.y;
                if target_x == position_x && target_y == position_y {
                    match target_identification.object_type {
                        ObjectType::Charactor(v) => {
                            match v {
                                CharactorType::Player => {
                                    println!("Clicked on Player");
                                    move_player_to_position(&mut destination_component, position_x, position_y);
                                    return;
                                },
                                CharactorType::NPC => {
                                    println!("Clicked on NPC");
                                    //select_target_to_talk(&mut palyer_target, charactor_component.id, position_x, position_y);
                                    return;
                                },
                                CharactorType::Monster => {
                                    println!("Clicked on Monstger");
                                    select_target_to_attack(&mut player_target, target_identification.id, position_x, position_y);
                                    return;
                                },
                                CharactorType::Companion => {
                                    println!("Clicked on Companion");
                                    move_player_to_position(&mut destination_component, position_x, position_y);
                                    return;
                                },
                            }
                        },
                        ObjectType::Stuff => {
                            move_player_to_position(&mut destination_component, position_x, position_y);
                            return;
                        },
                        ObjectType::Thing => {
                            println!("Clicked on Tile or Prijectile");
                            //TODO: for thing we need to set target and action type;
                            move_player_to_position(&mut destination_component, position_x, position_y);
                            return;
                        },
                        ObjectType::Projectile | ObjectType::Tile => {
                            println!("Clicked on Prijectile");
                            move_player_to_position(&mut destination_component, position_x, position_y);
                            return;
                        },
                    };
                };
            };
            println!("Clicked on Tile");
            move_player_to_position(&mut destination_component, position_x, position_y);            
        } else {
            // cursor is not inside the window
        }
    }
}

fn move_player_to_position(
    destination: &mut PositionComponent,
    x: i32,
    y: i32,
) {
    destination.destination_point = Some(Position { x, y });
    if let Some(first) = destination.destination_path.first().cloned() {    
        destination.destination_path.retain(|&x| x == first);                   //remove all values in destination path, but keep only first one;
    }
}

fn select_target_to_attack(
    player_target: &mut CharactorTargetComponent,
    id: usize,
    x: i32,
    y: i32,
) {
    player_target.target = Some(id);
    player_target.action = ActionType::Attack;
    player_target.target_position = Some(Position{ x, y });
}
