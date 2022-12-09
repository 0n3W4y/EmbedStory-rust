use serde::{ Serialize, Deserialize };

#[derive( Eq, ParticalEq, Serialize, Deserialize, Clone, Hash, Copy )]
pub enum BodyType{
    Head,
    Eye,
    Mouth,
    Nose,
    Ear,
    Arm,
    Leg,
    Torso,
    Brain,
    Lung,
    Heart
}

#[derive( Eq, ParticalEq, Serialize, Deserialize, Clone, Hash, Copy )]
pub enum PartType{
    Natural,
    Wood,
    Cybernetic,
    Mechanical,
}

#[derive( Eq, ParticalEq, Serialize, Deserialize, Clone, Hash, Copy )]
pub enum PartStatus{
    Healthy,
    Damaged,
    Broken,
    Disrupted,
}

pub struct BodyPart{
    bodypart_type: BodyType,
    current_health_points: f32,
    total_health_points: f32,
    modifier_health_points: f32,
    part_type: PartType,
    part_status: PartStatus,
}

impl BodyPart{
    pub fn new( body_type: BodyType ) -> Self{
        return BodyPart{
            bodypart_type: body_type,
            current_health_points: 0.0,
            total_health_points: 0.0,
            modifier_health_points: 0.0,
            part_type: PartType::Natural,
            part_status: PartStatus::Healthy,
        };
    }
    pub fn get_current_health_points( &self ) -> f32{
        return self.current_health_points;
    }

    pub fn get_total_health_points( &self ) -> f32{
        return self.total_health_points;
    }

    pub fn get_modified_health_points( &self ) -> f32{
        return self.modifier_health_points;
    }

    pub fn get_part_type( &self ) -> PartType{
        return self.part_type;
    }

    pub fn get_part_status( &self ) -> PartStatus{
        return self.part_status;
    }

    pub fn set_current_health_points( &mut self, value: f32 ){
        self.current_health_points = value;
    }

    pub fn set_total_health_points( &mut self, value: f32 ){
        self.total_health_points = value;
    }

    pub fn set_modifier_health_points( &mut self, value: f32 ){
        self.modifier_health_points = value;
    }

    pub fn set_part_type( &mut self, part_type: PartType ){
        self.part_type = part_type;
    }

    pub fn set_part_status( &mut self, part_status: PartStatus ){
        self.part_status = part_status;
    }
}