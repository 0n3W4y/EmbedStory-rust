use serde::{ Serialize, Deserialize };
use bevy::prelude::*;

use crate::resources::gamedata::character::Character;
use crate::resources::gamedata::object::Object;
use crate::resources::gamedata::ground_effect::GroundEffect;
use crate::resources::gamedata::stuff::Stuff;
use crate::resources::tilemap::ground_tilemap::GroundTilemap;
use crate::resources::deploy::Deploy;
use crate::scenes::SceneState;

#[derive( Serialize, Deserialize )]
pub struct GameGroundScene{
    pub scene_id: u32,
    pub tilemap: GroundTilemap,
    pub objects: Vec<Object>,
    pub stuff: Vec<Stuff>,
    pub characters: Vec<Character>,
    pub effects: Vec<GroundEffect>,

}
impl GameGroundScene{
    pub fn new( id: u32 ) -> GameGroundScene{
        let new_tilemap = GroundTilemap::new();
        return GameGroundScene{
            scene_id: id,
            tilemap: new_tilemap,
            objects: vec![],
            stuff: vec![],
            characters: vec![],
            effects: vec![],
        };        
    }
}

pub struct GameGroundSceneData{
    pub tilemap_ground_layer: Entity,
    pub tilemap_cover_layer: Entity,
    pub objects_layer: Entity,
    pub stuff_layer: Entity,
    pub characters_layer: Entity,
    pub effects_layer: Entity,
    //pub fog_layer: Entity,
}



pub struct GameGroundScenePlugin;

impl Plugin for GameGroundScenePlugin{
    fn build( &self, app: &mut App ){
        app.add_system_set( SystemSet::on_enter( SceneState::GameGroundScene ).with_system( setup ));
        app.add_system_set( SystemSet::on_update( SceneState::GameGroundScene ).with_system( update ));
        app.add_system_set( SystemSet::on_exit( SceneState::GameGroundScene ).with_system( cleanup ));
    }
}

fn setup(
    mut commands: Commands,
    deploy: Res<Deploy>,
){}

fn update(){}

fn cleanup(mut commands: Commands, scene_data: Res<GameGroundSceneData> ){
    //commands.entity( scene_data.fog_layer ).despawn_recursive();
    commands.entity( scene_data.effects_layer ).despawn_recursive();
    commands.entity( scene_data.characters_layer ).despawn_recursive();
    commands.entity( scene_data.stuff_layer ).despawn_recursive();
    commands.entity( scene_data.objects_layer ).despawn_recursive();
    commands.entity( scene_data.tilemap_cover_layer ).despawn_recursive();
    commands.entity( scene_data.tilemap_ground_layer ).despawn_recursive();
}
