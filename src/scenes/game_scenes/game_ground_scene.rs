use serde::{ Serialize, Deserialize };
use bevy::prelude::*;

use crate::resources::scene_data::character::Character;
use crate::resources::scene_data::thing::Thing;
use crate::resources::scene_data::ground_effect::GroundEffect;
use crate::resources::scene_data::stuff::Stuff;
use crate::resources::scene_manager::SceneManager;
use crate::resources::tilemap::ground_tilemap::GroundTilemap;
use crate::resources::deploy::Deploy;
use crate::scenes::SceneState;

#[derive( Serialize, Deserialize, Clone )]
pub struct GameGroundScene{
    pub scene_id: usize,
    pub index: usize, // vector index in scene_manager.ground_scene;
    pub tilemap: GroundTilemap,
    pub things: Vec<Thing>,
    pub stuff: Vec<Stuff>,
    pub characters: Vec<Character>,
    pub effects: Vec<GroundEffect>,
    //pub fog: Vec<>,
    //pub roof: Vec<>,

}
impl GameGroundScene{
    pub fn new( id: usize, index: usize ) -> Self{
        let new_tilemap = GroundTilemap::new();
        return GameGroundScene{
            scene_id: id,
            index: 0,
            tilemap: new_tilemap,
            things: vec![],
            stuff: vec![],
            characters: vec![],
            effects: vec![],
        };        
    }
}

pub struct GameGroundSceneData{
    pub tilemap_ground_layer: Entity,
    pub tilemap_cover_layer: Entity,
    pub things_layer: Entity,
    pub stuff_layer: Entity,
    pub characters_layer: Entity,
    pub effects_layer: Entity,
    //pub roof_layer: Entity,
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
    scene_manager: ResMut<SceneManager>,
){
    let current_scene: GameGroundScene = scene_manager.get_next_ground_scene().clone();
    //let current_scene: &mut GameGroundScene = game_data.scene_manager.get_next_ground_scene();
    //commands.insert_resource( GameGroundSceneData{ 
    //  tilemap_ground_layer: tilemap_ground_layer,
    //  tilemap_cover_layer: tilemap_cover_layer,
    //  objects_layer: objects_layer,
    //  stuff_layer: stuff_layer,
    //  characters_layer: characters_layer,
    //  effects_layer: effects_layer,
    //  roof_layer: roof_layer,
    //  for_layer: fog_layer,
    //});
}

fn update(){}

fn cleanup(mut commands: Commands, scene_data: Res<GameGroundSceneData> ){
    //commands.entity( scene_data.fog_layer ).despawn_recursive();
    commands.entity( scene_data.effects_layer ).despawn_recursive();
    commands.entity( scene_data.characters_layer ).despawn_recursive();
    commands.entity( scene_data.stuff_layer ).despawn_recursive();
    commands.entity( scene_data.things_layer ).despawn_recursive();
    commands.entity( scene_data.tilemap_cover_layer ).despawn_recursive();
    commands.entity( scene_data.tilemap_ground_layer ).despawn_recursive();
}
