//use serde::{ Deserialize, Serialize };
use rand::Rng;

use crate::scenes::game_scenes::game_scene::GameScene;
use crate::scenes::game_scenes::tilemap::tile::{GroundType, Tile, TilePermissions};

use super::deploy::Deploy;
use super::deploy_addiction::game_scene_biome_deploy::BiomeThings;
use super::scene_data::objects::body_part::BodyPart;
use super::scene_data::objects::thing::{Thing, ThingType};

pub struct ObjectManager {
    id: usize,
}

impl ObjectManager {
    pub fn new() -> Self {
        return ObjectManager { id: 0 };
    }

    pub fn create_thing(&mut self, thing_type: &ThingType, deploy: &Deploy) -> Thing {
        let id = self.create_id();
        let config = deploy.objects_deploy.get_config(thing_type);

        let mut thing = Thing{
            id, 
            thing_type: thing_type.clone(),
            permissions: config.permissions.to_vec(),
            resists: config.resists.to_vec(),
            resists_cache : config.resists.to_vec(),
            ..Default::default()
        };
       
        for ( bodypart_type, hp ) in config.body_struct.iter(){
            let mut body_part = BodyPart {
                bodypart_type: bodypart_type.clone(),
                ..Default::default()
            };
            body_part.set_health_points(*hp);

            thing.body_structure.push(body_part);
        }
        

        return thing;
    }

    pub fn create_thing_on_tile(&self, thing_type: &ThingType, tile: &mut Tile, deploy: &Deploy) -> Thing{
        let mut thing = self.create_thing(thing_type, deploy);
        let config = deploy.objects_deploy.get_config(thing_type);
        let mut allow_tile_permissions:Vec<TilePermissions> = config.tile_allow_permissions.to_vec();
        let mut deny_tile_permissions: Vec<TilePermissions> = config.tile_deny_permissions.to_vec();
        let mut movement_ratio = config.movement_ratio;

        for permission in deny_tile_permissions.into_iter(){
            let index = tile.permissions.into_iter().position(|x|{ x == permission});
            match index{
                Option::Some(v) => {tile.permissions.remove(v);},
                Option::None => {},
            };
        };

        for permission in allow_tile_permissions.into_iter(){
            let index = tile.permissions.into_iter().position(|x|{ x == permission});
            match index{ 
                Option::Some(_) =>{},
                Option::None => tile.permissions.push(permission)
            };
        };

        thing.position.x = tile.position.x;
        thing.position.y = tile.position.y;
        thing.graphic_position.x = tile.graphic_position.x;
        thing.graphic_position.y = tile.graphic_position.y;

        tile.thing_type = (Some(ThingType::Rock), thing.id);
        return thing;
    }

    fn generate_rocks_for_scene(&mut self, scene: &mut GameScene, deploy: &Deploy) {
        let tilemap = &mut scene.tilemap;
        let tile_storage = tilemap.get_tilemap_tile_storage();

        for tile in tile_storage.iter_mut() {
            let ground_type: &GroundType = &tile.ground_type;
            let thing_type = ThingType::Rock; // default;
            if *ground_type == GroundType::Rock {
                let mut thing = self.create_thing_on_tile(&thing_type, tile, deploy);
                thing.index = scene.things.len();
                scene.things.push(thing);
            }
        }
    }

    pub fn generate_things_for_scene(
        &mut self,
        scene: &mut GameScene,
        deploy: &Deploy,
        biome_things_setting: &BiomeThings,
    ) {
        //first, create natural things;
        //like rocks, ores, trees, etc..
        for (key, percent) in biome_things_setting.natural_things.iter() {
            match *key {
                ThingType::Rock => self.generate_rocks_for_scene(scene, deploy),
                ThingType::CopperOre | ThingType::IronOre => {
                    self.generate_ores_for_scene(scene, deploy, key, *percent);
                }
                ThingType::Tree 
                    | ThingType::FertileTree
                    | ThingType::Bush
                    | ThingType::FertileBush => {self.generate_other_things_for_scene(
                    scene,
                    deploy,
                    key,
                    *percent

                );},
                _ => {}
            }
        }
    }

