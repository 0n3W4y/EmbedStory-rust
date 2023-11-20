use bevy::prelude::*;

use crate::components::{IdentificationComponent, PositionComponent, DamageTextComponent, AttributesComponent};
use crate::config::TILE_SIZE;
use crate::resources::scene_manager::SceneManager;
use crate::scenes::game_scenes::game_scene::GameScene;
use crate::materials::material_manager::MaterialManager;
use crate::components::thing_component::{ThingComponent, ThingPermissionsComponent};
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
    for thing in scene.things.get_all_things().iter(){
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
        let tile_id = match scene.tilemap.get_tile_by_position(thing.position.x, thing.position.y){
            Some(v) => v.id,
            None => {
                println!("Can not get tile with x: {}, y: {}", thing.position.x, thing.position.y);
                continue;
            },
        };
        let new_z_position = Z_POSITION + ((total_tiles as f32 - tile_id as f32) / total_tiles as f32); // tile with index 0 have a higher z-order, with 10000 - lower z-order;
        let transform = Transform::from_xyz(x, y, new_z_position);

        let mut identification_component: IdentificationComponent = Default::default();
        let mut thing_component: ThingComponent = Default::default();
        let mut position_component: PositionComponent = Default::default();
        let mut attributes_component: AttributesComponent = Default::default();
        let mut permissions_component: ThingPermissionsComponent = Default::default();
        let damage_text_component: DamageTextComponent = Default::default();

        copy_from_thing_to_entity_component(
            &mut identification_component, 
            &mut thing_component, 
            &mut position_component, 
            &mut attributes_component, 
            &mut permissions_component, 
            thing
        );

        commands
        .spawn_bundle(SpriteSheetBundle {
            transform,
            sprite: TextureAtlasSprite::new(index as usize),
            texture_atlas: texture,
            ..Default::default()
        })
        .insert(identification_component)
        .insert(thing_component)
        .insert(position_component)
        .insert(permissions_component)
        .insert(damage_text_component)
        .insert(attributes_component);
    }    
}

pub fn copy_from_thing_to_entity_component(
    identification_component: &mut IdentificationComponent,
    thing_component: &mut ThingComponent, 
    position_component: &mut PositionComponent, 
    attributes_component: &mut AttributesComponent, 
    permissions_component: &mut ThingPermissionsComponent,
    thing: &Thing,
) {
    thing_component.thing_type = thing.thing_type.clone();
    identification_component.id = thing.id; 
    thing_component.graphic_index = thing.graphic_index;

    position_component.position.x = thing.position.x;
    position_component.position.y = thing.position.y;
    permissions_component.permissions = thing.permissions.to_vec();

    attributes_component.attributes = thing.attributes.clone();
    attributes_component.attributes_cache = thing.attributes.clone();

}