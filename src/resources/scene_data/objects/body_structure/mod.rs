use serde::{ Deserialize, Serialize };

pub mod body_part;
pub mod humanoid_body_structure;
pub mod thing_body_structure;

use thing_body_structure::ThingBodyStructure;
use humanoid_body_structure::HumaniodBodyStructure;

use self::body_part::{BodyPart, BodyPartType};

#[derive( Clone, Deserialize, Serialize, Debug, Eq, PartialEq )]
pub enum BodyStructureType{
    Humanoid, // 2 hands, 2 legs
    Bogomol, // 2 hands, 4 legs
    Gorro, // Mortal Kombat, 4 hands, 2 legs
    Roach, // 6 legs
    Thing, // 1 torso
}

#[derive( Deserialize, Serialize, Debug )]
pub struct BodyStructure{
    structure_type: BodyStructureType,
    humanoid: Option<HumaniodBodyStructure>,
    thing: Option<ThingBodyStructure>
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

    pub fn get_available_outer_parts( &self ) -> Vec<&mut BodyPart>{
        let mut vec: Vec<&mut BodyPart> = match self.structure_type {
            BodyStructureType::Humanoid =>{
                self.humanoid.unwrap().get_available_outer_parts()
            },
            BodyStructureType::Thing => {
                self.thing.unwrap().get_available_outer_parts()
            },
            _ => {
                vec![]
            },
        };

        return vec;
    }

    pub fn get_available_inner_parts_for_body_part( &self, body_part_type: BodyPartType ) -> Vec<&mut BodyPart>{
        let vec: Vec<&mut BodyPart> = match self.structure_type {
            BodyStructureType::Humanoid => {
                self.humanoid.unwrap().get_available_inner_parts_for_body_part( body_part_type )
            },
            BodyStructureType::Thing => {
                self.thing.unwrap().get_available_inner_parts_for_body_part( body_part_type )
            },
            _ => {
                vec![]
            }
        };
        
        return vec;
    }

    pub fn add_health_points(){}
    pub fn substruct_health_points(){}

    pub fn get_total_health_points(){}
    pub fn get_current_health_points(){}
}