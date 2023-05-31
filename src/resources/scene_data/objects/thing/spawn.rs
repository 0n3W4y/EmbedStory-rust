use bevy::prelude::*;

use crate::config::TILE_SIZE;
use crate::{materials::material_manager::MaterialManager, resources::scene_manager::SceneManager};
use crate::components::thing_component::ThingComponent;
use crate::resources::scene_data::objects::thing::ThingType;

use super::{draw::Z_POSITION, Thing};
use super::draw;

pub fn spawn(
    mut commands: Commands,
    scene_manager: Res<SceneManager>,
    material_manager: Res<MaterialManager>,
    thing: &Thing
){
    let x = thing.position.x as f32 * TILE_SIZE as f32;
    let mut y = thing.position.y as f32 * TILE_SIZE as f32;
    if thing.thing_type == ThingType::FertileTree
    || thing.thing_type == ThingType::Tree {
        y += (TILE_SIZE / 2) as f32;
    };
    let index = thing.graphic_index;
    let thing_type = &thing.thing_type;
    let total_tiles = scene_manager.get_current_game_scene().tilemap.get_total_tiles();

    let new_z_position = Z_POSITION + ((total_tiles as f32 - thing.tile_index as f32) / total_tiles as f32); // tile with index 0 have a higher z-order, with 10000 - lower z-order;
    let texture = material_manager
                .game_scene
                .things
                .get_atlas(thing_type);
    let transform = Transform::from_xyz(x, y, new_z_position); // third layer;
    let mut component: ThingComponent = Default::default();
    draw::copy_from_thing_to_entity_component(&mut component, thing);

    commands
        .spawn_bundle(SpriteSheetBundle {
            transform,
            sprite: TextureAtlasSprite::new(thing.graphic_index as usize),
            texture_atlas: texture,
            ..Default::default()
        })
        .insert(component);
}