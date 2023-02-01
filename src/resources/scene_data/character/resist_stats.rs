use serde::{Serialize, Deserialize};


#[derive( Serialize, Deserialize, Debug )]
pub enum Resists{
    Kinetic( i8 ),
    Fire( i8 ),
    Electric( i8 ),
    Plasma( i8 ),
    Laser( i8 ),
    Poison( i8 ),
    Knockdown( i8 ),
    Bleed( i8 ),
    Desiase( i8 ),
    Pain( i8 ),
    Fatigue( i8 ),
}

#[derive( Serialize, Deserialize, Debug )]
pub struct ResistStats{
    pub kinetic: Resists,
    pub fire: Resists,
    pub electric: Resists,
    pub plasma: Resists,
    pub laser: Resists,
    pub poison: Resists,
    pub knockdown: Resists,
    pub disease: Resists,
    pub bleed: Resists,
    pub pain: Resists,
    pub fatigue: Resists,
}

impl ResistStats{
    pub fn new() -> Self{
        return ResistStats { 
            kinetic: Resists::Kinetic( 0 ), 
            fire: Resists::Fire( 0 ), 
            electric: Resists::Electric( 0 ), 
            plasma: Resists::Plasma( 0 ), 
            laser: Resists::Laser( 0 ), 
            poison: Resists::Poison( 0 ), 
            knockdown: Resists::Knockdown( 0 ), 
            disease: Resists::Desiase( 0 ), 
            bleed: Resists::Bleed( 0 ), 
            pain: Resists::Pain( 0 ), 
            fatigue: Resists::Fatigue( 0 ), 
        };
    }

    pub fn get_kinetic( &self ) -> i8 {
        match self.kinetic {
            Resists::Kinetic( v ) => { v },
            _ => { panic!( "ResistStats.get_kinetic. Wrong ENUM assign" )},
        }
    }

    pub fn set_kinetic( &mut self, value: i8 ){
        self.kinetic = Resists::Kinetic( value );
    }

    pub fn add_kinetic( &mut self, value: i8 ){
        let old_value = match self.kinetic{
            Resists::Kinetic( v ) => { v },
            _ => { panic!("ResistStats.add_kinetic. Wrong ENUM assign" )},
        };

        let new_value = old_value + value;
        self.kinetic = if new_value > 75 {
            Resists::Kinetic( 75 )
        }else{
            Resists::Kinetic( new_value )
        };        
    }

    pub fn substract_kinetic( &mut self, value: i8 ){
        let old_value = match self.kinetic{
            Resists::Kinetic( v ) => { v },
            _ => { panic!("ResistStats.add_kinetic. Wrong ENUM assign" )},
        };

        let new_value = old_value - value;
        self.kinetic = if new_value < -75 {
            Resists::Kinetic( -75 )
        }else{
            Resists::Kinetic( new_value )
        };
    }


}