use bevy::prelude::*;

use crate::config::TILE_SIZE;
use crate::resources::scene_manager::SceneManager;
use crate::scenes::game_scenes::game_scene::GameScene;
use crate::materials::material_manager::MaterialManager;
use crate::components::thing_component::ThingComponent;
use crate::resources::scene_data::objects::thing::Thing;

use super::ThingType;

pub const Z_POSITION: f32 = 2.0; // third layer;

pub fn draw(
    mut commands: Commands,
    scene_manager: Res<SceneManager>,
    material_manager: Res<MaterialManager>,
){
    let scene: &GameScene = scene_manager.get_current_game_scene();
    let total_tiles = scene.tilemap.get_total_tiles();
    for thing in scene.things.iter(){
        let x = thing.position.x as f32 * TILE_SIZE as f32;
        let mut y = thing.position.y as f32 * TILE_SIZE as f32;
        if thing.thing_type == ThingType::FertileTree
        || thing.thing_type == ThingType::Tree {
            y += (TILE_SIZE / 2) as f32;
        };

        let index = thing.graphic_index;
        let thing_type = &thing.thing_type;

        let texture = material_manager
                    .game_scene
                    .things
                    .get_atlas(thing_type);
        let new_z_position = Z_POSITION + ((total_tiles as f32 - thing.tile_index as f32) / total_tiles as f32); // tile with index 0 have a higher z-order, with 10000 - lower z-order;
        let transform = Transform::from_xyz(x, y, new_z_position);
        let mut component: ThingComponent = Default::default();
        copy_from_thing_to_entity_component(&mut component, thing);

        commands
        .spawn_bundle(SpriteSheetBundle {
            transform,
            sprite: TextureAtlasSprite::new(index as usize),
            texture_atlas: texture,
            ..Default::default()
        })
        .insert(component);
    }    
}

pub fn copy_from_thing_to_entity_component(component: &mut ThingComponent, thing: &Thing){
    component.thing_type = thing.thing_type.clone();
    component.id = thing.id; 
    component.tile_index = thing.tile_index;
    component.position = thing.position.clone();
    component.graphic_index = thing.graphic_index;
    component.permissions = thing.permissions.to_vec();
    component.resists = thing.resists.clone();
    component.resists_cache = thing.resists_cache.clone();
    component.body_structure = thing.body_structure.clone();
    component.current_health_points = thing.current_health_points;
    component.total_health_points = thing.total_health_points;
}