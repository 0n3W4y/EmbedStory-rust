use bevy::prelude::*;

use crate::scenes::SceneState;

#[derive( Component )]
pub struct UserInterfaceCamera;

#[derive( Component )]
pub struct Orthographic2DCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin{
    fn build( &self, app: &mut App ){
        app.add_startup_system( spawn_ui_camera );
        app.add_startup_system( spawn_2d_camera );

        //app.add_system_set( SystemSet::on_update(SceneState::...).with_system(...));
        //app.add_system_set( SystemSet::on_exit(SceneState::...).with_system(...));
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

    //camera.orthographic_projection.top = 1.0;
    //camera.orthographic_projection.bottom = -1.0;
    //camera.orthographic_projection.right = 1.0 * RESOLUTION;
    //camera.orthographic_projection.left = -1.0 * RESOLUTION;

    commands
        .spawn_bundle(camera)
        .insert(Orthographic2DCamera)
        .insert(Name::new("Orthographic2DCamera"));
}

//todo: All functions with camera;