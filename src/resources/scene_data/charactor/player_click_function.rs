use bevy::prelude::*;

use crate::components::charactor_component::CharactorComponent;
use crate::components::charactor_component::PlayerComponent;
use crate::components::charactor_component::PositionComponent;
use crate::config::{MONITOR_HALF_HEIGHT, MONITOR_HALF_WIDTH, TILE_SIZE};
use crate::plugins::camera::Orthographic2DCamera;
use crate::resources::scene_data::charactor::CharactorStatus;
use crate::resources::scene_manager::SceneManager;
use crate::scenes::game_scenes::tilemap::tile::Position;

use super::CharactorType;

pub fn player_click(
    windows: Res<Windows>,
    mouse_button_input: Res<Input<MouseButton>>,
    scene_manager: Res<SceneManager>,
    mut player_query: Query<
        (&mut CharactorComponent, &mut PositionComponent),
        With<PlayerComponent>,
    >,
    //thing_query: Query<(&ThingComponent, &ThingPositionComponent)>,
    charactor_query: Query<(&CharactorComponent, &PositionComponent), Without<PlayerComponent>>,
    //stuff_component: Query<(&StuffComponent, &StuffPositionComponent)>,
    camera: Query<(&Transform, &OrthographicProjection), With<Orthographic2DCamera>>,
) {
    let (mut player, mut position) = player_query.single_mut();
    let window = windows.get_primary().unwrap();
    let scene = scene_manager.get_current_game_scene();

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

            for (charactor_component, position_component) in charactor_query.iter(){
                let char_position_x = position_component.position.x;
                let char_position_y = position_component.position.y;
                if char_position_x == position_x && char_position_y == position_y {
                    match charactor_component.charactor_type {
                        CharactorType::Monster => {
                            select_target_to_attack(&mut player, &mut position, charactor_component.id, position_x, position_y);
                            return;
                        },
                        _ => {
                            move_player_to(&mut player, &mut position, position_x, position_y);
                        }
                    };
                };
            };
            /*
            //check for click on stuff item in ground;
            for (stuff_component, position_component) in stuff_query.iter() {
                let stuff_position_x = position_component.position.x;
                let stuff_position_y = position_component.position.y;
                if stuff_position_x == position_x && stuff_position_y == position_y {
                    select_item_to_pickup( &mut player, &mut position, stuff_component.id, position_x, position_y);
                    return;
                };
            };
            */
            //check coordinates have a property values;
            move_player_to(&mut player, &mut position, position_x, position_y);
        } else {
            // cursor is not inside the window
        }
    }
}

fn move_player_to(
    player: &mut CharactorComponent,
    position: &mut PositionComponent,
    x: i32,
    y: i32,
) {
    position.destination_point = Position { x, y };
    player.status = CharactorStatus::Moving;
}

fn select_target_to_attack(
    player: &mut CharactorComponent,
    position: &mut PositionComponent,
    id: usize,
    x: i32,
    y: i32,
) {
    position.destination_point = Position { x, y };
    player.target = Some(id);
    player.status = CharactorStatus::Attacking;
}

fn select_item_to_pickup(
    player: &mut CharactorComponent,
    position: &mut PositionComponent,
    id: usize,
    x: i32,
    y: i32,
) {
    position.destination_point = Position { x, y };
    player.target = Some(id);
    player.status = CharactorStatus::PickupItem;
}
