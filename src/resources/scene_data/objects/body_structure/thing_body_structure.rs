use serde::{ Serialize, Deserialize };

use crate::resources::scene_data::objects::body_structure::body_part::{ BodyPart, BodyPartType, HealthPoints };

#[derive( Deserialize, Serialize, Debug )]
pub struct ThingBodyStructure {
    pub torso: BodyPart,
    pub current_health_points: HealthPoints,
    pub total_health_points: HealthPoints,
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

    pub fn calculate_total_health_points( &self ){
        let value = self.torso.get_total_health_points();
        self.set_total_health_points( value );
    }

    pub fn calculate_current_health_points( &self ){
        let value = self.torso.get_current_health_points();
        self.set_current_health_points( value );
    }

    pub fn get_available_body_parts( &self ) -> Vec<&BodyPart> {
        let mut result = vec![];
        result.push( &self.torso );
        return result;
    }

    fn set_total_health_points( &mut self, value: i16 ){
        self.total_health_points = HealthPoints::Total( value );
    }

    fn set_current_health_points( &mut self, value: i16 ){
        self.current_health_points = HealthPoints::Current( value );
    }
}
