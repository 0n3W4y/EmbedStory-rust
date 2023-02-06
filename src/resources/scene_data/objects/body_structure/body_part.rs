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
    LeftEye,
    RightEye,
    Mouth,
    Nose,
    LeftArm,
    LeftWrist,
    RightArm,
    RightWrist,
    LeftFoot,
    LeftSole,
    RightFoot,
    RightSole,
    Torso,
    Brain,
    LeftLung,
    RightLung,
    Heart,
    Groin
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
    Scratched,
    Damaged,
    Broken,
    Disrupted,
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
        let part_status = self.get_part_status();
        let max_health_points_percent = self.get_percent_for_part_status( part_status );
        let max_health_points: i16 = total_value * max_health_points_percent as i16 / 100;

        let new_value: i16 = old_value + current_value;
        //check for maximum health with part status and total healh points;
        if new_value < total_value && new_value <= max_health_points {
            self.set_current_health_points( new_value ); // value lower  total health and max health with part status;
        }else if new_value < total_value && new_value > max_health_points {
            self.set_current_health_points( max_health_points ); // value bigger max health, but still lower total healh points;
        }else{
            self.set_current_health_points( total_value );
        }
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

    pub fn get_part_type( &self ) -> &PartType{
        return &self.part_type;
    }

    pub fn get_part_status( &self ) -> &PartStatus{
        return &self.part_status;
    }

    pub fn check_part_status( &self ) -> PartStatus {
        let current_hp = self.get_current_health_points();
        let total_hp = self.get_total_health_points();
        let percent = ( current_hp * 100 / total_hp ) as i8;
        let part_status = self.get_part_status_from_percent( percent );
        return part_status;        
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

    fn add_total_health_points( &mut self, value: HealthPoints ){
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
    
    fn substruct_total_health_points( &mut self, value: HealthPoints ){
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

    fn get_part_status_from_percent( &self, percent: i8 ) -> PartStatus {
        match self.part_type {
            PartType::Natural => {
                let result = if percent > 90 { PartStatus::Healthy }
                else if percent <= 90 && percent > 60 { PartStatus::Scratched }
                else if percent <= 60 && percent > 30 { PartStatus::Damaged }
                else if percent <= 30 && percent > 0 { PartStatus::Broken }
                else { PartStatus::Disrupted };
                return result;
            },
            PartType::Wood => {
                let result = if percent > 90 { PartStatus::Healthy }
                else if percent <= 90 && percent > 80 { PartStatus::Scratched }
                else if percent <= 80 && percent > 50 { PartStatus::Damaged }
                else if percent <= 50 && percent > 0 { PartStatus::Broken }
                else { PartStatus::Disrupted };
                return result;
            },
            PartType::Mechanical => {
                let result = if percent > 90 { PartStatus::Healthy }
                else if percent <= 90 && percent > 50 { PartStatus::Scratched }
                else if percent <= 50 && percent > 20 { PartStatus::Damaged }
                else if percent <= 20 && percent > 0 { PartStatus::Broken }
                else { PartStatus::Disrupted };
                return result;
            },
            PartType::Cybernetic => {
                let result = if percent > 90 { PartStatus::Healthy }
                else if percent <= 90 && percent > 30 { PartStatus::Scratched }
                else if percent <= 30 && percent > 10 { PartStatus::Damaged }
                else if percent <= 10 && percent > 0 { PartStatus::Broken }
                else { PartStatus::Disrupted };
                return result;
            }
        }
    }

    fn get_percent_for_part_status( & self, part_status: &PartStatus ) -> i8 {
        match self.part_type {
            PartType::Natural => {
                let result = match part_status {
                    PartStatus::Disrupted => { 0 },
                    PartStatus::Broken => { 30 },
                    PartStatus::Damaged => { 60 },
                    PartStatus::Scratched => { 90 },
                    PartStatus::Healthy => { 100 },
                };
                return result;
            },
            PartType::Wood => {
                let result = match part_status {
                    PartStatus::Disrupted => { 0 },
                    PartStatus::Broken => { 50 },
                    PartStatus::Damaged => { 80 },
                    PartStatus::Scratched => { 90 },
                    PartStatus::Healthy => { 100 },
                };
                return result;
            },
            PartType::Mechanical => {
                let result = match part_status {
                    PartStatus::Disrupted => { 0 },
                    PartStatus::Broken => { 20 },
                    PartStatus::Damaged => { 50 },
                    PartStatus::Scratched => { 90 },
                    PartStatus::Healthy => { 100 },
                };
                return result;
            },
            PartType::Cybernetic => {
                let result = match part_status {
                    PartStatus::Disrupted => { 0 },
                    PartStatus::Broken => { 10 },
                    PartStatus::Damaged => { 30 },
                    PartStatus::Scratched => { 90 },
                    PartStatus::Healthy => { 100 },
                };
                return result;
            }
        }
    }


    
}