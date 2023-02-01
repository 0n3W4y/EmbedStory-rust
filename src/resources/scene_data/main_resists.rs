use serde::{Serialize, Deserialize};


#[derive( Eq, PartialEq, Serialize, Deserialize, Debug )]
pub enum MainResistType{
    Kinetic( i8 ),
    Fire( i8 ),
    Electric( i8 ),
    Plasma( i8 ),
    Laser( i8 )
}

#[derive( Serialize, Deserialize, Debug )]
struct MainResists{
    pub kinetic: MainResistType,
    pub fire: MainResistType,
    pub electric: MainResistType,
    pub plasma: MainResistType,
    pub laser: MainResistType,
}

impl MainResists{
    pub fn new() -> Self{
        return MainResists { 
            kinetic: MainResistType::Kinetic( 0 ), 
            fire: MainResistType::Fire( 0 ), 
            electric: MainResistType::Electric( 0 ), 
            plasma: MainResistType::Plasma( 0 ), 
            laser: MainResistType::Laser( 0 )
        }
    }
}