use bevy::prelude::*;
use bevy::input::mouse::{ MouseButtonInput, MouseMotion, MouseWheel };

use crate::components::charactor_component::PlayerComponent;
//use crate::resources::scene_manager;
use crate::scenes::AppState;
use crate::resources::scene_manager::SceneManager;

#[derive( Component )]
pub struct UserInterfaceCamera;

#[derive( Component )]
pub struct Orthographic2DCamera{
    pub cursor_position: Vec2,
    pub camera_on_player: bool,
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin{
    fn build( &self, app: &mut App ){
        //app.add_startup_system( spawn_ui_camera );
        app.add_startup_system( spawn_2d_camera );
        app
            .add_system(spawn_2d_camera.in_schedule(OnEnter(AppState::GameScene)))
            .add_systems(
                (
                    camera_zoom,
                    camera_move_by_left_button
                )
                .in_set(OnUpdate(AppState::GameScene))
            );
        //app.add_syste_set( SystemSet::on_exit( SceneState::_).width_system(_));
    }
}
/*
fn spawn_ui_camera( mut commands: Commands ){
    commands
        .spawn_bundle( UiCameraBundle::default())
        .insert( Name::new( "UserInterfaceCamera" ))
        .insert( UserInterfaceCamera );
}
*/
fn spawn_2d_camera( mut commands: Commands ){
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 2.0;
    //camera.transform.translation.z = 1000.0;
    //camera.orthographic_projection.top = 1.0;
    //camera.orthographic_projection.bottom = -1.0;
    //camera.orthographic_projection.right = 1.0 * RESOLUTION;
    //camera.orthographic_projection.left = -1.0 * RESOLUTION;

    commands
        .spawn((Camera2dBundle {
            projection: OrthographicProjection {
                scale: 2.0,
                ..Default::default()
                },
            ..Default::default()
            },
            Orthographic2DCamera{ cursor_position: Vec2::new( 0.0, 0.0 ), camera_on_player: false, },
            Name::new("Orthographic2DCamera"),
        ));
}

fn camera_zoom( 
    mut wheel_input: EventReader<MouseWheel>, 
    mut camera: Query<&mut OrthographicProjection>,
){
    let mut scaling = 0.0;
        for event in wheel_input.iter(){
            scaling = event.y;
        }
        if scaling != 0.0{
            for mut projection in camera.iter_mut(){
                if scaling < 0.0 {
                    projection.scale += 0.4;
                }else if scaling > 0.0 {
                    projection.scale -= 0.4;
                }
                if projection.scale >= 6.0 {
                    projection.scale = 6.0;
                }else if projection.scale <= 1.0 {
                    projection.scale = 1.0;                    
                }  
            }
        }else{
            return;
        } 
}

fn camera_move_by_left_button(
    mouse_button_input: Res<Input<MouseButton>>, 
    mut cursor_moved_events: EventReader<CursorMoved>, 
    mut camera: Query<( &mut Transform, &mut Orthographic2DCamera, &OrthographicProjection ), With<Orthographic2DCamera>>,
    scene_manager: Res<SceneManager>,
) {
    //let tilemap = &scene_manager.get_current_game_scene().tilemap;
    //let tile_size = tilemap.get_tile_size() as f32;
    //let map_width = tilemap.get_tilemap_width() as f32;
    //let map_height = tilemap.get_tilemap_height() as f32;

    //let mut min_x = -(map_width / 2.0  * tile_size);
    //let mut max_x = map_width / 2.0 * tile_size;
    //let mut max_y = map_height / 2.0 * tile_size;
    //let mut min_y = -(map_height / 2.0 * tile_size);
    if mouse_button_input.pressed( MouseButton::Left ) {
        for ( mut transform, mut cam, projection ) in camera.iter_mut(){
            if cam.camera_on_player {
                return;
            };
            
            for event in cursor_moved_events.iter() {
                if cam.cursor_position.x == 0.0 {
                    cam.cursor_position.x = event.position.x;
                    cam.cursor_position.y = event.position.y;
                }
                let projection_scale = projection.scale;
                let dif_x = cam.cursor_position.x - event.position.x;
                let dif_y = cam.cursor_position.y - event.position.y;
                let camera_x = dif_x * projection_scale;
                let canera_y = dif_y * projection_scale;

                //max_x = max_x * projection_scale;
                //max_y = max_y * projection_scale;
                //min_x = min_x * projection_scale;
                //min_y = min_y * projection_scale;

                transform.translation.x += camera_x;
                transform.translation.y += canera_y;
                /*
                if transform.translation.x > max_x {
                    transform.translation.x = max_x;
                }

                if transform.translation.x < min_x{
                    transform.translation.x = min_x;
                }

                if transform.translation.y > max_y {
                    transform.translation.y = max_y;
                }

                if transform.translation.y < min_y {
                    transform.translation.y = min_y;
                }                
                */
                cam.cursor_position.x = event.position.x;
                cam.cursor_position.y = event.position.y;               
            }
        }
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        for ( _, mut cam, _ ) in camera.iter_mut(){
            cam.cursor_position.x = 0.0;
            cam.cursor_position.y = 0.0;
        }
    }
}

pub fn move_by_player_moving(
    player: Query<&Transform, With<PlayerComponent>>,
    mut camera: Query<(&mut Transform, &mut Orthographic2DCamera, &OrthographicProjection), With<Orthographic2DCamera>>,
){
    let (mut cam_transform, cam, cam_projection) = camera.single_mut();
    let player_transform = player.single();

    if cam.camera_on_player {
        cam_transform.translation.x = player_transform.translation.x;
        cam_transform.translation.y = player_transform.translation.y;
    };
}

//todo: All functions with camera;