use bevy::{
    prelude::*,
    window::WindowMode,
    //window::PresentMode,
};

mod config;
mod resources;
mod scenes;
mod materials;
mod plugins;
mod components;

use config::*;

fn main(){
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            window: WindowDescriptor {
                title: TITLE.to_string(),
                width: WINDOW_HEIGHT*RESOLUTION,                 
                height: WINDOW_HEIGHT,
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
            },
            ..default()
        }))
        .init_resource::<resources::setting::Setting>()
        .init_resource::<resources::dictionary::Dictionary>()
        .init_resource::<resources::deploy::Deploy>()
        .add_state(scenes::SceneState::LoadingScene)
        //.add_startup_system(plugins::music::background_audio_channel_setup)
        //.add_system(plugins::music::play_background_music)
        .add_plugins(DefaultPlugins)
        //.add_plugin(AudioPlugin)
        .add_plugin(plugins::camera::CameraPlugin)
        .add_plugin(scenes::loading_scene::LoadingScenePlugin)
        .add_plugin(scenes::main_menu_scene::MainMenuScenePlugin)
        //.add_plugin(scenes::highscore_scene::HighscoreScenePlugin)
        .add_plugin(scenes::options_scene::OptionsScenePlugin)
        .add_plugin( scenes::create_char_scene::CreateCharScenePlugin )
        .add_plugin( scenes::loading_new_game_scene::LoadingNewGameScenePlugin )
        .add_plugin( scenes::game_scenes::game_scene::GameScenePlugin )        
        //.add_plugin(scenes::help_scene::HelpScenePlugin)
        //.add_plugin(scenes::credits_scene::CreditsScenePlugin)
        //.add_plugin(scenes::game_mode_select_scene::GameModeSelectScenePlugin)
        //.add_plugin(scenes::hero_select_scene::HeroSelectScenePlugin)
        //.add_plugin(scenes::result_scene::ResultScenePlugin)
        //.add_plugin(scenes::pause_scene::PauseScenePlugin)
        //.add_plugin(scenes::rewards_scene::RewardsScenePlugin)
        //.add_plugin(scenes::reward_scene::RewardScenePlugin)
        //.add_plugin(plugins::input::InputHandlePlugin)
        //.add_plugin(plugins::player::PlayerPlugin)
        //.add_plugin(plugins::weapon::WeaponPlugin)
        //.add_plugin(plugins::classic_mode::ClassicModePlugin)
        //.add_plugin(plugins::classic_mode::ui::ClassicModeUIPlugin)
        //.add_plugin(plugins::survival_mode::SurvivalModePlugin)
        //.add_plugin(plugins::survival_mode::ui::SurvivalModeUIPlugin)
        //.add_plugin(plugins::monster::MonsterPlugin)
        .run();
}