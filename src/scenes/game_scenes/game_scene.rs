use bevy::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::materials::material_manager::MaterialManager;
use crate::resources::scene_data::objects::character::Character;
use crate::resources::scene_data::objects::scene_effect::SceneEffect;
use crate::resources::scene_data::objects::stuff::Stuff;
use crate::resources::scene_data::objects::thing::{Thing, ThingType};
use crate::resources::scene_manager::{SceneManager, SceneType};
use crate::scenes::game_scenes::tilemap::tile::{CoverType, GroundType};
use crate::scenes::game_scenes::tilemap::Tilemap;
use crate::scenes::SceneState;

#[derive(Serialize, Deserialize, Clone)]
pub struct GameScene {
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
impl GameScene {
    pub fn new(id: usize) -> Self {
        let new_tilemap = Tilemap::new();
        return GameScene {
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

pub struct GameSceneData {
    pub tilemap_ground_layer: Vec<Entity>,
    pub tilemap_cover_layer: Vec<Entity>,
    pub things_layer: Vec<Entity>,
    pub stuff_layer: Vec<Entity>,
    pub characters_layer: Vec<Entity>,
    pub effects_layer: Vec<Entity>,
    //pub roof_layer: Option<Entity>,
    //pub fog_layer: Option<Entity>,
}

pub struct GameScenePlugin;

impl Plugin for GameScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(SceneState::GameScene)
                .with_system(spawn_tilemap_ground)
                .with_system(spawn_tilemap_cover)
                .with_system(spawn_things),
        );
        app.add_system_set(SystemSet::on_update(SceneState::GameScene).with_system(update));
        app.add_system_set(SystemSet::on_exit(SceneState::GameScene).with_system(cleanup));
    }
}

fn spawn_tilemap_ground(
    mut commands: Commands,
    scene: Res<GameScene>,
    mut scene_data: ResMut<GameSceneData>,
    material_manager: Res<MaterialManager>,
) {
    let tile_storage = scene.tilemap.get_tilemap_tile_storage();
    for tile in tile_storage.iter() {
        let x = tile.graphic_position.x;
        let y = tile.graphic_position.y;
        let ground_type = &tile.ground_type;

        let transform = Transform::from_xyz(x as f32, y as f32, 0.0);
        //TODO: Rebuild this to get image from material manager with function;
        let texture: Handle<Image> = material_manager.game_scene.ground_tile.get_image(ground_type).clone();
        let ground_tile: Entity = commands
            .spawn_bundle(SpriteBundle {
                transform,
                texture,
                ..Default::default()
            })
            .id();
        scene_data.tilemap_ground_layer.push(ground_tile);
    }
}

fn spawn_tilemap_cover(
    mut commands: Commands,
    scene: Res<GameScene>,
    mut scene_data: ResMut<GameSceneData>,
    material_manager: Res<MaterialManager>,
) {
    
    let tile_storage = scene.tilemap.get_tilemap_tile_storage();
    for tile in tile_storage.iter() {
        let x = tile.graphic_position.x;
        let y = tile.graphic_position.y;
        let cover_type = &tile.cover_type;

        if *cover_type == CoverType::None {
            continue;
        };
        let transform = Transform::from_xyz(x as f32, y as f32, 0.1); // second layer
        let index = tile.cover_graphic_index;

        let texture: Handle<Image> = material_manager
            .game_scene
            .cover_tile
            .get_image(cover_type, index)
            .clone();

        let cover_tile = commands.spawn_bundle(SpriteBundle {
            transform: transform,
            texture: texture,
            ..Default::default()
        })
        .id();
        scene_data.tilemap_cover_layer.push(cover_tile);
    }

}

fn spawn_things(
    mut commands: Commands,
    scene: Res<GameScene>,
    mut scene_data: ResMut<GameSceneData>,
    material_manager: Res<MaterialManager>,
) {
    let things = commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.2), // 2 - layer;
            ..Default::default()
        })
        .with_children(|parent| {
            for thing in scene.things.iter() {
                let thing_type = &thing.thing_type;

                let x: f32 = thing.graphic_position.x;
                let y: f32 = thing.graphic_position.y;

                let index = thing.graphic_index;
                let texture = material_manager
                    .game_scene
                    .things
                    .get_image(thing_type, index as usize);

                parent.spawn_bundle(SpriteBundle {
                    transform: transform,
                    texture: texture,
                    ..Default::default()
                });
            }
        })
        .id();

    scene_data.things_layer = Some(things);
}

fn update() {}

fn cleanup(
    mut commands: Commands,
    scene_data: Res<GameSceneData>,
    mut scene_manager: ResMut<SceneManager>,
    scene: Res<GameScene>,
) {
    let old_scene = scene_manager.get_ground_scene_by_id(scene.scene_id);
    //chnage all scene objects into scene in scene_manager;
    old_scene.characters = scene.characters;
    old_scene.effects = scene.effects;
    old_scene.stuff = scene.stuff;
    old_scene.things = scene.things;
    old_scene.tilemap = scene.tilemap;

    for tile in scene_data.tilemap_ground_layer.into_iter() {
        commands.entity(tile).despawn_recursive();
    }
        
    commands
        .entity(scene_data.characters_layer.unwrap())
        .despawn_recursive();
    commands
        .entity(scene_data.stuff_layer.unwrap())
        .despawn_recursive();
    commands
        .entity(scene_data.things_layer.unwrap())
        .despawn_recursive();
    commands
        .entity(scene_data.tilemap_cover_layer.unwrap())
        .despawn_recursive();
    commands
        .entity(scene_data.tilemap_ground_layer.unwrap())
        .despawn_recursive();
    //commands.entity( scene_data.fog_layer ).despawn_recursive();

    commands.remove_resource::<GameScene>();
}
