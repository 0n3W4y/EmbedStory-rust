use bevy::prelude::*;

use crate::scenes::SceneState;
use crate::config::*;

use crate::resources::dictionary::Dictionary;

pub struct LoadingSceneData{
    user_interface_root: Entity,
}

pub struct LoadingScenePlugin;

impl Plugin for  LoadingScenePlugin{
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(SceneState::LoadingScene)
                .with_system(setup)
                //.with_system(load_materials)
                //.with_system(load_data),
        );
        //app.add_system_set( SystemSet::on_update( SceneState::LoadingScene ).with_system( update_loader ));
        app.add_system_set( SystemSet::on_exit( SceneState::LoadingScene ).with_system( cleanup ));
    }
}

fn setup( mut commands: Commands, asset_server: Res<AssetServer>, dictionary: Dictionary ){

}

fn cleanup( mut commands: Commands, loading_scene_data: LoadingSceneData ){
    commands.entity( loading_scene_data.user_interface_root ).despawn_recursive();
}

fn load_images( mut commands: Commands, asset_server: Res<AssetServer> ){
    let font = asset_server.load( FIRASANS_BOLD_FONT );
}