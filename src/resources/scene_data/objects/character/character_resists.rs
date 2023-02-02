use serde::{Serialize, Deserialize};

const MAXIMUM_CHARACTERRESIST_VALUE: i8 = 75;
const MINIMUM_CHARACTERRESIST_VALUE: i8 = -125;

#[derive( Serialize, Deserialize, Debug )]
pub enum CharacterResistType{    
    Poison( i8 ),
    Knockdown( i8 ),
    Bleed( i8 ),
    Disease( i8 ),
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
            disease: Resist{ current: CharacterResistType::Disease( 0 ), modifier: CharacterResistType::Disease( 0 ) }, 
            bleed: Resist{ current: CharacterResistType::Bleed( 0 ), modifier: CharacterResistType::Bleed( 0 ) }, 
            pain: Resist{ current: CharacterResistType::Pain( 0 ), modifier: CharacterResistType::Pain( 0 ) }, 
            fatigue: Resist{ current: CharacterResistType::Fatigue( 0 ), modifier: CharacterResistType::Fatigue( 0 ) },
        };
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
            CharacterResistType::Disease( v ) => {
                let current_modifier_value: i8 = self.get_disease_modifier();
                let new_value = current_modifier_value + v;
                self.set_disease_modifier( new_value );

                if new_value > MAXIMUM_CHARACTERRESIST_VALUE {
                    self.set_disease_current( MAXIMUM_CHARACTERRESIST_VALUE );
                }else{
                    self.set_disease_current( new_value );
                };
            },
            CharacterResistType::Fatigue( v ) => {
                let current_modifier_value: i8 = self.get_fatigue_modifier();
                let new_value = current_modifier_value + v;
                self.set_fatigue_modifier( new_value );

                if new_value > MAXIMUM_CHARACTERRESIST_VALUE {
                    self.set_fatigue_current( MAXIMUM_CHARACTERRESIST_VALUE );
                }else{
                    self.set_fatigue_current( new_value );
                };
            },
            CharacterResistType::Knockdown( v ) => {
                let current_modifier_value: i8 = self.get_knockdown_modifier();
                let new_value = current_modifier_value + v;
                self.set_knockdown_modifier( new_value );

                if new_value > MAXIMUM_CHARACTERRESIST_VALUE {
                    self.set_knockdown_current( MAXIMUM_CHARACTERRESIST_VALUE );
                }else{
                    self.set_knockdown_current( new_value );
                };
            },
            CharacterResistType::Pain( v ) => {
                let current_modifier_value: i8 = self.get_pain_modifier();
                let new_value = current_modifier_value + v;
                self.set_pain_modifier( new_value );

                if new_value > MAXIMUM_CHARACTERRESIST_VALUE {
                    self.set_pain_current( MAXIMUM_CHARACTERRESIST_VALUE );
                }else{
                    self.set_pain_current( new_value );
                };
            },
        };       
    }

    pub fn substract( &mut self, resist: CharacterResistType ){
        match resist {
            CharacterResistType::Poison( v ) => {
                let current_modifier_value: i8 = self.get_poison_modifier();
                let new_value = current_modifier_value - v;
                self.set_poison_modifier( new_value );

                if new_value < MINIMUM_CHARACTERRESIST_VALUE {
                    self.set_poison_current( MINIMUM_CHARACTERRESIST_VALUE );
                }else{
                    self.set_poison_current( new_value );
                };
            },
            CharacterResistType::Bleed( v ) => {
                let current_modifier_value: i8 = self.get_bleed_modifier();
                let new_value = current_modifier_value - v;
                self.set_bleed_modifier( new_value );

                if new_value < MINIMUM_CHARACTERRESIST_VALUE {
                    self.set_bleed_current( MINIMUM_CHARACTERRESIST_VALUE );
                }else{
                    self.set_bleed_current( new_value );
                };
            },
            CharacterResistType::Disease( v ) => {
                let current_modifier_value: i8 = self.get_disease_modifier();
                let new_value = current_modifier_value - v;
                self.set_disease_modifier( new_value );

                if new_value < MINIMUM_CHARACTERRESIST_VALUE {
                    self.set_disease_current( MINIMUM_CHARACTERRESIST_VALUE );
                }else{
                    self.set_disease_current( new_value );
                };
            },
            CharacterResistType::Fatigue( v ) => {
                let current_modifier_value: i8 = self.get_fatigue_modifier();
                let new_value = current_modifier_value - v;
                self.set_fatigue_modifier( new_value );

                if new_value < MINIMUM_CHARACTERRESIST_VALUE {
                    self.set_fatigue_current( MINIMUM_CHARACTERRESIST_VALUE );
                }else{
                    self.set_fatigue_current( new_value );
                };
            },
            CharacterResistType::Knockdown( v ) => {
                let current_modifier_value: i8 = self.get_knockdown_modifier();
                let new_value = current_modifier_value - v;
                self.set_knockdown_modifier( new_value );

                if new_value < MINIMUM_CHARACTERRESIST_VALUE {
                    self.set_knockdown_current( MINIMUM_CHARACTERRESIST_VALUE );
                }else{
                    self.set_knockdown_current( new_value );
                };
            },
            CharacterResistType::Pain( v ) => {
                let current_modifier_value: i8 = self.get_pain_modifier();
                let new_value = current_modifier_value - v;
                self.set_pain_modifier( new_value );

                if new_value < MINIMUM_CHARACTERRESIST_VALUE {
                    self.set_pain_current( MINIMUM_CHARACTERRESIST_VALUE );
                }else{
                    self.set_pain_current( new_value );
                };
            },
        };
    }

    pub fn get_poison_current( &self ) -> i8 {
        match self.poison.current {
            CharacterResistType::Poison( v ) => { v },
            _ => { panic!( "ResistStats.get_poison_current. Wrong ENUM assign" )},
        }
    }

    pub fn get_knockdown_current( &self ) -> i8 {
        match self.knockdown.current {
            CharacterResistType::Knockdown( v ) => { v },
            _ => { panic!( "ResistStats.get_knockdown_current. Wrong ENUM assign" )},
        }
    }

    pub fn get_disease_current( &self ) -> i8 {
        match self.disease.current {
            CharacterResistType::Disease( v ) => { v },
            _ => { panic!( "ResistStats.get_disease_current. Wrong ENUM assign" )},
        }
    }  

    pub fn get_bleed_current( &self ) -> i8 {
        match self.bleed.current {
            CharacterResistType::Bleed( v ) => { v },
            _ => { panic!( "ResistStats.get_bleed_current. Wrong ENUM assign" )},
        }
    }

    pub fn get_pain_current( &self ) -> i8 {
        match self.pain.current {
            CharacterResistType::Pain( v ) => { v },
            _ => { panic!( "ResistStats.get_pain_current. Wrong ENUM assign" )},
        }
    }

    pub fn get_fatigue_current( &self ) -> i8 {
        match self.fatigue.current {
            CharacterResistType::Fatigue( v ) => { v },
            _ => { panic!( "ResistStats.get_fatigue_current. Wrong ENUM assign" )},
        }
    }

    pub fn get_poison_modifier( &self ) -> i8 {
        match self.poison.modifier {
            CharacterResistType::Poison( v ) => { v },
            _ => { panic!( "ResistStats.get_poison_modifier. Wrong ENUM assign" )},
        }
    }

    pub fn get_knockdown_modifier( &self ) -> i8 {
        match self.knockdown.modifier {
            CharacterResistType::Knockdown( v ) => { v },
            _ => { panic!( "ResistStats.get_knockdown_modifier. Wrong ENUM assign" )},
        }
    }

    pub fn get_disease_modifier( &self ) -> i8 {
        match self.disease.modifier {
            CharacterResistType::Disease( v ) => { v },
            _ => { panic!( "ResistStats.get_disease_modifier. Wrong ENUM assign" )},
        }
    }

    pub fn get_bleed_modifier( &self ) -> i8 {
        match self.bleed.modifier {
            CharacterResistType::Bleed( v ) => { v },
            _ => { panic!( "ResistStats.get_bleed_modifier. Wrong ENUM assign" )},
        }
    }

    pub fn get_pain_modifier( &self ) -> i8 {
        match self.pain.modifier {
            CharacterResistType::Pain( v ) => { v },
            _ => { panic!( "ResistStats.get_pain_modifier. Wrong ENUM assign" )},
        }
    }

    pub fn get_fatigue_modifier( &self ) -> i8 {
        match self.fatigue.modifier {
            CharacterResistType::Fatigue( v ) => { v },
            _ => { panic!( "ResistStats.get_fatifue_modifier. Wrong ENUM assign" )},
        }
    }

    pub fn set_poison_current( &mut self, value: i8 ){
        self.poison.current = CharacterResistType::Poison( value );
    }
    pub fn set_knockdown_current( &mut self, value: i8 ){
        self.knockdown.current = CharacterResistType::Knockdown( value );
    }
    pub fn set_disease_current( &mut self, value: i8 ){
        self.disease.current = CharacterResistType::Disease( value );
    }
    pub fn set_bleed_current( &mut self, value: i8 ){
        self.bleed.current = CharacterResistType::Bleed( value );
    }
    pub fn set_pain_current( &mut self, value: i8 ){
        self.pain.current = CharacterResistType::Pain( value );
    }
    pub fn set_fatigue_current( &mut self, value: i8 ){
        self.fatigue.current = CharacterResistType::Fatigue( value );
    }


    pub fn set_poison_modifier( &mut self, value: i8 ){
        self.poison.modifier = CharacterResistType::Poison( value );
    }
    pub fn set_knockdown_modifier( &mut self, value: i8 ){
        self.knockdown.modifier = CharacterResistType::Knockdown( value );
    }
    pub fn set_disease_modifier( &mut self, value: i8 ){
        self.disease.modifier = CharacterResistType::Disease( value );
    }
    pub fn set_bleed_modifier( &mut self, value: i8 ){
        self.bleed.modifier = CharacterResistType::Bleed( value );
    }
    pub fn set_pain_modifier( &mut self, value: i8 ){
        self.pain.modifier = CharacterResistType::Pain( value );
    }
    pub fn set_fatigue_modifier( &mut self, value: i8 ){
        self.fatigue.modifier = CharacterResistType::Fatigue( value );
    }


    

   


}