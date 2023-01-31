use serde::{ Serialize, Deserialize };
use bevy::prelude::*;
use rand::Rng;

use crate::materials::material_manager::MaterialManager;
use crate::resources::scene_data::character::Character;
use crate::resources::scene_data::thing::Thing;
use crate::resources::scene_data::ground_effect::GroundEffect;
use crate::resources::scene_data::stuff::Stuff;
use crate::resources::scene_manager::SceneManager;
use crate::resources::tilemap::ground_tilemap::GroundTilemap;
use crate::resources::tilemap::tile::ground_tilemap_tile::{GroundType, CoverType};
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
    pub fn new( id: usize ) -> Self{
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
    pub tilemap_ground_layer: Option<Entity>,
    pub tilemap_cover_layer: Option<Entity>,
    pub things_layer: Option<Entity>,
    pub stuff_layer: Option<Entity>,
    pub characters_layer: Option<Entity>,
    pub effects_layer: Option<Entity>,
    //pub roof_layer: Option<Entity>,
    //pub fog_layer: Option<Entity>,
}



pub struct GameGroundScenePlugin;

impl Plugin for GameGroundScenePlugin{
    fn build( &self, app: &mut App ){
        app.add_system_set( SystemSet::on_enter( SceneState::GameGroundScene )
            .with_system( spawn_tilemap_ground )
            .with_system( spawn_tilemap_cover )
            //.with_system( spawn_things )
        );
        app.add_system_set( SystemSet::on_update( SceneState::GameGroundScene ).with_system( update ));
        app.add_system_set( SystemSet::on_exit( SceneState::GameGroundScene ).with_system( cleanup ));
    }
}

fn spawn_tilemap_ground( 
    mut commands: Commands,
    scene: Res<GameGroundScene>,
    mut scene_data: ResMut<GameGroundSceneData>,
    material_manager: Res<MaterialManager>,
){
    let ground_tiles = commands.spawn_bundle( SpriteBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0), // 0 - layer;
        ..Default::default()
    })
    .with_children( |parent| {
        let tile_storage = scene.tilemap.get_tilemap_tile_storage();
        for i in 0..tile_storage.len(){
            let x = tile_storage[ i ].graphic_x;
            let y = tile_storage[ i ].graphic_y;
            let ground_type = &tile_storage[ i ].ground_type;

            let transform = Transform::from_xyz( x as f32, y as f32, 0.0 );
            let texture: Handle<Image> = match ground_type {
                GroundType::Earth => { material_manager.ground_scene.ground_tile.earth.clone() },
                GroundType::Dirt => { material_manager.ground_scene.ground_tile.dirt.clone() },
                GroundType::DryEarth => { material_manager.ground_scene.ground_tile.dry_earth.clone() },
                GroundType::Rock => { material_manager.ground_scene.ground_tile.rock.clone() },
                GroundType::RockEnvironment => { material_manager.ground_scene.ground_tile.rock_environment.clone() },
            };


            parent.spawn_bundle( SpriteBundle{
                transform: transform,
                texture: texture,
                ..Default::default()
            });
        }
    })
    .id();

    scene_data.tilemap_ground_layer = Some( ground_tiles );
}

fn spawn_tilemap_cover(
    mut commands: Commands,
    scene: Res<GameGroundScene>,
    mut scene_data: ResMut<GameGroundSceneData>,
    material_manager: Res<MaterialManager>,
){
    let cover_tiles = commands.spawn_bundle( SpriteBundle{
        transform: Transform::from_xyz(0.0, 0.0, 0.1), // 1 - layer;
        ..Default::default()
    })
    .with_children( |parent| {
        let tile_storage = scene.tilemap.get_tilemap_tile_storage();
        let mut rnd = rand::thread_rng();
        for i in 0..tile_storage.len(){
            let x = tile_storage[ i ].graphic_x;
            let y = tile_storage[ i ].graphic_y;
            let cover_type = &tile_storage[ i ].cover_type;

            if *cover_type == CoverType::None { continue; };

            let transform = Transform::from_xyz( x as f32, y as f32, 0.0 );

            let index = match cover_type{
                CoverType::Ice => { tile_storage[ i ].cover_graphic_index as usize },
                CoverType::Water => { tile_storage[ i ].cover_graphic_index as usize },
                CoverType::Shallow => { tile_storage[ i ].cover_graphic_index as usize },
                _ => { 
                    let indexes = material_manager.ground_scene.cover_tile.get_indexes( cover_type );
                    if indexes == 0 { 
                        continue
                    }else if indexes == 1{
                        0
                    }else{
                        rnd.gen_range( 0..( indexes ))
                    }
                },
            };
            let texture:Handle<Image> = material_manager.ground_scene.cover_tile.get_image( cover_type, index ).clone();

            parent.spawn_bundle( SpriteBundle{
                transform: transform,
                texture: texture,
                ..Default::default()
            });
        }
    })
    .id();

    scene_data.tilemap_cover_layer = Some( cover_tiles );
}

fn update(){}

fn cleanup(
    mut commands: Commands, 
    scene_data: Res<GameGroundSceneData>, 
    mut scene_manager: ResMut<SceneManager>, 
    scene: Res<GameGroundScene> 
){
    let old_scene = scene_manager.get_ground_scene_by_id( scene.scene_id );
    *old_scene = scene.clone(); // copy and paste scene into scene_manager;

    commands.entity( scene_data.effects_layer.unwrap() ).despawn_recursive();
    commands.entity( scene_data.characters_layer.unwrap() ).despawn_recursive();
    commands.entity( scene_data.stuff_layer.unwrap() ).despawn_recursive();
    commands.entity( scene_data.things_layer.unwrap() ).despawn_recursive();
    commands.entity( scene_data.tilemap_cover_layer.unwrap() ).despawn_recursive();
    commands.entity( scene_data.tilemap_ground_layer.unwrap() ).despawn_recursive();
    //commands.entity( scene_data.fog_layer ).despawn_recursive();

    commands.remove_resource::<GameGroundScene>();
}
