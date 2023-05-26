use bevy::prelude::*;

use crate::resources::scene_manager::SceneManager;
use crate::components::charactor_component::PlayerComponent;
use crate::plugins::camera::Orthographic2DCamera;
use crate::components::charactor_component::CharactorComponent;
use crate::resources::scene_data::objects::charactor::CharactorStatus;
use crate::scenes::game_scenes::tilemap::tile::Tile;
use crate::config::{TILE_SIZE, MONITOR_HALF_HEIGHT, MONITOR_HALF_WIDTH};

pub fn player_click(
    windows: Res<Windows>, 
    mouse_button_input: Res<Input<MouseButton>>,
    scene_manager: Res<SceneManager>,
    mut player_query: Query<&mut CharactorComponent, With<PlayerComponent>>,
    camera: Query<( &Transform, &OrthographicProjection), With<Orthographic2DCamera>>,
){
    let mut player = player_query.single_mut();
    let window = windows.get_primary().unwrap();
    let scene = scene_manager.get_current_game_scene();
    

    if mouse_button_input.just_released(MouseButton::Left) {
        let mut x:f32 = 0.0;
        let mut y:f32 = 0.0;
        let mut cam_x:f32 = 0.0;
        let mut cam_y:f32 = 0.0;
        let mut camera_scale: f32 = 0.0;

        if let Some(_position) = window.cursor_position() {
            x = _position.x;
            y = _position.y;
            for ( transform, projection ) in camera.iter(){
                cam_x = transform.translation.x;
                cam_y = transform.translation.y;
                camera_scale = projection.scale;
            }
            //let position_x:i8 = ((( x - MONITOR_HALF_WIDTH as f32 + cam_x ) * camera_scale )  / TILE_SIZE as f32 ).round() as i8;
            //let position_y:i8 = ((( y - MONITOR_HALF_HEIGHT as f32 + cam_y ) * camera_scale )  / TILE_SIZE as f32 ).round() as i8;
            let position_x:i32 = (( x  + cam_x / camera_scale - MONITOR_HALF_WIDTH as f32 )  / ( TILE_SIZE as f32 / camera_scale )   ).round() as i32;
            let position_y:i32 = (( y  + cam_y / camera_scale - MONITOR_HALF_HEIGHT as f32  )  /  ( TILE_SIZE as f32 / camera_scale )  ).round() as i32;

            let tile: &Tile = scene.tilemap.get_tile_by_position(position_x, position_y);

            //check coordinates have a property values;
            player.destination_point.x = position_x;
            player.destination_point.y = position_y;
            player.status = CharactorStatus::Moving;

        } else {
            // cursor is not inside the window
        }
    }

}