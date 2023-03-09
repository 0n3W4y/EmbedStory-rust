//use serde::{ Deserialize, Serialize };
use rand::Rng;

use crate::scenes::game_scenes::game_scene::GameScene;
use crate::scenes::game_scenes::tilemap::tile::{GroundType, Tile, TilePermissions, CoverType};
use crate::scenes::game_scenes::tilemap::Tilemap;
use crate::scenes::game_scenes::tilemap;

use super::deploy::Deploy;
use super::deploy_addiction::game_scene_biome_deploy::BiomeThings;
use super::scene_data::objects::body_part;
use super::scene_data::objects::body_part::BodyPart;
use super::scene_data::objects::thing::{Thing, ThingType};
use super::scene_data::objects::thing;

#[derive(Default)]
pub struct ObjectManager {
    id: usize,
}

impl ObjectManager {

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

        for (bodypart_type, hp) in config.body_structure.iter() {
            let mut body_part = BodyPart {
                bodypart_type: bodypart_type.clone(),
                ..Default::default()
            };
            body_part::set_health_points(&mut body_part,*hp);
            println!("{:?}", body_part.get_current_health_points());

            thing.body_structure.push(body_part);
        }
        thing::calculate_total_health_points(&mut thing);
        thing::calculate_current_health_points(&mut thing);

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
                Option::None => continue,
            };
        }

        for permission in allow_tile_permissions.iter() {
            let index = tile.permissions.iter().position(|x| x == permission);
            match index {
                Option::Some(_) => continue,
                Option::None => tile.permissions.push(permission.clone()),
            };
        }

        thing.position.x = tile.position.x;
        thing.position.y = tile.position.y;
        thing.graphic_position.x = tile.graphic_position.x;
        thing.graphic_position.y = tile.graphic_position.y;
        thing.tile_index = tile.index;

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
                let thing = self.create_thing_on_tile(&thing_type, tile, deploy);
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
        //first get all "free" tiles, where i can put a Thing except rocks;
        let mut vec_of_free_tiles: Vec<usize> = vec![];
        for tile in scene.tilemap.get_tilemap_tile_storage_mut().iter(){
            if matches!(
                tile.permissions
                    .iter()
                    .find(|&x|{x == &TilePermissions::PlaceThing}),
                Some(TilePermissions::PlaceThing)
            ) {
                vec_of_free_tiles.push(tile.index);
            };
        };

        self.generate_rocks_for_scene(scene, deploy);

        //Generate other things, without Rock things;
        for (key, percent) in biome_things_setting.natural_things.iter() {
            match *key {
                ThingType::Rock => {},
                ThingType::CopperOre | ThingType::IronOre => {
                    self.generate_ores_for_scene(scene, deploy, key, *percent);
                },
                _ => {
                    self.generate_other_things_for_scene(scene, deploy, key, *percent, &mut vec_of_free_tiles);

                }
            }
        }

        //after all, spread indexes for all things we create;
        self.spread_indexes_for_things(&mut scene.things, &scene.tilemap);
        // sorting vec by evolving tile_index for spawn things and spread Z-Order;
        //scene.things.sort_by(|a, b| b.tile_index.cmp(&a.tile_index));

    }

    pub fn generate_pattern_things_for_scene(&self, scene: &mut GameScene, deploy: &Deploy) {
        //TODO: Generate Houses, cities etc...
    }

    fn generate_other_things_for_scene(
        &mut self,
        scene: &mut GameScene,
        deploy: &Deploy,
        thing_type: &ThingType,
        percent: f32,
        vec_of_free_tiles: &mut Vec<usize>
    ) {
        let tilesize = scene.tilemap.get_tile_size();
        let half_tilesize: u16 = tilesize / 2;
        let mut rng = rand::thread_rng();
        let tilemap = &mut scene.tilemap;
        let tilemap_total_tiles = tilemap.get_total_tiles();
        let tilemap_half_width_height = (tilemap.get_tilemap_height() as usize + tilemap.get_tilemap_width() as usize) / 2;
        let mut number: usize = 0;
        let mut total_objects = (tilemap_total_tiles as f32 * percent / 100.0) as usize;

        if total_objects * 2 >= (vec_of_free_tiles.len()) {
            println!( 
                "object_manager.generate_other_thing_for_scene. Warning! Total things '{:?}' most than 50% of free tiles in tilemap for things",
                thing_type 
                );
        };

        while total_objects > 0 {
            //TODO: Create graphics indexes for thing; Like SmallTree Normal Tree, BigTree; Or add this into enums and create images;

            let random_index = rng.gen_range(0..vec_of_free_tiles.len());
            let tile_index: usize = vec_of_free_tiles[random_index];
            let tile = tilemap.get_tile_by_index_mut(tile_index);

            if (*thing_type == ThingType::Tree
                || *thing_type == ThingType::FertileTree
                || *thing_type == ThingType::Bush
                || *thing_type == ThingType::FertileBush)
                && (tile.ground_type == GroundType::RockEnvironment 
                || tile.cover_type == CoverType::RockyRoad
                || tile.cover_type == CoverType::WoodenFloor
            ){
                number += 1;
                if number >= tilemap_half_width_height {
                    // protect from endless loop, too much objects on tilamp;
                    println!(
                        "object_manager.generate_other_things_for_scene. Breaking the loop with creating thing:'{:?}'. Current objects not generated: {:?}",
                        thing_type,
                        total_objects 
                    );
                    break;
                }
            }

            let mut thing = self.create_thing_on_tile(thing_type, tile, deploy);
            if *thing_type == ThingType::Tree
            || *thing_type == ThingType::FertileTree {
                thing.graphic_position.y = thing.graphic_position.y + half_tilesize as f32;
            };
            scene.things.push(thing);
            total_objects -= 1;
            vec_of_free_tiles.remove(random_index);
        }
    }

    fn generate_ores_for_scene(
        &mut self,
        scene: &mut GameScene,
        deploy: &Deploy,
        thing_type: &ThingType,
        percent: f32
    ) {
        let mut rng = rand::thread_rng();
        let mut rock_thing_storage: Vec<usize> = vec![];

        // collect all things with type "rock" into vec;
        for i in 0..scene.things.len() {
            if scene.things[i].thing_type == ThingType::Rock {
                rock_thing_storage.push(i);
            };
        }

        let max_things = rock_thing_storage.len();
        let mut max_ore_things = (max_things as f32 * percent / 100.0) as usize;

        if max_things + 10 <= max_ore_things { // 10 - magic number ;
            println!(
                "object_manager.generate_ores_for_scene. Break generation of '{:?}', because all things in map '{:?}' <= '{:?}' ore things in biome config",
                thing_type,
                max_things,
                max_ore_things
                );
            return;
        }

        while max_ore_things > 0 {
            let random_index = rng.gen_range(0..rock_thing_storage.len());
            let thing_index_to_replace = rock_thing_storage[random_index];
            let tile_index_to_replace = scene.things[thing_index_to_replace].tile_index;
            let tile = scene.tilemap.get_tile_by_index_mut(tile_index_to_replace);
            let mut ore_thing = self.create_thing_on_tile(thing_type, tile, deploy);

            ore_thing.graphic_index = scene.things[thing_index_to_replace].graphic_index;
            scene.things[thing_index_to_replace] = ore_thing;
            max_ore_things -= 1;
        }
    }

    fn create_id(&mut self) -> usize {
        let id = self.id;
        self.id += 1;
        return id;
    }

    fn spread_indexes_for_things(&self, thing_storage: &mut Vec<Thing>, tilemap: &Tilemap) {
        // TODO: do this for log and boulder things;
        for thing in thing_storage.iter_mut() {
            match thing.thing_type {
                ThingType::CopperOre
                | ThingType::IronOre
                | ThingType::IronWall
                | ThingType::Rock
                | ThingType::SteelWall
                | ThingType::StoneWall
                | ThingType::WoodenWall => {
                    let index = self.find_graphic_index_for_thing(
                        tilemap,
                        thing.position.x,
                        thing.position.y,
                        &thing.thing_type,
                    );
                    thing.graphic_index = index;
                }
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

        let left_top_index: i32 = (y + 1 + half_tilemap_height as i32) * tilemap_height as i32 + x
            - 1
            + half_tilemap_width as i32;

        let left_index: i32 = (y + half_tilemap_height as i32) * tilemap_height as i32 + x - 1
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

        let left_bottom_index: i32 =
            (y - 1 + half_tilemap_height as i32) * tilemap_height as i32 + x - 1
                + half_tilemap_width as i32;

        let right_bottom_index: i32 = (y - 1 + half_tilemap_height as i32) * tilemap_height as i32
            + x
            + 1
            + half_tilemap_width as i32;

        let vec_of_indexes = vec![
            top_index,
            left_index,
            right_index,
            bottom_index,
            left_top_index,
            right_top_index,
            left_bottom_index,
            right_bottom_index,
        ];

        let mut vec_of_bool: Vec<bool> = vec![];

        for index in vec_of_indexes.iter() {
            let new_bool = if *index < 0 || *index as usize >= tilemap_total_tiles {
                false
            } else {
                match tile_storage[*index as usize].thing_type {
                    Option::Some((v, _)) => match *thing_type {
                        ThingType::Rock | ThingType::CopperOre | ThingType::IronOre => {
                            if v == ThingType::Rock
                            || v == ThingType::CopperOre
                            || v == ThingType::IronOre {
                                true
                            } else {
                                false
                            }
                        }
                        ThingType::IronWall
                        | ThingType::SteelWall
                        | ThingType::WoodenWall
                        | ThingType::StoneWall => {
                            if v == ThingType::IronWall
                            || v == ThingType::SteelWall
                            || v == ThingType::WoodenWall
                            || v == ThingType::StoneWall {
                                true
                            } else {
                                false
                            }
                        }
                        _ => {
                            if v == *thing_type {
                                true
                            } else {
                                false
                            }
                        }
                    },
                    Option::None => false,
                }
            };

            vec_of_bool.push(new_bool);
        }

        return tilemap::generate::get_index_for_graphic_placement(vec_of_bool);
    }
}