    pub fn generate_pattern_things_for_scene(&self, scene: &mut GameScene, deploy: &Deploy) {
        //TODO: Generate Houses, cities etc...
    }

    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn generate_other_things_for_scene(
        &mut self,
        scene: &mut GameScene,
        deploy: &Deploy,
        thing_type: &ThingType,
        thing_type_percent: f32,
    ) {
        let mut rng = rand::thread_rng();
        let tilemap = &mut scene.tilemap;
        let tilemap_total_tiles = tilemap.get_total_tiles();
        let tilemap_width = tilemap.get_tilemap_width();
        let tilemap_height = tilemap.get_tilemap_height();
        let total_objects = (tilemap_total_tiles as f32 * thing_type_percent / 100.0) as usize;
        let tile_size = tilemap.get_tile_size();

        if total_objects >= (tilemap_total_tiles / 2) {
            println!( "object_manager.generate_other_thing_for_scene. Warning! Total things '{:?}' most than 50% of total tiles in tilemap", thing_type );
        };

        let mut number = 0; //count how many things r generated in tilemap;
        while total_objects > number {
            let random_x: i32 = rng.gen_range(0..(tilemap_width as i32 + 1));
            let random_y: i32 = rng.gen_range(0..(tilemap_height as i32 + 1));
            let tile_index: usize = (random_y * tilemap_height as i32 + random_x) as usize;
            let tile = tilemap.get_tile_by_index(tile_index);

            //check for thing in current tile
            if tile.thing_type.0 != Option::None && matches!(tile.permissions.into_iter().find(|&x|{ x == TilePermissions::PlaceThing}), Some(TilePermissions::PlaceThing)) {
                number += 1;

                let mut thing = self.generate_thing(thing_type, deploy);
                thing.position.x = random_x;
                thing.position.y = random_y;
                thing.graphic_position.x = random_x as f32 * tile_size as f32;
                thing.graphic_position.y = random_y as f32 * tile_size as f32;

                let tilemap_storage: &Vec<Tile> = tilemap.get_tilemap_tile_storage();
                thing.graphic_index = self.get_thing_graphic_index(
                    tilemap_storage,
                    tilemap_height,
                    tilemap_total_tiles,
                    random_x,
                    random_y,
                    thing_type,
                );

                let tile_mut = tilemap.get_tile_by_index_mut(tile_index);
                tile_mut.thing_type = (Some(thing_type.clone()), thing.id);
                let not_allowed_permissions = vec![
                    TilePermissions::PlaceFloor, 
                    TilePermissions::PlaceStuff, 
                    TilePermissions::PlaceThing, 
                    TilePermissions::RemoveFloor,
                    ];
                let allowed_permisisions = vec![
                    TilePermissions::Fog, 
                    TilePermissions::Roof,
                    TilePermissions::Walk
                    ];
                tile_mut.can_place_floor = false;
                tile_mut.can_place_thing = false;
                tile_mut.can_place_stuff = false;
                //tile.movement_ratio = 400; TODO: add this into config;

                scene.things.push(thing);
            } else {
                continue;
            }
        }
    }

    fn generate_ores_for_scene(&mut self, scene: &mut GameScene, deploy: &Deploy, thing_type: &ThingType, percent: f32) {}

    fn create_id(&mut self) -> usize {
        let id = self.id;
        self.id += 1;
        return id;
    }

    pub fn get_thing_graphic_index(
        &self,
        tile_storage: &Vec<Tile>,
        tilemap_height: u16,
        tilemap_total_tiles: usize,
        x: u16,
        y: u16,
        thing_type: &ThingType,
    ) -> u8 {
        match thing_type {
            ThingType::Rock => {
                let top_index = ((y - 1) * tilemap_height + x) as isize;
                let left_index = (y * tilemap_height + (x - 1)) as isize;
                let right_index = (y * tilemap_height + (x + 1)) as isize;
                let bot_index = ((y + 1) * tilemap_height + x) as isize;

                let top: bool = if top_index < 0 || top_index >= tilemap_total_tiles as isize {
                    false
                } else {
                    if tile_storage[top_index as usize].ground_type == GroundType::Rock {
                        true
                    } else {
                        false
                    }
                };

                let left: bool = if left_index < 0 || left_index >= tilemap_total_tiles as isize {
                    false
                } else {
                    if tile_storage[left_index as usize].ground_type == GroundType::Rock {
                        true
                    } else {
                        false
                    }
                };

                let right: bool = if right_index < 0 || right_index >= tilemap_total_tiles as isize
                {
                    false
                } else {
                    if tile_storage[right_index as usize].ground_type == GroundType::Rock {
                        true
                    } else {
                        false
                    }
                };

                let bottom: bool = if bot_index < 0 || bot_index >= tilemap_total_tiles as isize {
                    false
                } else {
                    if tile_storage[bot_index as usize].ground_type == GroundType::Rock {
                        true
                    } else {
                        false
                    }
                };

                if top && left && right && bottom {
                    return 0; // all
                } else if top && left && right && !bottom {
                    return 1; // top + left + right;
                } else if top && left && !right && bottom {
                    return 2; // top + left + bottom;
                } else if top && !left && right && bottom {
                    return 3; // top + right + bottom;
                } else if !top && left && right && bottom {
                    return 4; // left + right + bottom;
                } else if top && !left && !right && bottom {
                    return 5; // top + bottom;
                } else if !top && left && right && !bottom {
                    return 6; // left + right;
                } else if top && left && !right && !bottom {
                    return 7; // top + left;
                } else if top && !left && right && !bottom {
                    return 8; // top + right;
                } else if !top && left && !right && bottom {
                    return 9; // left + bottom;
                } else if !top && !left && right && bottom {
                    return 10; // right + bottom;
                } else if top && !left && !right && !bottom {
                    return 11; // top;
                } else if !top && left && !right && !bottom {
                    return 12; // left;
                } else if !top && !left && right && !bottom {
                    return 13; // right;
                } else if !top && !left && !right && bottom {
                    return 14; // bottom;
                } else {
                    return 15; // alone;
                }
            }
            _ => 0,
        }
    }
}
