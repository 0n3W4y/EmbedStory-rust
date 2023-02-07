use serde::{ Deserialize, Serialize };

use crate::resources::scene_data::objects::body_structure::body_part::{ BodyPart, BodyPartType };
use crate::resources::scene_data::objects::body_structure::body_part::HealthPoints;

use super::body_part::{PartStatus, PartType};

#[derive( Deserialize, Serialize, Debug, Clone )]
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

    pub fn get_available_outer_parts( &mut self ) -> Vec<&BodyPartType>{
        let mut result: Vec<&BodyPartType> = vec![];
        let part_status: PartStatus = PartStatus::Disrupted;

        if *self.head.get_part_status() != part_status { result.push( &self.head.get_body_part_type() )};
        if *self.left_eye.get_part_status() != part_status{ result.push( &self.left_eye.get_body_part_type() )};
        if *self.right_eye.get_part_status() != part_status{ result.push( &self.right_eye.get_body_part_type() )};
        if *self.nose.get_part_status() != part_status { result.push( &self.nose.get_body_part_type() )};
        if *self.mouth.get_part_status() != part_status { result.push( &self.mouth.get_body_part_type() )};
        if *self.torso.get_part_status() != part_status { result.push( &self.torso.get_body_part_type() )};
        if *self.groin.get_part_status() != part_status { result.push( &self.groin.get_body_part_type() )};
        if *self.left_arm.get_part_status() != part_status { result.push( &self.left_arm.get_body_part_type() )};
        if *self.right_arm.get_part_status() != part_status { result.push( &self.right_arm.get_body_part_type() )};
        if *self.left_wrist.get_part_status() != part_status { result.push( &self.left_wrist.get_body_part_type() )};
        if *self.right_wrist.get_part_status() != part_status { result.push( &self.right_wrist.get_body_part_type() )};
        if *self.left_sole.get_part_status() != part_status { result.push( &self.left_sole.get_body_part_type() )};
        if *self.left_foot.get_part_status() != part_status { result.push( &self.left_foot.get_body_part_type() )};
        if *self.right_sole.get_part_status() != part_status { result.push( &self.right_sole.get_body_part_type() )};
        if *self.right_foot.get_part_status() != part_status { result.push( &self.right_foot.get_body_part_type() )};

        return result;
    }

    pub fn get_available_inner_parts_for_body_part( &mut self, body_part_type: &BodyPartType ) -> Vec<&BodyPartType>{
        let mut result: Vec<&BodyPartType> = vec![];
        let part_status = PartStatus::Disrupted;

        match body_part_type {
            BodyPartType::Head => {
                result.push( &self.brain.get_body_part_type() );
            },
            BodyPartType::Torso => {
                result.push( &self.heart.get_body_part_type() );
                if *self.left_lung.get_part_status() != part_status { result.push( &self.left_lung.get_body_part_type() )};
                if *self.right_lung.get_part_status() != part_status { result.push( &self.right_lung.get_body_part_type() )};
            },
            _ => {},
        }

        return result;
    }

    pub fn add_current_health_points( &self, part_type: &BodyPartType, value: HealthPoints ){
        let container = self.get_body_part_mut( part_type );
        container.add_current_health_points( value );
    }

    pub fn substruct_current_health_points( &self, part_type: &BodyPartType, value: HealthPoints ){
        let container = self.get_body_part_mut( part_type );
        container.substruct_current_health_points( value );
    }

    pub fn add_modifier_health_points( &self, part_type: &BodyPartType, value: HealthPoints ){
        let container = self.get_body_part_mut( part_type );
        container.add_modifier_health_points( value );
    }

    pub fn substruct_modifier_health_points( &self, part_type: &BodyPartType, value: HealthPoints ){
        let container = self.get_body_part_mut( part_type );
        container.substruct_modifier_health_points( value );
    }

    pub fn get_total_health_points( &self ) -> i16{
        match self.total_health_points{
            HealthPoints::Total( v ) => { return v; },
            _ => { panic!( "humanoid_body_structure.get_total_health_points. Wrong ENUM assigned " )}
        };
    }

    pub fn get_current_health_points( &self ) -> i16{
        match self.current_health_points{
            HealthPoints::Current( v ) => { return v; },
            _ => { panic!( "humanoid_body_structure.get_current_health_points. Wrong ENUM assigned " )}
        };
    }

    pub fn calculate_total_health_points( &self ){
        let result = self.head.get_total_health_points() +
            self.left_eye.get_total_health_points() +
            self.right_eye.get_total_health_points() +
            self.nose.get_total_health_points() +
            self.mouth.get_total_health_points() +
            self.brain.get_total_health_points() +
            self.torso.get_total_health_points() +
            self.left_lung.get_total_health_points() +
            self.right_lung.get_total_health_points() +
            self.heart.get_total_health_points() +
            self.groin.get_total_health_points() +
            self.left_arm.get_total_health_points() +
            self.left_wrist.get_total_health_points() +
            self.right_arm.get_total_health_points() +
            self.right_wrist.get_total_health_points() +
            self.left_sole.get_total_health_points() +
            self.left_foot.get_total_health_points() +
            self.right_sole.get_total_health_points() +
            self.right_foot.get_total_health_points();

        self.total_health_points = HealthPoints::Total( result );
    }

    pub fn calculate_current_health_points( &self ){
        let result = self.head.get_current_health_points() +
            self.left_eye.get_current_health_points() +
            self.right_eye.get_current_health_points() +
            self.nose.get_current_health_points() +
            self.mouth.get_current_health_points() +
            self.brain.get_current_health_points() +
            self.torso.get_current_health_points() +
            self.left_lung.get_current_health_points() +
            self.right_lung.get_current_health_points() +
            self.heart.get_current_health_points() +
            self.groin.get_current_health_points() +
            self.left_arm.get_current_health_points() +
            self.left_wrist.get_current_health_points() +
            self.right_arm.get_current_health_points() +
            self.right_wrist.get_current_health_points() +
            self.left_sole.get_current_health_points() +
            self.left_foot.get_current_health_points() +
            self.right_sole.get_current_health_points() +
            self.right_foot.get_current_health_points();

        self.current_health_points = HealthPoints::Current( result );
    }

    pub fn get_part_status( &self, body_part_type: &BodyPartType ) -> &PartStatus{
        let container = self.get_body_part( body_part_type );
        return container.get_part_status();
    }

    pub fn change_part_status_to( &self, body_part_type: &BodyPartType, part_status: PartStatus ){
        let container = self.get_body_part_mut( body_part_type );
        container.change_part_status( part_status );
    }

    pub fn get_part_type( &self, body_part_type: &BodyPartType ) -> &PartType {
        let container = self.get_body_part( body_part_type );
        return container.get_part_type();
    }

    pub fn change_part_type_to( &self, body_part_type: &BodyPartType, part_type: PartType ){
        let container = self.get_body_part_mut( body_part_type );
        container.change_part_type( part_type );
    }

    fn get_body_part_mut( &mut self, body_part_type: &BodyPartType ) -> &mut BodyPart{
        let container: &mut BodyPart = match body_part_type {
            BodyPartType::Brain => { &mut self.brain },
            BodyPartType::Head => { &mut self.head },
            BodyPartType::Groin => { &mut self.groin },
            BodyPartType::Heart => { &mut self.heart },
            BodyPartType::LeftArm => { &mut self.left_arm },
            BodyPartType::LeftEye => { &mut self.left_eye },
            BodyPartType::LeftFoot => { &mut self.left_foot },
            BodyPartType::LeftLung => { &mut self.left_lung },
            BodyPartType::LeftSole => { &mut self.left_sole },
            BodyPartType::LeftWrist => { &mut self.left_wrist },
            BodyPartType::Mouth => { &mut self.mouth },
            BodyPartType::Nose => { &mut self.nose },
            BodyPartType::RightArm => { &mut self.right_arm },
            BodyPartType::RightEye => { &mut self.right_eye },
            BodyPartType::RightFoot => { &mut self.right_foot },
            BodyPartType::RightLung => { &mut self.right_lung },
            BodyPartType::RightSole => { &mut self.right_sole },
            BodyPartType::RightWrist => { &mut self.right_wrist },
            BodyPartType::Torso => { &mut self.torso },
            _ => { panic!( "humanoid_body_structure.get_body_part_mut. There is no part type: '{:?}' in Humanoid Body Structure", body_part_type )}
        };

        return container;
    }

    fn get_body_part( &self,  body_part_type: &BodyPartType ) -> &BodyPart{
        let container: & BodyPart = match body_part_type {
            BodyPartType::Brain => { & self.brain },
            BodyPartType::Head => { & self.head },
            BodyPartType::Groin => { & self.groin },
            BodyPartType::Heart => { & self.heart },
            BodyPartType::LeftArm => { & self.left_arm },
            BodyPartType::LeftEye => { & self.left_eye },
            BodyPartType::LeftFoot => { & self.left_foot },
            BodyPartType::LeftLung => { & self.left_lung },
            BodyPartType::LeftSole => { & self.left_sole },
            BodyPartType::LeftWrist => { & self.left_wrist },
            BodyPartType::Mouth => { & self.mouth },
            BodyPartType::Nose => { & self.nose },
            BodyPartType::RightArm => { & self.right_arm },
            BodyPartType::RightEye => { & self.right_eye },
            BodyPartType::RightFoot => { & self.right_foot },
            BodyPartType::RightLung => { & self.right_lung },
            BodyPartType::RightSole => { & self.right_sole },
            BodyPartType::RightWrist => { & self.right_wrist },
            BodyPartType::Torso => { & self.torso },
            _ => { panic!( "humanoid_body_structure.get_body_part_mut. There is no part type: '{:?}' in Humanoid Body Structure", body_part_type )}
        };

        return container;
    }
}



