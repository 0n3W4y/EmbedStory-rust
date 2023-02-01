use serde::{ Deserialize, Serialize };

use crate::resources::scene_data::character::body_part::BodyPart;

#[derive( Clone, Deserialize, Serialize, Debug, Eq, PartialEq )]
enum BodyStructureType{
    Humanoid,
    Bogomol,
    Gorro, // Mortal Kombat,
    Roach,
}

pub struct BodyStructure{
    structure_type: BodyStructureType,

}

#[derive( Deserialize, Serialize, Debug )]
pub struct HumaniodBodyStructure{
    pub head: BodyPart,
    pub brain: BodyPart,
    pub nose: BodyPart,
    pub left_eye: BodyPart,
    pub right_eye: BodyPart,
    pub mouth: BodyPart,
    pub torso: BodyPart,
    pub left_lung: BodyPart,
    pub right_lung: BodyPart,
    pub heart: BodyPart,
    pub left_leg: BodyPart,
    pub right_leg: BodyPart,
    pub current_health_points: i16,
    pub total_health_points: i16,
}

impl HumaniodBodyStructure{
    pub fn new() -> Self{
        return HumaniodBodyStructure { 
            head: BodyPart::new(), 
            brain: (), 
            nose: (), 
            left_eye: (), 
            right_eye: (), 
            mouth: (), 
            torso: (), 
            left_lung: (), 
            right_lung: (), 
            heart: (), 
            left_leg: (), 
            right_leg: (), 
            current_health_points: (), 
            total_health_points: () 
        };
    }
}

#[derive( Deserialize, Serialize, Debug )]
pub struct ThingBodyStructure {
    pub torso: BodyPart,
    pub current_health_points: i16,
    pub total_health_points: i16,
}


