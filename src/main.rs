use bevy::{
    prelude::*,
    window::{ PresentMode, WindowMode },
};

fn main(){
    App::new()
        .insert_resource(WindowDescriptor { 
            title: "test".to_string(), 
            width: 1280.0,                 
            height: 768.0,
            present_mode: PresentMode::Immediate,
            resizable: true,
            mode: WindowMode::Windowed,  
            ..default()
        })
        //.init_resource::<resources::setting::Setting>()
        //.init_resource::<resources::dictionary::Dictionary>()
        //.add_state(scenes::SceneState::LoadingScene)
        //.add_startup_system(plugins::music::background_audio_channel_setup)
        //.add_system(plugins::music::play_background_music)
        //.add_plugins(DefaultPlugins)
        //.add_plugin(AudioPlugin)
        .run();
}