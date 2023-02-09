use serde::{ Deserialize, Serialize };

pub mod body_part;
pub mod humanoid_body_structure;
pub mod thing_body_structure;

use thing_body_structure::ThingBodyStructure;
use humanoid_body_structure::HumaniodBodyStructure;

use self::body_part::{ BodyPartType, HealthPoints, PartStatus, PartType };

#[derive( Clone, Deserialize, Serialize, Debug, Eq, PartialEq )]
pub enum BodyStructureType{
    Humanoid, // 2 hands, 2 legs
    Bogomol, // 2 hands, 4 legs
    Gorro, // Mortal Kombat, 4 hands, 2 legs
    Roach, // 6 legs
    Thing, // 1 torso
}

#[derive( Deserialize, Serialize, Debug, Clone )]
pub struct BodyStructure{
    pub structure_type: BodyStructureType,
    pub humanoid: Option<HumaniodBodyStructure>,
    pub thing: Option<ThingBodyStructure>
}

impl BodyStructure{
    pub fn new( body_type: BodyStructureType ) -> Self{
        let result: BodyStructure = match body_type {
            BodyStructureType::Humanoid => {
                BodyStructure {
                    structure_type: body_type.clone(),
                    humanoid: Some( HumaniodBodyStructure::new() ),
                    thing: None
                }
            },
            BodyStructureType::Roach => {
                BodyStructure {
                    structure_type: body_type.clone(),
                    humanoid: None,
                    thing: None
                }
            },
            BodyStructureType::Bogomol => {
                BodyStructure {
                    structure_type: body_type.clone(),
                    humanoid: None,
                    thing: None
                }
            },
            BodyStructureType::Gorro => {
                BodyStructure {
                    structure_type: body_type.clone(),
                    humanoid: None,
                    thing: None
                }
            },
            BodyStructureType::Thing => {
                BodyStructure { 
                    structure_type: body_type.clone(), 
                    humanoid: None, 
                    thing: Some( ThingBodyStructure::new() ) 
                }
            },
        };

        return result;
    }

    pub fn get_available_outer_parts( &self ) -> Vec<&BodyPartType>{
        let vec: Vec<&BodyPartType> = match self.structure_type {
            BodyStructureType::Humanoid =>{
                self.humanoid.as_ref().unwrap().get_available_outer_parts()
            },
            BodyStructureType::Thing => {
                self.thing.as_ref().unwrap().get_available_outer_parts()
            },
            _ => {
                vec![]
            },
        };

        return vec;
    }

    pub fn get_available_inner_parts_for_body_part( &self, body_part_type: &BodyPartType ) -> Vec<&BodyPartType>{
        let vec: Vec<&BodyPartType> = match self.structure_type {
            BodyStructureType::Humanoid => {
                self.humanoid.as_ref().unwrap().get_available_inner_parts_for_body_part( body_part_type )
            },
            BodyStructureType::Thing => {
                self.thing.as_ref().unwrap().get_available_inner_parts_for_body_part( body_part_type )
            },
            _ => {
                vec![]
            }
        };
        
        return vec;
    }

    pub fn add_current_health_points( &mut self, part_type: &BodyPartType, value: HealthPoints ){
        match self.structure_type {
            BodyStructureType::Humanoid => { self.humanoid.as_mut().unwrap().add_current_health_points( part_type, value )},
            BodyStructureType::Thing => { self.thing.as_mut().unwrap().add_current_health_points( part_type, value )},
            _ => {},
        }
    }
    pub fn substruct_current_health_points( &mut self, part_type: &BodyPartType, value: HealthPoints ){
        match self.structure_type {
            BodyStructureType::Humanoid => { self.humanoid.as_mut().unwrap().substruct_current_health_points( part_type, value )},
            BodyStructureType::Thing => { self.thing.as_mut().unwrap().substruct_current_health_points( part_type, value )},
            _ => {},
        }
    }

    pub fn add_modifier_health_points( &mut self, part_type: &BodyPartType, value: HealthPoints ){
        match self.structure_type {
            BodyStructureType::Humanoid => { self.humanoid.as_mut().unwrap().add_modifier_health_points( part_type, value )},
            BodyStructureType::Thing => { self.thing.as_mut().unwrap().add_modifier_health_points( part_type, value )},
            _ => {},
        }
    }

    pub fn substruct_modifier_health_points( &mut self, part_type: &BodyPartType, value: HealthPoints ){
        match self.structure_type {
            BodyStructureType::Humanoid => { self.humanoid.as_mut().unwrap().substruct_modifier_health_points( part_type, value )},
            BodyStructureType::Thing => { self.thing.as_mut().unwrap().substruct_modifier_health_points( part_type, value )},
            _ => {},
        }
    }

    pub fn get_total_health_points( &self ) -> i16 {
        match self.structure_type {
            BodyStructureType::Humanoid => { self.humanoid.as_ref().unwrap().get_total_health_points() },
            BodyStructureType::Thing => { self.thing.as_ref().unwrap().get_total_health_points() },
            _ => { 0 },
        }
    }
    pub fn get_current_health_points( &self ) ->i16 {
        match self.structure_type {
            BodyStructureType::Humanoid => { self.humanoid.as_ref().unwrap().get_current_health_points() },
            BodyStructureType::Thing => { self.thing.as_ref().unwrap().get_current_health_points() },
            _ => { 0 },
        }
    }

    pub fn change_part_status_to( &mut self, part_type: &BodyPartType, part_status: PartStatus ){
        match self.structure_type {
            BodyStructureType::Humanoid => { self.humanoid.as_mut().unwrap().change_part_status_to( part_type, part_status )},
            BodyStructureType::Thing => { self.thing.as_mut().unwrap().change_part_status_to( part_type, part_status )},
            _ => {},
        }
    }

    pub fn get_part_status( &self, part_type: &BodyPartType ) -> &PartStatus{
        match self.structure_type {
            BodyStructureType::Humanoid => { self.humanoid.as_ref().unwrap().get_part_status( part_type )},
            BodyStructureType::Thing => { self.thing.as_ref().unwrap().get_part_status( part_type )}
            _ => { &PartStatus::Healthy },
        }
    }

    pub fn change_part_type_to( &mut self, body_part_type: &BodyPartType, part_type: PartType ){
        match self.structure_type {
            BodyStructureType::Humanoid => { self.humanoid.as_mut().unwrap().change_part_type_to( body_part_type, part_type )},
            BodyStructureType::Thing => { self.thing.as_mut().unwrap().change_part_type_to( body_part_type, part_type )},
            _ => {},
        }
    }

    pub fn get_part_type( &self, body_part_type: &BodyPartType ) -> &PartType {
        match self.structure_type {
            BodyStructureType::Humanoid => { self.humanoid.as_ref().unwrap().get_part_type( body_part_type )},
            BodyStructureType::Thing => { self.thing.as_ref().unwrap().get_part_type( body_part_type )},
            _ => { &PartType::Natural },
        }
    }

    pub fn calculate_total_health_points( &mut self ){
        match self.structure_type {
            BodyStructureType::Humanoid => { self.humanoid.as_mut().unwrap().calculate_total_health_points() },
            BodyStructureType::Thing => { self.thing.as_mut().unwrap().calculate_total_health_points() },
            _ => {},
        }
    }

    pub fn calculate_current_health_points( &mut self ){
        match self.structure_type {
            BodyStructureType::Humanoid => { self.humanoid.as_mut().unwrap().calculate_current_health_points() },
            BodyStructureType::Thing => { self.thing.as_mut().unwrap().calculate_current_health_points() },
            _ => {},
        }
    }

}