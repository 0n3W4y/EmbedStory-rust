use serde::{ Serialize, Deserialize };

use crate::resources::scene_data::objects::body_structure::body_part::{ BodyPart, BodyPartType, HealthPoints };

use super::body_part::{PartStatus, PartType};

#[derive( Deserialize, Serialize, Debug, Clone )]
pub struct ThingBodyStructure {
    pub torso: BodyPart,
    current_health_points: HealthPoints,
    total_health_points: HealthPoints,
}

impl ThingBodyStructure {
    pub fn new() -> Self{
        return ThingBodyStructure{
            torso: BodyPart::new( BodyPartType::Torso ),
            current_health_points: HealthPoints::Current( 0 ),
            total_health_points: HealthPoints::Total( 0 ),
        }
    }

    pub fn get_total_health_points( &self ) -> i16 {
        match self.total_health_points {
            HealthPoints::Total( v ) => { return v; },
            _ => { panic!( "thing_body_structure.get_total_health_points. Wrong ENUM assign" )}
        };
    }

    pub fn get_current_health_points( &self ) -> i16 {
        match self.current_health_points {
            HealthPoints::Current( v ) => { return v; },
            _ => { panic!( "thing_body_structure.get_current_health_points. Wrong ENUM assign" )}
        };
    }

    pub fn calculate_total_health_points( &self ){
        let value = self.torso.get_total_health_points();
        self.set_total_health_points( value );
    }

    pub fn calculate_current_health_points( &self ){
        let value = self.torso.get_current_health_points();
        self.set_current_health_points( value );
    }

    pub fn get_available_outer_parts( &self ) -> Vec<&BodyPartType> {
        let mut result: Vec<&BodyPartType> = vec![];
        result.push( self.torso.get_body_part_type() );
        return result;
    }

    pub fn get_available_inner_parts_for_body_part( &self, body_part_type: &BodyPartType ) -> Vec<&BodyPartType>{
        let mut result = vec![];
        return result;
    }

    pub fn add_current_health_points( &self, part_type: &BodyPartType, value: HealthPoints ){
        match part_type {
            BodyPartType::Torso => { self.torso.add_current_health_points( value )},
            _ => {}
        }

        self.calculate_current_health_points();
    }

    pub fn substruct_current_health_points( &self, part_type: &BodyPartType, value: HealthPoints ){
        match part_type {
            BodyPartType::Torso => { self.torso.substruct_current_health_points( value )},
            _ => {}
        }

        self.calculate_current_health_points();
    }

    pub fn add_modifier_health_points( &self, part_type: &BodyPartType, value: HealthPoints ){
        match part_type {
            BodyPartType::Torso => { self.torso.add_modifier_health_points( value )},
            _ => {}
        }

        self.calculate_total_health_points();
        self.calculate_current_health_points();
    }

    pub fn substruct_modifier_health_points( &self, part_type: &BodyPartType, value: HealthPoints ){
        match part_type {
            BodyPartType::Torso => { self.torso.substruct_modifier_health_points( value )},
            _ => {}
        }
        
        self.calculate_total_health_points();
        self.calculate_current_health_points();
    }

    pub fn get_part_status( &self, part_type: &BodyPartType ) -> &PartStatus{
        match part_type{
            BodyPartType::Torso => { self.torso.get_part_status() },
            _ => { panic!( "Can't get part status in THING bodystructure from {:?}", part_type )}
        }
    }
    pub fn change_part_status_to( &self, part_type: &BodyPartType, part_status: PartStatus ){
        match part_type{
            BodyPartType::Torso => { self.torso.change_part_status( part_status ) },
            _ => { panic!( "Can't change part status in THING bodystructure from {:?}", part_type )}
        }
    }
    pub fn get_part_type( &self, body_part_type: &BodyPartType ) -> &PartType {
        match body_part_type {
            BodyPartType::Torso => { self.torso.get_part_type() },
            _ => { panic!( "Can't get part type in THING bodystructure from: {:?}", body_part_type )}
        }
    }
    pub fn change_part_type_to( &self, body_part_type: &BodyPartType, part_type: PartType ){
        match body_part_type {
            BodyPartType::Torso => { self.torso.change_part_type( part_type )},
            _ => { panic!( "Can't change part type in THING bodystructure from: {:?}", body_part_type )}
        }
    }

    fn set_total_health_points( &mut self, value: i16 ){
        self.total_health_points = HealthPoints::Total( value );
    }

    fn set_current_health_points( &mut self, value: i16 ){
        self.current_health_points = HealthPoints::Current( value );
    }
}
