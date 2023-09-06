use bevy::prelude::*;

use crate::components::charactor_component::ActionType;
use crate::components::charactor_component::CharactorComponent;
use crate::components::charactor_component::CharactorTargetComponent;
use crate::components::charactor_component::PlayerComponent;
use crate::components::charactor_component::PositionComponent;
use crate::config::{MONITOR_HALF_HEIGHT, MONITOR_HALF_WIDTH, TILE_SIZE};
use crate::plugins::camera::Orthographic2DCamera;
use crate::resources::scene_manager::SceneManager;
use crate::scenes::game_scenes::tilemap::tile::Position;

use super::CharactorStatus;
use super::CharactorType;

pub fn player_click(
    windows: Res<Windows>,
    mouse_button_input: Res<Input<MouseButton>>,
    scene_manager: Res<SceneManager>,
    mut player_query: Query<
        (&mut CharactorComponent, &mut PositionComponent, &mut CharactorTargetComponent),
        With<PlayerComponent>,
    >,
    //thing_query: Query<(&ThingComponent, &ThingPositionComponent)>,
    charactor_query: Query<(&CharactorComponent, &PositionComponent), Without<PlayerComponent>>,
    //stuff_component: Query<(&StuffComponent, &StuffPositionComponent)>,
    camera: Query<(&Transform, &OrthographicProjection), With<Orthographic2DCamera>>,
) {
    let (mut player, mut position, mut palyer_target) = player_query.single_mut();
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



            //TODO: может быть сделать проход по сетке tile, с заносом информации в tile каждое перемещение. Все объекты
            // стационарны, кроме чаров


            for (charactor_component, position_component) in charactor_query.iter(){
                let char_position_x = position_component.position.x;
                let char_position_y = position_component.position.y;
                if char_position_x == position_x && char_position_y == position_y {
                    match charactor_component.charactor_type {
                        CharactorType::Monster => {
                            select_target_to_attack(&mut player, &mut palyer_target, charactor_component.id, position_x, position_y);
                            return;
                        },
                        CharactorType::Companion => {
                            println!("Clicked on Companion");
                            move_player_to_position(&mut player, &mut position, position_x, position_y);
                        },
                        CharactorType::NPC => {
                            println!("Clicked on NPC");
                            //select_target_to_talk(&mut palyer_target, charactor_component.id, position_x, position_y);
                        }
                        _ => {
                            move_player_to_position(&mut player, &mut position, position_x, position_y);
                        }
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
    player: &mut CharactorComponent,
    position: &mut PositionComponent,
    x: i32,
    y: i32,
) {
    player.status = CharactorStatus::TryMove;
    position.destination_point = Some(Position { x, y });
}

fn select_target_to_attack(
    player: &mut CharactorComponent,
    player_target: &mut CharactorTargetComponent,
    id: usize,
    x: i32,
    y: i32,
) {
    if player.status == CharactorStatus::Standing {
        player.status = CharactorStatus::TryAttack;
    }
    player_target.target = Some(id);
    player_target.action = ActionType::Attack;
    player_target.target_position = Some(Position{ x, y });
}
