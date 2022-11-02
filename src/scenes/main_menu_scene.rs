use bevy::prelude::*;
use bevy::App::AppExit;

use crate::scenes::SceneState;

#[derive(Component, Copy, Clone)]
enum ButtonComponent{
    Play,
    Load,
    Options,
    Quit
}

impl ButtonComponent{
    pub fn iterator() -> Iter<'static, ButtonComponent> {
        [
            ButtonComponent::Play,
            ButtonComponent::Load,
            ButtonComponent::Options,
            ButtonComponent::Quit,
        ].iter()
    }
}

struct MainMenuSceneData{
    user_interface_root: Entity,
}

struct MainMenuScenePlugin;

impl Plugins for MainMenuScenePlugin{
    fn build( &self, app: &mut App ){
        app.add_system_set(SystemSet::on_enter( SceneState::MainMenuScene ).with_system(setup));
        app.add_system_set( SystemSet::on_update( SceneState::MainMenuScene ).with_system( button_handle_system ));
        app.add_system_set( SystemSet::on_exit( SceneState::MainMenuScene ).with_system( cleanup ));
    }
}

fn setup( mut commands: Commands, dictionary: Res<Dictionary>, font: Res<FontMaterials>, texture_manager: Res<MaterialManager>){

}

fn button_handle_system(){

}

fn cleanup( mut commands: Commands, main_menu_scene_data: Res<MainMenuSceneData> ){
    commands.entity( main_menu_scene_data.user_interface_root ).despawn_recursive();
}