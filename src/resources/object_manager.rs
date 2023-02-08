//use serde::{ Deserialize, Serialize };

use crate::scenes::game_scenes::game_scene::GameScene;
use crate::scenes::game_scenes::tilemap::tile::{Tile, GroundType};
use crate::resources::scene_data::objects::body_structure::body_part::HealthPoints;

use super::scene_data::objects::thing::{ Thing, ThingType, ThingConfig };
use super::scene_data::objects::body_structure::body_part::BodyPartType;
use super::deploy::Deploy;
use super::deploy_addiction::game_scene_biome_deploy::BiomeThings;


pub struct ObjectManager{
    id: usize,
}

impl ObjectManager{
    pub fn new() -> Self{
        return ObjectManager{
            id: 0,
        };
    }

    pub fn generate_things_for_scene( &mut self, scene: &mut GameScene, deploy: &Deploy, biome_things_setting: &BiomeThings ){
        //first, create rock things generated before in tilemap;
        self.generate_rock_things_for_scene( scene, deploy );
    }

    pub fn generate_thing( &mut self, thing_type: &ThingType, deploy: &Deploy ) -> Thing{
        let id = self.create_id();
        let mut thing = Thing::new( id, thing_type.clone() );

        //setup thing with dpeloy settings
        //let config = deploy.objects_deploy.thing.get_config( &thing.thing_type );
        let config = ThingConfig {
            can_harvested: true,
            can_be_destroied: true,
            can_repaired: false,
            electric_resist: 90,
            fire_resist: 90,
            kinetic_resist: 50,
            laser_resist: 20,
            plasma_resist: 10,
            health_points: 900,
        };

        thing.can_be_destroied = config.can_be_destroied;
        thing.can_harvested = config.can_harvested;
        thing.can_repaired = config.can_harvested;

        thing.resists.set_electric_modifier( config.electric_resist );
        thing.resists.set_fire_modifier( config.fire_resist );
        thing.resists.set_kinetic_modifier( config.kinetic_resist );
        thing.resists.set_laser_modifier( config.laser_resist );
        thing.resists.set_plasma_modifier( config.plasma_resist );

        thing.body_structure.add_modifier_health_points( &BodyPartType::Torso, HealthPoints::Modifier( config.health_points ));
        thing.body_structure.calculate_total_health_points();
        thing.body_structure.calculate_current_health_points();

        return thing;
    }

    pub fn get_thing_graphic_index_for_rock( &self, tile_storage: &Vec<Tile>, tilemap_height: u16, tilemap_total_tiles: usize,  x: u16, y: u16, thing_type: &ThingType ) -> u8{

        match thing_type {
            ThingType::Rock => {
                let top_index = (( y - 1 ) * tilemap_height + x) as isize;
                let left_index = (y * tilemap_height + ( x - 1 )) as isize;
                let right_index = (y * tilemap_height + ( x + 1)) as isize;
                let bot_index = (( y + 1 ) * tilemap_height + x) as isize;

                let top: bool = if top_index < 0 || top_index >= tilemap_total_tiles as isize { 
                    false 
                }else{ 
                    if tile_storage[ top_index as usize ].ground_type == GroundType::Rock { true } else { false }
                };

                let left: bool = if left_index < 0 || left_index >= tilemap_total_tiles as isize {
                    false
                }else{
                    if tile_storage[ left_index as usize ].ground_type == GroundType::Rock { true } else { false }
                };

                let right: bool = if right_index < 0 || right_index >= tilemap_total_tiles as isize {
                    false
                }else{
                    if tile_storage[ right_index as usize ].ground_type == GroundType::Rock { true } else { false }
                };

                let bottom: bool = if bot_index < 0 || bot_index >= tilemap_total_tiles as isize {
                    false
                }else{
                    if tile_storage[ bot_index as usize ].ground_type == GroundType::Rock { true } else { false }
                };

                if top && left && right && bottom {
                    return 0; // all
                }else if top && left && right && !bottom {
                    return 1; // top + left + right;
                }else if top && left && !right && bottom {
                    return 2; // top + left + bottom;
                }else if top && !left && right && bottom {
                    return 3; // top + right + bottom;
                }else if !top && left && right && bottom {
                    return 4; // left + right + bottom;
                }else if top && !left && !right && bottom {
                    return 5; // top + bottom;
                }else if !top && left && right && !bottom {
                    return 6; // left + right;
                }else if top && left && !right && !bottom {
                    return 7; // top + left;
                }else if top && !left && right && !bottom {
                    return 8; // top + right;
                }else if !top && left && !right && bottom {
                    return 9; // left + bottom;
                }else if !top && !left && right && bottom {
                    return 10; // right + bottom;
                }else if top && !left && !right && !bottom {
                    return  11; // top;
                }else if !top && left && !right && !bottom {
                    return 12; // left;
                }else if !top && !left && right && !bottom {
                    return 13; // right;
                }else if !top && !left && !right && bottom {
                    return 14; // bottom;
                }else{
                    return 15; // alone;
                }
            },
            _ => { 0 }
        }
    }

    pub fn set_id( &mut self, id:usize ){
        self.id = id;
    }

    fn generate_rock_things_for_scene( &mut self, scene: &mut GameScene, deploy: &Deploy ){
        let tilemap = &mut scene.tilemap;
        let tilemap_height = tilemap.get_tilemap_height();
        let tilemap_total_tiles = tilemap.get_total_tiles();
        
       

        for i in 0..tilemap.get_tilemap_tile_storage().len(){
            let ground_type: &GroundType = &tilemap.get_tilemap_tile_storage()[ i ].ground_type;
            if *ground_type == GroundType::Rock {
                let mut rock_thing: Thing = self.generate_thing( &ThingType::Rock, deploy );
                rock_thing.x = tilemap.get_tilemap_tile_storage()[ i ].x;
                rock_thing.y = tilemap.get_tilemap_tile_storage()[ i ].y;
                rock_thing.index = scene.things.len();

                let index = tilemap.get_tilemap_tile_storage()[ i ].index;
                tilemap.get_tile_by_index( index ).can_place_thing = false;
                tilemap.get_tile_by_index( index ).can_walk = false;
                tilemap.get_tile_by_index( index ).thing_type = Some( ThingType::Rock );
                tilemap.get_tile_by_index( index ).thing_id = rock_thing.id;

                let tilemap_storage: &Vec<Tile> = tilemap.get_tilemap_tile_storage();

                let graphics_index = self.get_thing_graphic_index_for_rock( tilemap_storage, tilemap_height, tilemap_total_tiles, rock_thing.x, rock_thing.y, &ThingType::Rock );
                rock_thing.graphic_index = graphics_index;

                scene.things.push( rock_thing );
            }
        }
    }

    fn create_id( &mut self ) -> usize{
        let id = self.id;
        self.id += 1;
        return id;
    }
}