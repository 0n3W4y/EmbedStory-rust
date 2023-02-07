use bevy::prelude::*;
use bevy::input::mouse::{ MouseButtonInput, MouseMotion, MouseWheel };

use crate::scenes::SceneState;

#[derive( Component )]
pub struct UserInterfaceCamera;

#[derive( Component )]
pub struct Orthographic2DCamera{
    pub cursor_position: Vec2,
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin{
    fn build( &self, app: &mut App ){
        app.add_startup_system( spawn_ui_camera );
        app.add_startup_system( spawn_2d_camera );

        app.add_system_set( SystemSet::on_update(SceneState::GameScene).with_system( camera_zoom ));
        app.add_system_set( SystemSet::on_update(SceneState::GameScene).with_system( camera_move_by_left_button ));
        //app.add_syste_set( SystemSet::on_exit( SceneState::_).width_system(_));
    }
}

fn spawn_ui_camera( mut commands: Commands ){
    commands
        .spawn_bundle( UiCameraBundle::default())
        .insert( Name::new( "UserInterfaceCamera" ))
        .insert( UserInterfaceCamera );
}

fn spawn_2d_camera( mut commands: Commands ){
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale = 2.0;
    //camera.orthographic_projection.top = 1.0;
    //camera.orthographic_projection.bottom = -1.0;
    //camera.orthographic_projection.right = 1.0 * RESOLUTION;
    //camera.orthographic_projection.left = -1.0 * RESOLUTION;

    commands
        .spawn_bundle(camera)
        .insert(Orthographic2DCamera{ cursor_position: Vec2::new( 0.0, 0.0 ) })
        .insert(Name::new("Orthographic2DCamera"));
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
    mut camera: Query<( &mut Transform, &mut Orthographic2DCamera, &OrthographicProjection ), With<Orthographic2DCamera>> )
{
    if mouse_button_input.pressed( MouseButton::Left ) {
        for ( mut transform, mut cam, projection ) in camera.iter_mut(){
            for event in cursor_moved_events.iter() {
                if cam.cursor_position.x == 0.0 {
                    cam.cursor_position.x = event.position.x;
                    cam.cursor_position.y = event.position.y;
                }
                let dif_x = cam.cursor_position.x - event.position.x;
                let dif_y = cam.cursor_position.y - event.position.y;
                transform.translation.x += dif_x * projection.scale;
                transform.translation.y += dif_y * projection.scale;

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

//todo: All functions with camera;