use serde::{ Deserialize, Serialize };

use crate::scenes::game_scenes::game_scene::GameScene;

use super::{scene_data::objects::{thing::{ Thing, ThingType, ThingConfig }, body_structure::body_part::BodyPartType}, deploy::Deploy};


pub struct ObjectManager{
    id: usize,
}

impl ObjectManager{
    pub fn new() -> Self{
        return ObjectManager{
            id: 0,
        };
    }

    pub fn generate_things_for_scene( &self, scene: &mut GameScene, deploy: &Deploy ){
        
    }

    pub fn generate_thing( &self, thing_type: &ThingType, deploy: &Deploy ) -> Thing{   
        let thing: Thing = match thing_type{
            ThingType::Boulder => {
                self.generate_boulder( deploy )
            },
            ThingType::Bush => {},
            ThingType::ClayWall => {},
            ThingType::FertileBush => {},
            ThingType::FertileBush => {},
            ThingType::FertileTree => {},
            ThingType::IronDoor => {},
            ThingType::IronWall => {},
            ThingType::Log => {},
            ThingType::ReinforcedIronDoor => {},
            _ =>{},
        };

        return thing;
    }

    pub fn generate_boulder( &self, deploy: &Deploy ) -> Thing{
        let id = self.create_id();
        let thing = Thing::new( id, ThingType::Boulder );

        //setup thing with dpeloy settings
        //let thing_setting = deploy.objects_deploy.thing.get_deploy( &thing.thing_type );
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

        thing.body_structure.add_modifier_health_points( &BodyPartType::Torso, config.health_points );
        thing.body_structure.calculate_total_health_points();
        thing.body_structure.calculate_current_health_points();
        
        return thing;
    }

    pub fn set_id( &mut self, id:usize ){
        self.id = id;
    }

    fn create_id( &mut self ) -> usize{
        let id = self.id;
        self.id += 1;
        return id;
    }
}