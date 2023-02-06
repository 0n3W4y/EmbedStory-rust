use serde::{ Deserialize, Serialize };

use crate::resources::scene_data::objects::body_structure::body_part::{ BodyPart, BodyPartType };
use crate::resources::scene_data::objects::body_structure::body_part::HealthPoints;

use super::body_part::PartStatus;

#[derive( Deserialize, Serialize, Debug )]
pub struct HumaniodBodyStructure {
    pub head: BodyPart,
    pub brain: BodyPart,
    pub left_eye: BodyPart,
    pub right_eye: BodyPart,
    pub nose: BodyPart,
    pub mouth: BodyPart,
    pub torso: BodyPart,
    pub left_lung: BodyPart,
    pub right_lung: BodyPart,
    pub heart: BodyPart,
    pub groin: BodyPart,
    pub left_arm: BodyPart,
    pub left_wrist: BodyPart,
    pub right_arm: BodyPart,
    pub right_wrist:BodyPart,
    pub left_sole: BodyPart,
    pub left_foot: BodyPart,
    pub right_sole: BodyPart,
    pub right_foot: BodyPart,
    current_health_points: HealthPoints,
    total_health_points: HealthPoints,
}

impl<'a> HumaniodBodyStructure{
    pub fn new() -> Self {
        return HumaniodBodyStructure { 
            head: BodyPart::new( BodyPartType::Head ), 
            brain: BodyPart::new( BodyPartType::Brain ),
            left_eye: BodyPart::new( BodyPartType::LeftEye ),
            right_eye: BodyPart::new( BodyPartType::RightEye ),
            nose: BodyPart::new( BodyPartType::Nose ),
            mouth: BodyPart::new( BodyPartType::Mouth ),
            torso: BodyPart::new( BodyPartType::Torso ),
            left_lung: BodyPart::new( BodyPartType::LeftLung ),
            right_lung: BodyPart::new( BodyPartType::RightLung ),
            heart: BodyPart::new( BodyPartType::Heart ),
            groin: BodyPart::new( BodyPartType::Groin ),
            left_arm: BodyPart::new( BodyPartType::LeftArm ),
            left_wrist: BodyPart::new( BodyPartType::LeftWrist ),
            right_arm: BodyPart::new( BodyPartType::RightArm ),
            right_wrist: BodyPart::new( BodyPartType::RightWrist ),
            left_sole: BodyPart::new( BodyPartType::LeftSole ),
            left_foot: BodyPart::new( BodyPartType::LeftFoot ),
            right_sole: BodyPart::new( BodyPartType::RightSole ),
            right_foot: BodyPart::new( BodyPartType::RightFoot ),            
            current_health_points: HealthPoints::Current( 0 ), 
            total_health_points: HealthPoints::Total( 0 ), 
        };
    }

    pub fn get_available_outer_parts( &mut self ) -> Vec<&mut BodyPart>{
        let mut result: Vec<&mut BodyPart> = vec![];
        let part_status: PartStatus = PartStatus::Disrupted;

        if *self.head.get_part_status() != part_status { result.push( &mut self.head )};
        if *self.left_eye.get_part_status() != part_status{ result.push( &mut self.left_eye )};
        if *self.right_eye.get_part_status() != part_status{ result.push( &mut self.right_eye )};
        if *self.nose.get_part_status() != part_status { result.push( &mut self.nose )};
        if *self.mouth.get_part_status() != part_status { result.push( &mut self.mouth )};
        if *self.torso.get_part_status() != part_status { result.push( &mut self.torso )};
        if *self.groin.get_part_status() != part_status { result.push( &mut self.groin )};
        if *self.left_arm.get_part_status() != part_status { result.push( &mut self.left_arm )};
        if *self.right_arm.get_part_status() != part_status { result.push( &mut self.right_arm )};
        if *self.left_wrist.get_part_status() != part_status { result.push( &mut self.left_wrist )};
        if *self.right_wrist.get_part_status() != part_status { result.push( &mut self.right_wrist )};
        if *self.left_sole.get_part_status() != part_status { result.push( &mut self.left_sole )};
        if *self.left_foot.get_part_status() != part_status { result.push( &mut self.left_foot )};
        if *self.right_sole.get_part_status() != part_status { result.push( &mut self.right_sole )};
        if *self.right_foot.get_part_status() != part_status { result.push( &mut self.right_foot )};

        return result;
    }

    pub fn get_available_inner_parts_for_body_part( &mut self, body_part_type: BodyPartType ) -> Vec<&mut BodyPart>{
        let mut result = vec![];
        let part_status = PartStatus::Disrupted;

        match body_part_type {
            BodyPartType::Head => {
                result.push( &mut self.brain );
            },
            BodyPartType::Torso => {
                result.push( &mut self.heart );
                if *self.left_lung.get_part_status() != part_status { result.push( &mut self.left_lung )};
                if *self.right_lung.get_part_status() != part_status { result.push( &mut self.right_lung )};
            },
            _ => {
                panic!(" There is no inner parts for part: {:?} .", body_part_type );
            },
        }

        return result;
    }

    pub fn add_health_points( &self, part_type: BodyPartType, value: HealthPoints ){

    }

    pub fn substruct_health_points( &self, part_type: BodyPartType, value: HealthPoints ){

    }

    pub fn add_health_points_modifier( &self, part_type: BodyPartType, value: HealthPoints ){

    }

    pub fn substruct_health_points_modifier( &self, part_type: BodyPartType, value: HealthPoints ){

    }
}



