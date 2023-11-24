use bevy::prelude::*;

use crate::components::IdentificationComponent;
use crate::components::ObjectType;
use crate::components::PositionComponent;
use crate::components::charactor_component::ActionType;
use crate::components::charactor_component::CharactorTargetComponent;
use crate::components::charactor_component::DestinationComponent;
use crate::components::charactor_component::PlayerComponent;
use crate::config::{MONITOR_HALF_HEIGHT, MONITOR_HALF_WIDTH, TILE_SIZE};
use crate::plugins::camera::Orthographic2DCamera;
use crate::resources::scene_manager::SceneManager;
use crate::scenes::game_scenes::tilemap::tile::Position;

use super::CharactorType;

pub fn player_click(
    windows: Res<Windows>,
    mouse_button_input: Res<Input<MouseButton>>,
    scene_manager: Res<SceneManager>,
    mut player_query: Query<
        (&mut PositionComponent, &mut CharactorTargetComponent, &mut DestinationComponent),
        With<PlayerComponent>,
    >,
    target_query: Query<(&IdentificationComponent, &PositionComponent)>,
    camera: Query<(&Transform, &OrthographicProjection), With<Orthographic2DCamera>>,
) {
    let (
        mut position, 
        mut player_target, 
        mut destination_component
    ) = player_query.single_mut();
    let window = windows.get_primary().unwrap();
    //let scene = scene_manager.get_current_game_scene();

    if mouse_button_input.just_released(MouseButton::Left) {
        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;
        let mut cam_x: f32 = 0.0;
        let mut cam_y: f32 = 0.0;
        let mut camera_scale: f32 = 0.0;

        if let Some(_position) = window.cursor_position() {
            x = _position.x;
            y = _position.y;
            for (transform, projection) in camera.iter() {
                cam_x = transform.translation.x;
                cam_y = transform.translation.y;
                camera_scale = projection.scale;
            }
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
                                },
                                CharactorType::NPC => {
                                    println!("Clicked on NPC");
                                    //select_target_to_talk(&mut palyer_target, charactor_component.id, position_x, position_y);
                                },
                                CharactorType::Monster(_) => {
                                    println!("Clicked on Monstger");
                                    select_target_to_attack(&mut player_target, target_identification.id, position_x, position_y);
                                    return;
                                },
                                CharactorType::Companion => {
                                    println!("Clicked on Companion");
                                    move_player_to_position(&mut destination_component, position_x, position_y);
                                },
                            }
                        },
                        ObjectType::Stuff => {
                            move_player_to_position(&mut destination_component, position_x, position_y);
                        },
                        ObjectType::Thing => {
                            println!("Clicked on Tile or Prijectile");
                            //TODO: for thing we need to set target and action type;
                            move_player_to_position(&mut destination_component, position_x, position_y);
                        },
                        ObjectType::Projectile | ObjectType::Tile => {
                            println!("Clicked on Tile or Prijectile");
                            move_player_to_position(&mut destination_component, position_x, position_y);
                        },
                    };
                };
            };
            /*
            //check click thing on ground
            for (thing_component, position_component) in thing_query.iter() {
                let thing_position_x = posotion_component.position.x;
                let thing_position_y = position_component.position.y;
                let thing_type = thing_component.thing_type;
                if thing_position_x == position_x && thing_position_y == position_y {
                    select_thing_to_use( &mut player, &mut position, thing_type, thing_component.id, position_x, position_y)
                };
            }
            */
            //check coordinates have a property values;
        } else {
            // cursor is not inside the window
        }
    }
}

fn move_player_to_position(
    destination: &mut DestinationComponent,
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
