use serde::{ Deserialize, Serialize };

pub mod body_part;
pub mod humanoid_body_structure;
pub mod thing_body_structure;

use thing_body_structure::ThingBodyStructure;
use humanoid_body_structure::HumaniodBodyStructure;

#[derive( Clone, Deserialize, Serialize, Debug, Eq, PartialEq )]
pub enum BodyStructureType{
    Humanoid,
    Bogomol,
    Gorro, // Mortal Kombat,
    Roach,
    Thing,
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
}