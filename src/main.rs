use bevy::{
    prelude::*,
    window::{WindowMode, WindowResolution},
    //window::PresentMode,
};

mod config;
mod resources;
mod scenes;
mod materials;
mod plugins;
mod components;

use config::*;
use scenes::AppState;

fn main(){
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window {
                title: TITLE.to_string(),
                resolution: WindowResolution::new(WINDOW_HEIGHT*RESOLUTION, WINDOW_HEIGHT),
                //present_mode: PresentMode::Immediate,
                resizable: false,
                //resize_constraints: bevy::window::WindowResizeConstraints {
                //   min_width: WINDOW_HEIGHT * RESOLUTION,
                //    max_width: WINDOW_HEIGHT * RESOLUTION,
                //    min_height: WINDOW_HEIGHT,
                //    max_height: WINDOW_HEIGHT,
                //},
                mode: WindowMode::Windowed,  
                ..default()
            }),
            ..default()
        }))
        .init_resource::<resources::setting::Setting>()
        .init_resource::<resources::dictionary::Dictionary>()
        .init_resource::<resources::deploy::Deploy>()
        .add_state::<AppState>()
        //.add_plugins(DefaultPlugins)
        //.add_plugin(AudioPlugin)
        .add_plugin(plugins::camera::CameraPlugin)
        .add_plugin(scenes::loading_scene::LoadingScenePlugin)
        .add_plugin(scenes::main_menu_scene::MainMenuScenePlugin)
        .add_plugin(scenes::options_scene::OptionsScenePlugin)
        .add_plugin(scenes::create_char_scene::CreateCharScenePlugin )
        .add_plugin(scenes::loading_new_game_scene::LoadingNewGameScenePlugin )
        .add_plugin(scenes::game_scenes::game_scene::GameScenePlugin )        
        .run();
}