use bevy::prelude::*;

use crate::scenes::SceneState;
use crate::resources::dictionary::Dictionary;

enum MainButtonComponent{
    Return,
    Start,
}

enum IncreaseDecreseButtonComponent{

}

pub struct CreateCharSceneData{
    pub user_interface_root: Entity,
}

pub struct CreateCharScenePlugin;

impl Plugin for CreateCharScenePlugin{
    fn build( &self, app: &mut App ){
        app.add_system_set(SystemSet::on_enter( SceneState::MainMenuScene ).with_system( setup ));
        app.add_system_set( SystemSet::on_update( SceneState::MainMenuScene ).with_system( button_handle_system ));
        app.add_system_set( SystemSet::on_exit( SceneState::MainMenuScene ).with_system( cleanup ));
    }
}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    dictionary: Res<Dictionary>
){

}

fn button_handle_system(){}

fn cleanup(){}