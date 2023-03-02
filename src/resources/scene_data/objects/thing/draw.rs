use bevy::prelude::*;

use crate::resources::scene_manager::SceneManager;
use crate::scenes::game_scenes::game_scene::GameScene;
use crate::materials::material_manager::MaterialManager;
use crate::components::thing_component::ThingComponent;

pub const Z_POSITION: f32 = 0.2;

pub fn draw(
    mut commands: Commands,
    scene_manager: Res<SceneManager>,
    material_manager: Res<MaterialManager>,
){
    let mut new_msg: String = "".to_owned();
    let scene: &GameScene = scene_manager.get_current_game_scene();
    for tile in scene.tilemap.get_tilemap_tile_storage().iter().rev(){
        let thing_id = match tile.thing_type {
            Option::Some(v) => {v.1},
            Option::None => continue,
        };
        

        let mut msg = "Can not get thing from scne with id:".to_owned();
        msg.push_str(&thing_id.to_string());
        let thing = scene.get_thing_by_id(thing_id).unwrap_or_else(|| panic!("{:?}", msg));


        new_msg.push_str("; type:");
        new_msg.push_str(&thing.thing_type.to_string());
        new_msg.push_str("; pos.x:");
        new_msg.push_str(&thing.position.x.to_string());
        new_msg.push_str("; pos.y:");
        new_msg.push_str(&thing.position.y.to_string());
        new_msg.push_str("; tile_index:");
        new_msg.push_str(&thing.tile_index.to_string());
        new_msg.push_str(" | ");

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
    println!("{:?}", new_msg);
    /*
    for thing in scene.things.iter().rev(){
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
    */
}