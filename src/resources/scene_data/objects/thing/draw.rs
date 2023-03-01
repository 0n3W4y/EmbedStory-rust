use bevy::prelude::*;

use crate::scenes::game_scenes::game_scene::GameScene;
use crate::materials::material_manager::MaterialManager;
use crate::components::thing_component::ThingComponent;

pub const Z_POSITION: f32 = 0.2;

pub fn draw(
    mut commands: Commands,
    scene: Res<GameScene>,
    material_manager: Res<MaterialManager>,
){
    for thing in scene.things.iter(){
        let x: f32 = thing.graphic_position.x;
        let y: f32 = thing.graphic_position.y;
        let index = thing.graphic_index;
        let thing_type = &thing.thing_type;

        let texture = material_manager
                    .game_scene
                    .things
                    .get_image(thing_type, index as usize);
        let transform = Transform::from_xyz(x, y, Z_POSITION); // third layer;

        commands
        .spawn_bundle(SpriteBundle {
            transform,
            texture,
            ..Default::default()
        })
        .insert(ThingComponent{id: thing.id, tile_index: thing.tile_index});
    }
}