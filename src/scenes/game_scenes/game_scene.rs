use serde::{ Serialize, Deserialize };
use bevy::prelude::*;
use rand::Rng;

use crate::materials::material_manager::MaterialManager;
use crate::resources::scene_data::objects::character::Character;
use crate::resources::scene_data::objects::thing::Thing;
use crate::resources::scene_data::objects::scene_effect::SceneEffect;
use crate::resources::scene_data::objects::stuff::Stuff;
use crate::resources::scene_manager::{ SceneManager, SceneType };
use crate::scenes::game_scenes::tilemap::Tilemap;
use crate::scenes::game_scenes::tilemap::tile::{ GroundType, CoverType };
use crate::scenes::SceneState;

#[derive( Serialize, Deserialize, Clone )]
pub struct GameScene{
    pub scene_type: SceneType,
    pub scene_id: usize,
    pub index: usize, // vector index in scene_manager.ground_scene;
    pub tilemap: Tilemap,
    pub things: Vec<Thing>,
    pub stuff: Vec<Stuff>,
    pub characters: Vec<Character>,
    pub effects: Vec<SceneEffect>,
    //pub fog: Vec<>,
    //pub roof: Vec<>,

}
impl GameScene{
    pub fn new( id: usize ) -> Self{
        let new_tilemap = Tilemap::new();
        return GameScene{
            scene_type: SceneType::GroundScene,
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

pub struct GameSceneData{
    pub tilemap_ground_layer: Option<Entity>,
    pub tilemap_cover_layer: Option<Entity>,
    pub things_layer: Option<Entity>,
    pub stuff_layer: Option<Entity>,
    pub characters_layer: Option<Entity>,
    pub effects_layer: Option<Entity>,
    //pub roof_layer: Option<Entity>,
    //pub fog_layer: Option<Entity>,
}



pub struct GameScenePlugin;

impl Plugin for GameScenePlugin{
    fn build( &self, app: &mut App ){
        app.add_system_set( SystemSet::on_enter( SceneState::GameScene )
            .with_system( spawn_tilemap_ground )
            .with_system( spawn_tilemap_cover )
            .with_system( spawn_things )
        );
        app.add_system_set( SystemSet::on_update( SceneState::GameScene ).with_system( update ));
        app.add_system_set( SystemSet::on_exit( SceneState::GameScene ).with_system( cleanup ));
    }
}

fn spawn_tilemap_ground( 
    mut commands: Commands,
    scene: Res<GameScene>,
    mut scene_data: ResMut<GameSceneData>,
    material_manager: Res<MaterialManager>,
){
    let ground_tiles = commands.spawn_bundle( SpriteBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0), // 0 - layer;
        ..Default::default()
    })
    .with_children( |parent| {
        let tile_storage = scene.tilemap.get_tilemap_tile_storage();
        for i in 0..tile_storage.len(){
            let x = tile_storage[ i ].graphic_position.x;
            let y = tile_storage[ i ].graphic_position.y;
            let ground_type = &tile_storage[ i ].ground_type;

            let transform = Transform::from_xyz( x as f32, y as f32, 0.0 );
            let texture: Handle<Image> = match ground_type {
                GroundType::Earth => { material_manager.game_scene.ground_tile.earth.clone() },
                GroundType::Dirt => { material_manager.game_scene.ground_tile.dirt.clone() },
                GroundType::DryEarth => { material_manager.game_scene.ground_tile.dry_earth.clone() },
                GroundType::Rock => { material_manager.game_scene.ground_tile.rock.clone() },
                GroundType::RockEnvironment => { material_manager.game_scene.ground_tile.rock_environment.clone() },
                GroundType::Clay => { material_manager.game_scene.ground_tile.clay.clone() },
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
    scene: Res<GameScene>,
    mut scene_data: ResMut<GameSceneData>,
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
            let x = tile_storage[ i ].graphic_position.x;
            let y = tile_storage[ i ].graphic_position.y;
            let cover_type = &tile_storage[ i ].cover_type;

            if *cover_type == CoverType::None { continue; };

            let transform = Transform::from_xyz( x as f32, y as f32, 0.0 );

            let index = match cover_type{
                CoverType::Ice => { tile_storage[ i ].cover_graphic_index as usize },
                CoverType::Water => { tile_storage[ i ].cover_graphic_index as usize },
                CoverType::Shallow => { tile_storage[ i ].cover_graphic_index as usize },
                _ => { 
                    let indexes = material_manager.game_scene.cover_tile.get_indexes( cover_type );
                    if indexes == 0 { 
                        continue
                    }else if indexes == 1{
                        0
                    }else{
                        rnd.gen_range( 0..( indexes ))
                    }
                },
            };
            let texture:Handle<Image> = material_manager.game_scene.cover_tile.get_image( cover_type, index ).clone();

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

fn spawn_things(
    mut commands: Commands,
    scene: Res<GameScene>,
    mut scene_data: ResMut<GameSceneData>,
    material_manager: Res<MaterialManager>,
) {
    let things = commands.spawn_bundle(bundle);

    scene_data.things_layer = Some(things);
}

fn update(){}

fn cleanup(
    mut commands: Commands, 
    scene_data: Res<GameSceneData>, 
    mut scene_manager: ResMut<SceneManager>, 
    scene: Res<GameScene>
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

    commands.remove_resource::<GameScene>();
}
