use bevy::prelude::*;
use serde::{ Serialize, Deserialize };

use crate::scenes::SceneState;

#[derive( Serialize, Deserialize )]
pub struct GameGlobalmapScene{

}

pub struct GameGlobalmapScenePlugin;

impl Plugin for GameGlobalmapScenePlugin{
    fn build( &self, app: &mut App ){
        app.add_system_set( SystemSet::on_enter( SceneState::GameGroundScene ).with_system( setup ));
        app.add_system_set( SystemSet::on_update( SceneState::GameGroundScene ).with_system( update ));
        app.add_system_set( SystemSet::on_exit( SceneState::GameGroundScene ).with_system( cleanup ));
    }
}

fn setup(){}

fn update(){}

fn cleanup(){}