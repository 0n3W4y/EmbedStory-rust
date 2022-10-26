mod scene_manager;
mod entity_manager;
mod texture_manager;
mod deploy;


use bevy::{
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    prelude::*,
    window::{CursorMoved, PresentMode,},
    diagnostic::{ Diagnostics, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin },
    reflect::TypeRegistry,
};
use entity_manager::EntityManager;
use texture_manager::TextureManager;
use scene_manager::SceneManager;
use deploy::Deploy;


struct Game{
    scene_manager: SceneManager,
    entity_manager: EntityManager,
    texture_manager: TextureManager,
    deploy: Deploy,
}

fn main(){
    App::new()
        .insert_resource(WindowDescriptor { 
            title: "test".to_string(), 
            width: 1280.0,                 
            height: 768.0,
            present_mode: PresentMode::Immediate,
            resizable: true,             
            ..default()
        })
        .add_plugins( DefaultPlugins )
        .register_type::<TextureManager>()
        .register_type::<EntityManager>()
        .register_type::<SceneManager>()
        .register_type::<Deploy>()
        .run();
}