//use serde::{ Deserialize, Serialize };
use rand::Rng;

use crate::scenes::game_scenes::game_scene::GameScene;
use crate::scenes::game_scenes::tilemap::tile::{GroundType, Tile, TilePermissions};
use crate::scenes::game_scenes::tilemap::Tilemap;

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

        let mut thing = Thing {
            id,
            thing_type: thing_type.clone(),
            permissions: config.permissions.to_vec(),
            resists: config.resists.to_vec(),
            resists_cache: config.resists.to_vec(),
            ..Default::default()
        };

        for (bodypart_type, hp) in config.body_struct.iter() {
            let mut body_part = BodyPart {
                bodypart_type: bodypart_type.clone(),
                ..Default::default()
            };
            body_part.set_health_points(*hp);

            thing.body_structure.push(body_part);
        }

        return thing;
    }

    pub fn create_thing_on_tile(
        &mut self,
        thing_type: &ThingType,
        tile: &mut Tile,
        deploy: &Deploy,
    ) -> Thing {
        let mut thing = self.create_thing(thing_type, deploy);
        let config = deploy.objects_deploy.get_config(thing_type);
        let allow_tile_permissions: Vec<TilePermissions> = config.tile_allow_permissions.to_vec();
        let deny_tile_permissions: Vec<TilePermissions> = config.tile_deny_permissions.to_vec();

        for permission in deny_tile_permissions.iter() {
            let index = tile.permissions.iter().position(|x| x == permission);
            match index {
                Option::Some(v) => {
                    tile.permissions.remove(v);
                }
                Option::None => {}
            };
        }

        for permission in allow_tile_permissions.iter() {
            let index = tile.permissions.iter().position(|x| x == permission);
            match index {
                Option::Some(_) => {}
                Option::None => tile.permissions.push(permission.clone()),
            };
        }

        thing.position.x = tile.position.x;
        thing.position.y = tile.position.y;
        thing.graphic_position.x = tile.graphic_position.x;
        thing.graphic_position.y = tile.graphic_position.y;

        tile.thing_type = Some((ThingType::Rock, thing.id));
        return thing;
    }

    fn generate_rocks_for_scene(&mut self, scene: &mut GameScene, deploy: &Deploy) {
        let tilemap = &mut scene.tilemap;
        let tile_storage = tilemap.get_tilemap_tile_storage_mut();

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
                },
                _ => {
                    self.generate_other_things_for_scene(scene, deploy, key, *percent);
                }
            }
        }

        //after all, spread indexes for all things we create
        self.spread_indexes_for_things( &mut scene.things, &scene.tilemap );
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
        let half_tilemap_width = tilemap_width / 2;
        let half_tilemap_height = tilemap_height / 2;
        let mut total_objects = (tilemap_total_tiles as f32 * thing_type_percent / 100.0) as usize;

        if total_objects >= (tilemap_total_tiles / 2) {
            println!( "object_manager.generate_other_thing_for_scene. Warning! Total things '{:?}' most than 50% of total tiles in tilemap", thing_type );
        };

        let mut number = 0; //protction fro endless loop;
        while total_objects > 0 {
            let start_range_x = -(half_tilemap_width as i32);
            let end_range_x = half_tilemap_width as i32;
            let random_x: i32 = rng.gen_range(start_range_x..end_range_x);

            let start_range_y = -(half_tilemap_height as i32);
            let end_range_y = half_tilemap_height as i32;
            let random_y: i32 = rng.gen_range(start_range_y..end_range_y);

            let tile_index: usize = ((random_y + half_tilemap_height as i32)* tilemap_height as i32 + (random_x + half_tilemap_width as i32)) as usize;
            let tile = tilemap.get_tile_by_index_mut(tile_index);

            //check for thing in current tile
            if matches!(
                tile.permissions
                    .iter()
                    .find(|&x| {x == &TilePermissions::PlaceThing}),
                Some(TilePermissions::PlaceThing)
            ){
                let mut thing = self.create_thing_on_tile(thing_type, tile, deploy);
                thing.index = scene.things.len();
                scene.things.push(thing);
                total_objects -= 1;
            }else{
                number += 1;
                total_objects += 1;
                if 10 <= number { // protect from endless loop, too much objects on tilamp;
                    println!("object_manager.generate_other_things_for_scene. Breaking the loop with crateing thing on :'{:?}'", thing_type );
                    break;
                };
            }
        }
    }

    fn generate_ores_for_scene(
        &mut self,
        scene: &mut GameScene,
        deploy: &Deploy,
        thing_type: &ThingType,
        percent: f32,
    ) {
        let mut rng = rand::thread_rng();
        let mut rock_thing_storage: Vec<(&Thing, usize)> = vec![];

        // collect all things with type "rock" into vec;
        for i in 0..scene.things.len(){
            let thing = &scene.things[i];
            if thing.thing_type == ThingType::Rock {
                rock_thing_storage.push((thing, i));
            };
        }

        let max_things = rock_thing_storage.len();
        let mut max_ore_things = (max_things as f32 * percent / 100.0) as usize;

        if max_things <= max_ore_things{
            println!(
                "object_manager.generate_ores_for_scene. Break generation of '{:?}', because no all things in map '{:?}' < '{:?}' ore things in biome",
                thing_type,
                max_things,
                max_ore_things
                );
            return;
        }


        while max_ore_things > 0 {
            let random_index = rng.gen_range(0..rock_thing_storage.len());
            let thing_index_to_replace = rock_thing_storage[random_index].1;
            let rock_thing = &things_storage[thing_index_to_replace];
            let mut ore_thing = self.create_thing(thing_type, deploy);
            let tile = scene.tilemap.get_tile_by_index_mut(value);
        }
        //TODO:: получить лен() получить thing, попнуть его из вектора, повторить в while.


    }

    fn create_id(&mut self) -> usize {
        let id = self.id;
        self.id += 1;
        return id;
    }

    fn spread_indexes_for_things( &self, thing_storage: &mut Vec<Thing>, tilemap: &Tilemap ){
        for thing in thing_storage.iter_mut(){
            match thing.thing_type {
                ThingType::CopperOre 
                | ThingType::IronOre 
                | ThingType::IronWall 
                | ThingType::Rock 
                | ThingType::SteelWall 
                | ThingType::StoneWall 
                | ThingType::WoodenWall => {
                    let index = self.find_graphic_index_for_thing(tilemap, thing.position.x, thing.position.y, &thing.thing_type);
                    thing.graphic_index = index;
                },
                _ => {}
            }
            
        }
    }

    pub fn find_graphic_index_for_thing(
        &self,
        tilemap: &Tilemap,
        x: i32,
        y: i32,
        thing_type: &ThingType,
    ) -> u8 {
        let tile_storage = tilemap.get_tilemap_tile_storage();
        let tilemap_width = tilemap.get_tilemap_width();
        let tilemap_height = tilemap.get_tilemap_height();
        let tilemap_total_tiles = tilemap.get_total_tiles();
        let half_tilemap_width = tilemap_width / 2;
        let half_tilemap_height = tilemap_height / 2;

        let top_index: i32 = (y + 1 + half_tilemap_height as i32) * tilemap_height as i32
            + x
            + half_tilemap_width as i32;

        let left_top_index: i32 = (y + 1 + half_tilemap_height as i32) * tilemap_height as i32 
            + x
            - 1
            + half_tilemap_width as i32;

        let left_index: i32 = (y + half_tilemap_height as i32) * tilemap_height as i32 
            + x 
            - 1
            + half_tilemap_width as i32;

        let right_top_index: i32 = (y + 1 + half_tilemap_height as i32) * tilemap_height as i32
            + 1
            + x
            + half_tilemap_width as i32;

        let right_index: i32 = (y + half_tilemap_height as i32) * tilemap_height as i32
            + x
            + 1
            + half_tilemap_width as i32;

        let bottom_index: i32 = (y - 1 + half_tilemap_height as i32) * tilemap_height as i32
            + x
            + half_tilemap_width as i32;

        let left_bottom_index: i32 =(y - 1 + half_tilemap_height as i32) * tilemap_height as i32 
            + x 
            - 1
            + half_tilemap_width as i32;

        let right_bottom_index: i32 = (y - 1 + half_tilemap_height as i32) * tilemap_height as i32
            + x
            + 1
            + half_tilemap_width as i32;

        let vec_of_indexes = vec![
            top_index,
            left_top_index,
            left_index,
            right_top_index,
            right_index,
            bottom_index,
            left_bottom_index,
            right_bottom_index,
        ];

        let mut vec_of_bool: Vec<bool> = vec![];

        for index in vec_of_indexes.iter() {
            let new_bool = if *index < 0 || *index as usize >= tilemap_total_tiles {
                false
            } else {
                match tile_storage[*index as usize].thing_type.0 {
                    Option::Some(v) => {
                        match *thing_type {
                            ThingType::Rock | ThingType::CopperOre | ThingType::IronOre => {
                                if v == ThingType::Rock 
                                    || v == ThingType::CopperOre
                                    || v == ThingType::IronOre {
                                        true
                                } else {
                                    false
                                }
                            },
                            ThingType::IronWall | ThingType::SteelWall | ThingType::WoodenWall | ThingType::StoneWall => {
                                if v == ThingType::IronWall
                                    || v == ThingType::SteelWall
                                    || v == ThingType::WoodenWall
                                    || v == ThingType::StoneWall {
                                        true
                                } else {
                                    false
                                }
                            },
                            _ => {
                                if v == *thing_type {
                                    true
                                } else {
                                    false
                                }
                            }
                        }
                    }
                    Option::None => false,
                }
            };

            vec_of_bool.push(new_bool);
        }

        return tilemap.get_index_for_graphic_placement(vec_of_bool);
    }
}
