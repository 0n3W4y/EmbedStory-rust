use serde::{ Serialize, Deserialize };

#[derive( Serialize, Deserialize, Debug )]
pub enum HealthPoints{
    Current( i16 ),
    Total( i16 ),
    Modifier( i16 ),
}

#[derive( Eq, PartialEq, Serialize, Deserialize, Clone, Debug )]
pub enum BodyPartType{
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

#[derive( Eq, PartialEq, Serialize, Deserialize, Clone, Debug )]
pub enum PartType{
    Natural,
    Wood,
    Cybernetic,
    Mechanical,
}

#[derive( Eq, PartialEq, Serialize, Deserialize, Clone, Debug )]
pub enum PartStatus{
    Healthy,
    Damaged,
    Broken,
    Disrupted,
}

impl PartStatus{
    pub fn get_percent( part_status: PartStatus ) -> u8 {
        return match part_status {
            PartStatus::Healthy => 100, // 61 - 100,
            PartStatus::Damaged => 60, // 31 - 60,
            PartStatus::Broken => 30, // 1 - 30,
            PartStatus::Disrupted => 0, // 0 - -100,
        }
    }
}

#[derive( Deserialize, Serialize, Debug )]
pub struct BodyPart{
    bodypart_type: BodyPartType,
    current_health_points: HealthPoints,
    total_health_points: HealthPoints,
    modifier_health_points: HealthPoints,
    part_type: PartType,
    part_status: PartStatus,
}

impl BodyPart{
    pub fn new( body_type: BodyPartType ) -> Self{
        return BodyPart{
            bodypart_type: body_type,
            current_health_points: HealthPoints::Current( 0 ),
            total_health_points: HealthPoints::Total( 0 ),
            modifier_health_points: HealthPoints::Modifier( 0 ),
            part_type: PartType::Natural,
            part_status: PartStatus::Healthy,
        };
    }
    pub fn get_current_health_points( &self ) -> i16{
        match self.current_health_points {
            HealthPoints::Current( v ) => { v },
            _ => { panic!( "body_parts.get_current_health_points. Wrond ENUM assigned!")},
        }
    }

    pub fn add_current_health_points( &mut self, value: HealthPoints ){
        let current_value = match value{
            HealthPoints::Current( v ) => { v },
            _ => { panic!( "body_part.add_current_health_points. Wrong value for Current Health Points" )}
        };

        let old_value = self.get_current_health_points();
        let total_value: i16 = self.get_total_health_points();
        let new_value: i16 = old_value + current_value;
        if new_value < total_value {
            self.set_current_health_points( new_value );
        }else{
            self.set_current_health_points( total_value );
        };
    }

    pub fn substruct_current_health_points( &mut self, value: HealthPoints ){
        let current_value = match value{
            HealthPoints::Current( v ) => { v },
            _ => { panic!( "body_part.add_current_health_points. Wrong value for Current Health Points" )}
        };

        let old_value = self.get_current_health_points();
        let total_value: i16 = self.get_total_health_points();
        let new_value: i16 = old_value - current_value;
        if new_value < 0 {
            self.set_current_health_points( new_value );
        }else{
            self.set_current_health_points( 0 );
        };
    }

    pub fn get_total_health_points( &self ) -> i16{
        match self.total_health_points{
            HealthPoints::Total( v ) => { v },
            _ => { panic!( "body_parts.get_total_health_points. Wrond ENUM assigned!")},
        }
    }

    pub fn add_total_health_points( &mut self, value: HealthPoints ){
        let current_value = match value{
            HealthPoints::Total( v ) => { v },
            _ => { panic!( "body_part.add_current_health_points. Wrong value for Total Health Points" )}
        };

        let old_value = self.get_total_health_points();
        let new_value: i16 = old_value + current_value;
        self.set_total_health_points( new_value );
        
        //change current health while total change;
        let current_health: i16 = self.get_current_health_points();
        self.set_current_health_points( current_health + current_value );
    }

    pub fn substruct_total_health_points( &mut self, value: HealthPoints ){
        let current_value = match value{
            HealthPoints::Total( v ) => { v },
            _ => { panic!( "body_part.add_current_health_points. Wrong value for Total Health Points" )}
        };

        let old_value = self.get_total_health_points();
        let new_value: i16 = old_value - current_value;
        if new_value <= 1 {
            self.set_total_health_points( 1 );
            self.set_current_health_points( 1 );
        }else{
            self.set_total_health_points( new_value );
            let current_health: i16 = self.get_current_health_points();
            let new_current_health: i16 = current_health - current_value;
            if new_current_health <= 1 {
                self.set_current_health_points( 1 );
            }else{
                self.set_current_health_points( new_current_health );
            }
        }
    }

    pub fn get_modifier_health_points( &self ) -> i16{
        match self.modifier_health_points {
            HealthPoints::Modifier( v ) => { v },
            _ => { panic!( "body_parts.get_modifier_health_points. Wrond ENUM assigned!")},
        }
    }

    pub fn add_modifier_health_points( &mut self, value: HealthPoints ){
        let current_value = match value{
            HealthPoints::Modifier( v ) => { v },
            _ => { panic!( "body_part.add_current_health_points. Wrong value for Modifier Health Points" )}
        };

        let old_value = self.get_modifier_health_points();
        let new_value = old_value + current_value;
        self.set_modifier_health_points( new_value );

        self.add_total_health_points( HealthPoints::Total( current_value ));
    }

    pub fn substruct_modifier_health_points( &mut self, value: HealthPoints ){
        let current_value = match value{
            HealthPoints::Modifier( v ) => { v },
            _ => { panic!( "body_part.add_current_health_points. Wrong value for Modifier Health Points" )}
        };
        let old_value = self.get_modifier_health_points();
        let new_value = old_value - current_value;
        self.set_modifier_health_points( new_value );

        self.substruct_total_health_points( HealthPoints::Total( new_value ));
    }

    pub fn get_part_type( &self ) -> PartType{
        return self.part_type;
    }

    pub fn get_part_status( &self ) -> PartStatus{
        return self.part_status;
    }

    pub fn set_current_health_points( &mut self, value: i16 ){
        self.current_health_points = HealthPoints::Current( value );
    }

    pub fn set_total_health_points( &mut self, value: i16 ){
        self.total_health_points = HealthPoints::Total( value );
    }

    pub fn set_modifier_health_points( &mut self, value: i16 ){
        self.modifier_health_points = HealthPoints::Modifier( value );
    }

    pub fn set_part_type( &mut self, part_type: PartType ){
        self.part_type = part_type;
    }

    pub fn set_part_status( &mut self, part_status: PartStatus ){
        self.part_status = part_status;
    }
}