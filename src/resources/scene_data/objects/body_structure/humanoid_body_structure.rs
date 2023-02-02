use serde::{ Deserialize, Serialize };

use crate::resources::scene_data::objects::body_structure::body_part::BodyPart;

use super::body_part::{BodyPartType, HealthPoints, PartStatus};



#[derive( Deserialize, Serialize, Debug )]
struct HumanoidHeadStructure{
    pub head: BodyPart,
    pub left_eye: BodyPart,
    pub right_eye: BodyPart,
    pub nose: BodyPart,
    pub mouth: BodyPart,
    pub brain: BodyPart,
}

impl<'a> HumanoidHeadStructure{
    pub fn new() -> Self{
        return HumanoidHeadStructure{
            head: BodyPart::new( BodyPartType::Head ),
            left_eye: BodyPart::new( BodyPartType::Eye ),
            right_eye: BodyPart::new( BodyPartType::Eye ),
            nose: BodyPart::new( BodyPartType::Nose ),
            mouth: BodyPart::new( BodyPartType::Mouth ),
            brain: BodyPart::new( BodyPartType::Brain )
        }
    }

    pub fn get_available_outer_parts( &'a mut self, mut vec: &Vec<&'a mut BodyPart> ){
        let head_part_status = self.head.get_part_status();
        let left_eye_part_status = self.left_eye.get_part_status();
        let right_eye_part_status = self.right_eye.get_part_status();
        let nose_part_status = self.nose.get_part_status();
        let mouth_part_status = self.mouth.get_part_status();

        let part_status = PartStatus::Disrupted;

        if *head_part_status != part_status { vec.push( &mut self.head )};
        if *left_eye_part_status != part_status { vec.push( &mut self.left_eye )};
        if *right_eye_part_status != part_status { vec.push( &mut self.right_eye )};
        if *nose_part_status != part_status { vec.push( &mut self.nose )};
        if *mouth_part_status != part_status { vec.push( &mut self.mouth )};
    }

    pub fn get_available_inner_parts( &'a mut self, vec: &Vec<&'a mut BodyPart> ){
        let brain_part_status = self.brain.get_part_status();
        if *brain_part_status != PartStatus::Disrupted { vec.push( &mut self.brain )};
    }
}

#[derive( Deserialize, Serialize, Debug )]
struct HumanoidTorsoStructure {
    pub torso: BodyPart,
    pub left_lung: BodyPart,
    pub right_lung: BodyPart,
    pub heart: BodyPart,
    pub groin: BodyPart,
}

impl<'a> HumanoidTorsoStructure {
    pub fn new() -> Self {
        return HumanoidTorsoStructure {
            torso: BodyPart::new( BodyPartType::Torso ),
            left_lung: BodyPart::new( BodyPartType::Lung ),
            right_lung: BodyPart::new( BodyPartType::Lung ),
            heart: BodyPart::new( BodyPartType::Heart ),
            groin: BodyPart::new( BodyPartType::Groin ),
        }
    }

    pub fn get_available_outer_parts( &'a mut self, vec: &Vec<&'a mut BodyPart> ){
        let torso_part_status = self.torso.get_part_status();
        let groin_part_status = self.groin.get_part_status();

        let part_status = PartStatus::Disrupted;

        if *torso_part_status != part_status { vec.push( &mut self.torso )};
        if *groin_part_status != part_status { vec.push( &mut self.groin )};
    }

    pub fn get_available_inner_parts( &'a mut self, vec: &Vec<&'a mut BodyPart> ){
        let left_lung_part_status = self.left_lung.get_part_status();
        let right_lung_part_status = self.right_lung.get_part_status();
        let heart_part_status = self.heart.get_part_status();

        let part_status = PartStatus::Disrupted;

        if *left_lung_part_status != part_status { vec.push( &mut self.left_lung )};
        if *right_lung_part_status != part_status { vec.push( &mut self.right_lung )};
        if *heart_part_status != part_status { vec.push( &mut self.heart )};
    }
}

#[derive( Deserialize, Serialize, Debug )]
struct HumanoidHandStructure{
    pub arm: BodyPart,
    pub wrist: BodyPart,
}

impl<'a> HumanoidHandStructure{
    pub fn new() -> Self{
        return HumanoidHandStructure{
            arm: BodyPart::new( BodyPartType::Arm ),
            wrist: BodyPart::new( BodyPartType::Wrist )
        }
    }

    pub fn get_available_outer_parts( &'a mut self, vec: &Vec<&'a mut BodyPart> ){

    }

    pub fn get_available_inner_parts( &'a mut self, vec: &Vec<&'a mut BodyPart> ){
        
    }
}

#[derive( Deserialize, Serialize, Debug )]
struct HumanoidLegStructure{
    pub sole: BodyPart,
    pub foot: BodyPart,
}

impl HumanoidLegStructure {
    pub fn new() -> Self{
        return HumanoidLegStructure{
            sole: BodyPart::new( BodyPartType::Sole ),
            foot: BodyPart::new( BodyPartType::Foot )
        }        
    }
}

#[derive( Deserialize, Serialize, Debug )]
pub struct HumaniodBodyStructure {
    pub head: HumanoidHeadStructure,
    pub torso: HumanoidTorsoStructure,
    pub left_hand: HumanoidHandStructure,
    pub right_hand: HumanoidHandStructure,
    pub left_leg: HumanoidLegStructure,
    pub right_leg: HumanoidLegStructure,
    pub current_health_points: HealthPoints,
    pub total_health_points: HealthPoints,
}

impl<'a> HumaniodBodyStructure{
    pub fn new() -> Self {
        return HumaniodBodyStructure { 
            head: HumanoidHeadStructure::new(), 
            torso: HumanoidTorsoStructure::new(), 
            left_hand: HumanoidHandStructure::new(),
            right_hand: HumanoidHandStructure::new(),
            left_leg: HumanoidLegStructure::new(), 
            right_leg: HumanoidLegStructure::new(),
            current_health_points: HealthPoints::Current( 0 ), 
            total_health_points: HealthPoints::Total( 0 ), 
        };
    }

    pub fn get_available_outer_parts( &mut self ) -> Vec<&'a mut BodyPart>{
        let result: Vec<&'a mut BodyPart> = vec![];
        self.head.get_available_outer_parts( &result );
        self.torso.get_available_outer_parts( &result );
        self.right_hand.get_available_outer_parts( &result );
        self.left_hand.get_available_outer_parts( &result );
        self.left_leg.get_available_outer_parts( &result );
        self.right_leg.get_available_outer_parts( &result );

        return result;
    }
    pub fn get_available_inner_parts(){}
    pub fn get_available_inner_parts_for_body_part( &mut self, body_part_type: BodyPartType ) -> Vec<&mut BodyPart>{

    }

    pub fn add_health_points( &self, part_type: &mut BodyPart ){

    }

    pub fn substruct_health_points( &self, part_type: &mut BodyPart ){

    }
}



