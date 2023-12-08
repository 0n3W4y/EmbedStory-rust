//use serde::{ Deserialize, Serialize };
use rand::Rng;

use crate::scenes::game_scenes::game_scene::GameScene;
use crate::scenes::game_scenes::tilemap::tile::{GroundType, Tile, TilePermissions, CoverType, Position};
use crate::scenes::game_scenes::tilemap;

use super::deploy::Deploy;
use super::scene_data::thing::{Thing, ThingType, ThingConfig};


#[derive(Default)]
pub struct ThingManager {
    id: usize,
}

impl ThingManager {

    pub fn create_thing(&mut self, thing_type: &ThingType, config: &ThingConfig) -> Thing {
        let id = self.create_id();
        let thing = Thing {
            id,
            thing_type: thing_type.clone(),
            permissions: config.permissions.to_vec(),
            resists: config.resists.clone(),
            attributes: config.attributes.clone(),
            attributes_cache: config.attributes.clone(),
            ..Default::default()
        };
        return thing;
    }

    pub fn create_thing_on_tile(
        &mut self,
        thing_type: &ThingType,
        tile: &mut Tile,
        deploy: &Deploy,
    ) -> Thing {
        let config = deploy.objects_deploy.get_config(thing_type);
        let mut thing = self.create_thing(thing_type, config);
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

        return thing;
    }

    fn generate_rocks_for_scene(&mut self, scene: &mut GameScene, deploy: &Deploy) {
        let tile_storage = scene.tilemap.get_tilemap_tile_storage_mut();
        for tile in tile_storage.iter_mut() {
            let ground_type: &GroundType = &tile.ground_type;
            let thing_type = ThingType::Rock; // default;
            if *ground_type == GroundType::Rock {
                match tile.permissions.iter().find(|&&x| x == TilePermissions::Walk) {
                    Some(_) => continue,
                    None => {
                        let thing = self.create_thing_on_tile(&thing_type, tile, deploy);
                        scene.things.store(thing);
                    }
                }                
            }
        }
    }

    pub fn generate_things_for_scene(
        &mut self,
        scene: &mut GameScene,
        deploy: &Deploy
    ) {
        //first get all "free" tiles, where i can put a Thing except rocks;
        let location_config = deploy.game_scene.get_scene_setting(&scene.location);
        let biome_config = deploy.game_scene_biome.get_biome_setting(&location_config.biome_type);

        let mut vec_of_free_tiles: Vec<usize> = vec![];
        let tile_storage = scene.tilemap.get_tilemap_tile_storage();
        for tile in tile_storage.iter(){
            match tile.permissions.iter().find(|&&x| x == TilePermissions::PlaceThing) {
                Some(_) => vec_of_free_tiles.push(tile.id),
                None => {},
            }
        };

        for (key, percent) in biome_config.objects.things.natural_things.iter() {
            match *key {
                ThingType::Rock => self.generate_rocks_for_scene(scene, deploy),
                ThingType::CopperOre | ThingType::IronOre => {
                    self.generate_ores_for_scene(scene, deploy, key, *percent);
                },
                _ => {
                    self.generate_other_things_for_scene(scene, deploy, key, *percent, &mut vec_of_free_tiles);

                }
            }
        }

        self.spread_indexes_for_things(&mut scene.things.rocks_and_ores);           //spread graphic indexes in rocks and ores;
        //scene.things.sort_by(|a, b| b.tile_index.cmp(&a.tile_index));                         // sorting vec by evolving tile_index for spawn things and spread Z-Order;

    }

    //pub fn generate_pattern_things_for_scene(&self, scene: &mut GameScene, deploy: &Deploy) {
        //TODO: Generate Houses, cities etc...
    //}

    fn generate_other_things_for_scene(
        &mut self,
        scene: &mut GameScene,
        deploy: &Deploy,
        thing_type: &ThingType,
        percent: f32,
        vec_of_free_tiles: &mut Vec<usize>
    ) {
        //let tilesize = scene.tilemap.get_tile_size();
        //let half_tilesize: u16 = tilesize / 2;
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
                && (tile.cover_type == CoverType::RockyRoad
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

            let thing = self.create_thing_on_tile(thing_type, tile, deploy);
            scene.things.store(thing);
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
        let mut rock_things: Vec<usize> = vec![];
        for thing in scene.things.rocks_and_ores.iter() {
            if thing.thing_type == ThingType::Rock {
                rock_things.push(thing.id);
            }
        }

        let mut rng = rand::thread_rng();
        let max_things = rock_things.len();
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
            let random_index = rng.gen_range(0..rock_things.len());
            let thing_id_to_replace = rock_things[random_index];
            match scene.things.rocks_and_ores.iter_mut().find(|x| x.id == thing_id_to_replace){
                Some(v) => {
                    let tile = match scene.tilemap.get_tile_by_position_mut(v.position.x, v.position.y){
                        Some(v) => v,
                        None => {
                            println!("can not det tile by position x: {}, y: {}", v.position.x, v.position.y);
                            continue;
                        }
                    };
                    let mut ore_thing = self.create_thing_on_tile(thing_type, tile, deploy);
                    ore_thing.graphic_index = v.graphic_index;
                    *v = ore_thing;
                    max_ore_things -= 1;
                },
                None => {},
            }
        }
    }

    fn create_id(&mut self) -> usize {
        let id = self.id;
        self.id += 1;
        return id;
    }

    fn spread_indexes_for_things(&self, thing_storage: &mut Vec<Thing>) {
        let clone_thing_storage = thing_storage.clone();
        for thing in thing_storage.iter_mut() {
            let index = self.find_graphic_index_for_thing(
                thing.position.x,
                thing.position.y,
                &clone_thing_storage,
            );
            thing.graphic_index = index;
        }
    }

    pub fn find_graphic_index_for_thing(                        //предполагается, что в thing_storage будет собраны things по типу одинаковые.
        &self,
        x: i32,
        y: i32,
        thing_storage: &Vec<Thing>,
    ) -> u8 {
        let top_index: Position<i32> = Position {x, y: y + 1};
        let left_top_index = Position {x: x - 1, y: y + 1};
        let left_index = Position {x: x - 1, y};
        let right_top_index = Position {x: x + 1, y: y + 1};
        let right_index = Position {x: x + 1, y};
        let bottom_index = Position {x , y: y - 1};
        let left_bottom_index= Position {x: x - 1, y: y - 1};
        let right_bottom_index = Position {x: x + 1, y: y - 1};

        let vec_of_thing_positions: Vec<Position<i32>> = vec![
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

        for index in vec_of_thing_positions.iter() {
            let mut new_bool = false;
            for thing in thing_storage.iter() {
                if thing.position.x == index.x && thing.position.y == index.y {
                    new_bool = true;
                }
            }
            vec_of_bool.push(new_bool);
        }

        return tilemap::generate::get_index_for_graphic_placement(vec_of_bool);
    }
}
