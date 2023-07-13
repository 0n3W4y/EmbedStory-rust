use bevy::prelude::*;

use crate::config::TILE_SIZE;
use crate::resources::scene_manager::SceneManager;
use crate::scenes::game_scenes::game_scene::GameScene;
use crate::materials::material_manager::MaterialManager;
use crate::components::thing_component::{ThingComponent, ThingPositionComponent, ThingStatsComponent};
use crate::resources::scene_data::thing::Thing;

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

        //move tile because height 256 pixels;
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

        let mut thing_component: ThingComponent = Default::default();
        let mut thing_position_component: ThingPositionComponent = Default::default();
        let mut thing_stats_component: ThingStatsComponent = Default::default();

        copy_from_thing_to_entity_component(&mut thing_component, &mut thing_position_component, &mut thing_stats_component, thing);

        commands
        .spawn_bundle(SpriteSheetBundle {
            transform,
            sprite: TextureAtlasSprite::new(index as usize),
            texture_atlas: texture,
            ..Default::default()
        })
        .insert(thing_component)
        .insert(thing_position_component)
        .insert(thing_stats_component);
    }    
}

pub fn copy_from_thing_to_entity_component(
    thing_component: &mut ThingComponent, 
    thing_position: &mut ThingPositionComponent, 
    thing_stats: &mut ThingStatsComponent, 
    thing: &Thing,
) {
    thing_component.thing_type = thing.thing_type.clone();
    thing_component.id = thing.id; 
    thing_component.tile_index = thing.tile_index;
    thing_component.graphic_index = thing.graphic_index;

    thing_position.position.x = thing.position.x;
    thing_position.position.y = thing.position.y;
    thing_position.permissions = thing.permissions.to_vec();

    thing_stats.resists = thing.resists.clone();
    thing_stats.extra_stats = thing.extra_stats.clone();

}