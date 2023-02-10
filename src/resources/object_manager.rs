//use serde::{ Deserialize, Serialize };
use rand::Rng;

use crate::resources::scene_data::objects::body_part::HealthPoints;
use crate::resources::scene_data::objects::resists::Resist;
use crate::scenes::game_scenes::game_scene::GameScene;
use crate::scenes::game_scenes::tilemap::tile::{GroundType, Tile, Position};

use super::deploy::Deploy;
use super::deploy_addiction::game_scene_biome_deploy::BiomeThings;
use super::scene_data::objects::body_part::BodyPartType;
use super::scene_data::objects::thing::{Thing, ThingType};

pub struct ObjectManager {
    id: usize,
}

impl ObjectManager {
    pub fn new() -> Self {
        return ObjectManager { id: 0 };
    }

    pub fn generate_things_for_scene(
        &mut self,
        scene: &mut GameScene,
        deploy: &Deploy,
        biome_things_setting: &BiomeThings,
    ) {
        //first, create natural things generated before in tilemap;
        //like rocks, ores, trees, etc..
        for i in 0..biome_things_setting.natural_things.len() {
            match biome_things_setting.natural_things[i] {
                ThingType::Rock => self.generate_rocks_for_scene(scene, deploy),
                ThingType::CopperOre | ThingType::IronOre => {
                    self.generate_ores_for_scene(scene, deploy)
                }
                ThingType::Tree | ThingType::FertileTree => self.generate_other_things_for_scene(
                    scene,
                    deploy,
                    &biome_things_setting.natural_things[i],
                    biome_things_setting.natural_things_value[i],
                ),
                _ => {}
            }
        }
    }

    pub fn generate_pattern_things_for_scene(&self, scene: &mut GameScene, deploy: &Deploy) {
        //TODO: Generate Houses, cities etc...
    }

    pub fn generate_thing(&mut self, thing_type: &ThingType, deploy: &Deploy) -> Thing {
        let id = self.create_id();
        let mut thing = Thing::new(id, thing_type.clone());

        //setup thing with dpeloy settings
        let config = deploy.objects_deploy.get_config(thing_type);

        thing.can_be_destroied = config.can_be_destroied;
        thing.can_harvested = config.can_harvested;
        thing.can_repaired = config.can_harvested;

        for resist in thing.resists.iter_mut() {
            match resist {
                Resist::Electric(v) => *v = config.electric_resist,
                Resist::Fire(v) => *v = config.fire_resist,
                Resist::Kinetic(v) => *v = config.kinetic_resist,
                Resist::Laser(v) => *v = config.laser_resist,
                Resist::Plasma(v) => *v = config.plasma_resist,
                _ => {}
            }
        }
        
        //Torso body part;
        thing.body_structure[0].set_health_points(config.health_points, config.health_points, config.health_points);
        

        return thing;
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

    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn generate_rocks_for_scene(&mut self, scene: &mut GameScene, deploy: &Deploy) {
        let tilemap = &mut scene.tilemap;
        let tilemap_height = tilemap.get_tilemap_height();
        let tilemap_total_tiles = tilemap.get_total_tiles();
        let tile_size = tilemap.get_tile_size();

        for i in 0..tilemap.get_tilemap_tile_storage().len() {
            let ground_type: &GroundType = &tilemap.get_tilemap_tile_storage()[i].ground_type;
            if *ground_type == GroundType::Rock {
                let mut rock_thing: Thing = self.generate_thing(&ThingType::Rock, deploy);

                let tile_copy = tilemap.get_tilemap_tile_storage()[i]; // is copy?
                rock_thing.position = tile_copy.position;
                rock_thing.graphic_position = tile_copy.graphic_position;
                rock_thing.index = scene.things.len();

                let tilemap_storage: &Vec<Tile> = tilemap.get_tilemap_tile_storage();

                let graphics_index = self.get_thing_graphic_index(
                    tilemap_storage,
                    tilemap_height,
                    tilemap_total_tiles,
                    rock_thing.position.0,
                    rock_thing.position.1,
                    &ThingType::Rock,
                );
                rock_thing.graphic_index = graphics_index;

                //let index = tilemap.get_tilemap_tile_storage()[ i ].index;
                let tile = tilemap.get_tile_by_index_mut(i);
                tile.can_place_thing = false;
                tile.can_place_floor = false;
                tile.can_place_stuff = false;
                tile.can_remove_floor = false;
                tile.can_walk = false;
                tile.thing_type = Some(ThingType::Rock);
                tile.thing_id = rock_thing.id;

                scene.things.push(rock_thing);
            }
        }
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
            let random_x: u16 = rng.gen_range(0..(tilemap_width + 1));
            let random_y: u16 = rng.gen_range(0..(tilemap_height + 1));
            let tile_index: usize = (random_y * tilemap_height + random_x) as usize;
            let tile = tilemap.get_tile_by_index(tile_index);

            //check for thing in current tile
            if tile.thing_type != Option::None && tile.can_place_thing {
                number += 1;

                let mut thing = self.generate_thing(thing_type, deploy);
                thing.x = random_x;
                thing.y = random_y;
                thing.graphic_x = random_x as u32 * tile_size as u32;
                thing.graphic_y = random_y as u32 * tile_size as u32;

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
                tile_mut.thing_type = Some(thing_type.clone());
                tile_mut.thing_id = thing.id;
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

    fn generate_ores_for_scene(&mut self, scene: &mut GameScene, deploy: &Deploy) {}

    fn create_id(&mut self) -> usize {
        let id = self.id;
        self.id += 1;
        return id;
    }
}
