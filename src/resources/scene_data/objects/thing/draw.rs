use bevy::prelude::*;

use crate::resources::scene_manager::SceneManager;
use crate::scenes::game_scenes::game_scene::GameScene;
use crate::materials::material_manager::MaterialManager;
use crate::components::thing_component::ThingComponent;

use super::spawn;

pub const Z_POSITION: f32 = 2.0;

pub fn draw(
    mut commands: Commands,
    scene_manager: Res<SceneManager>,
    material_manager: Res<MaterialManager>,
){
    let scene: &GameScene = scene_manager.get_current_game_scene();
    let total_tiles = scene.tilemap.get_total_tiles();
    for thing in scene.things.iter(){
        let x: f32 = thing.graphic_position.x;
        let y: f32 = thing.graphic_position.y;
        let index = thing.graphic_index;
        let thing_type = &thing.thing_type;

        let texture = material_manager
                    .game_scene
                    .things
                    .get_image(thing_type, index as usize);
        let new_z_position = Z_POSITION + thing.tile_index as f32 / total_tiles as f32;
        let transform = Transform::from_xyz(x, y, new_z_position); // third layer;
        let mut component: ThingComponent = Default::default();
        spawn::copy_from_thing_to_entity_component(&mut component, thing);

        commands
        .spawn_bundle(SpriteBundle {
            transform,
            texture,
            ..Default::default()
        })
        .insert(component);
    }
    
}