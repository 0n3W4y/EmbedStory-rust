use serde::{Serialize, Deserialize};

const MAXIMUM_CHARACTERRESIST_VALUE: i8 = 75;
const MINIMUM_CHARACTERRESIST_VALUE: i8 = -125;

#[derive( Serialize, Deserialize, Debug )]
pub enum CharacterResistType{    
    Poison( i8 ),
    Knockdown( i8 ),
    Bleed( i8 ),
    Desiase( i8 ),
    Pain( i8 ),
    Fatigue( i8 ),
}

#[derive( Serialize, Deserialize, Debug )]
struct Resist{
    current: CharacterResistType,
    modifier: CharacterResistType,
}

#[derive( Serialize, Deserialize, Debug )]
struct CharacterResists{
    pub poison: Resist,
    pub knockdown: Resist,
    pub disease: Resist,
    pub bleed: Resist,
    pub pain: Resist,
    pub fatigue: Resist,
}

impl CharacterResists{
    pub fn new() -> Self{
        return CharacterResists {
            poison: Resist{ current: CharacterResistType::Poison( 0 ), modifier: CharacterResistType::Poison( 0 ) },
            knockdown: Resist{ current: CharacterResistType::Knockdown( 0 ), modifier: CharacterResistType::Knockdown( 0 ) },
            disease: Resist{ current: CharacterResistType::Desiase( 0 ), modifier: CharacterResistType::Desiase( 0 ) }, 
            bleed: Resist{ current: CharacterResistType::Bleed( 0 ), modifier: CharacterResistType::Bleed( 0 ) }, 
            pain: Resist{ current: CharacterResistType::Pain( 0 ), modifier: CharacterResistType::Pain( 0 ) }, 
            fatigue: Resist{ current: CharacterResistType::Fatigue( 0 ), modifier: CharacterResistType::Fatigue( 0 ) },
        };
    }

    pub fn get_poison_current( &self ) -> i8 {
        match self.poison.current {
            CharacterResistType::Poison( v ) => { v },
            _ => { panic!( "ResistStats.get_poison_current. Wrong ENUM assign" )},
        }
    }

    pub fn get_poison_modifier( &self ) -> i8 {
        match self.poison.modifier {
            CharacterResistType::Poison( v ) => { v },
            _ => { panic!( "ResistStats.get_current. Wrong ENUM assign" )},
        }
    }

    pub fn set_poison_current( &mut self, value: i8 ){
        self.poison.current = CharacterResistType::Poison( value );
    }

    pub fn set_poison_modifier( &mut self, value: i8 ){
        self.poison.modifier = CharacterResistType::Poison( value );
    }

    //by default add only modifier values;
    pub fn add( &mut self, resist: CharacterResistType ){
        match resist {
            CharacterResistType::Poison( v ) => {
                let current_modifier_value: i8 = self.get_poison_modifier();
                let new_value = current_modifier_value + v;
                self.set_poison_modifier( new_value );

                if new_value > MAXIMUM_CHARACTERRESIST_VALUE {
                    self.set_poison_current( MAXIMUM_CHARACTERRESIST_VALUE );
                }else{
                    self.set_poison_current( new_value );
                };
            },
            CharacterResistType::Bleed( v ) => {
                let current_modifier_value: i8 = self.get_bleed_modifier();
                let new_value = current_modifier_value + v;
                self.set_bleed_modifier( new_value );

                if new_value > MAXIMUM_CHARACTERRESIST_VALUE {
                    self.set_bleed_current( MAXIMUM_CHARACTERRESIST_VALUE );
                }else{
                    self.set_bleed_current( new_value );
                };
            },
            CharacterResistType::Desiase( v ) => {},
            CharacterResistType::Fatigue( v ) => {},
            CharacterResistType::Knockdown( v ) => {},
            CharacterResistType::Pain( v ) => {},
        };       
    }

    pub fn substract( &mut self, resist: CharacterResistType ){
        
    }


}