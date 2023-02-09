use serde::{Serialize, Deserialize};

const MAXIMUM_CHARACTERRESIST_VALUE: i8 = 75;
const MINIMUM_CHARACTERRESIST_VALUE: i8 = -125;


#[derive( Eq, PartialEq, Serialize, Deserialize, Debug, Clone )]
pub enum MainResistType{
    Kinetic( i8 ),
    Fire( i8 ),
    Electric( i8 ),
    Plasma( i8 ),
    Laser( i8 )
}

#[derive( Serialize, Deserialize, Debug, Clone )]
pub struct Resist{
    pub current: MainResistType,
    pub modifier: MainResistType,
}

#[derive( Serialize, Deserialize, Debug, Clone )]
pub struct MainResists{
    pub kinetic: Resist,
    pub fire: Resist,
    pub electric: Resist,
    pub plasma: Resist,
    pub laser: Resist,
}

impl MainResists{
    pub fn new() -> Self{
        return MainResists { 
            kinetic: Resist { current: MainResistType::Kinetic( 0 ), modifier: MainResistType::Kinetic( 0 ) },
            fire: Resist { current: MainResistType::Fire( 0 ), modifier: MainResistType::Fire( 0 ) }, 
            electric: Resist { current: MainResistType::Electric( 0 ), modifier: MainResistType::Electric( 0 ) },
            plasma: Resist { current: MainResistType::Plasma( 0 ), modifier: MainResistType::Plasma( 0 ) },
            laser: Resist { current: MainResistType::Laser( 0 ), modifier: MainResistType::Laser( 0 ) }
        }
    }

    //by default we add and substruct MODIFIER value;
    pub fn add( &mut self, resist: MainResistType ){
        match resist {
            MainResistType::Kinetic( v ) => {
                let old_value = self.get_kinetic_modifier();
                let new_value = old_value + v;
                self.set_kinetic_modifier( new_value );

                if new_value > MAXIMUM_CHARACTERRESIST_VALUE {
                    self.set_kinetic_current( MAXIMUM_CHARACTERRESIST_VALUE );
                }else{
                    self.set_kinetic_current( new_value );
                }
            },
            MainResistType::Electric( v ) => {
                let old_value = self.get_electric_modifier();
                let new_value = old_value + v;
                self.set_electric_modifier( new_value );

                if new_value > MAXIMUM_CHARACTERRESIST_VALUE {
                    self.set_electric_current( MAXIMUM_CHARACTERRESIST_VALUE );
                }else{
                    self.set_electric_current( new_value );
                }
            },
            MainResistType::Fire( v ) => {
                let old_value = self.get_fire_modifier();
                let new_value = old_value + v;
                self.set_fire_modifier( new_value );

                if new_value > MAXIMUM_CHARACTERRESIST_VALUE {
                    self.set_fire_current( MAXIMUM_CHARACTERRESIST_VALUE );
                }else{
                    self.set_fire_current( new_value );
                }
            },
            MainResistType::Plasma( v ) => {
                let old_value = self.get_plasma_modifier();
                let new_value = old_value + v;
                self.set_plasma_modifier( new_value );

                if new_value > MAXIMUM_CHARACTERRESIST_VALUE {
                    self.set_plasma_current( MAXIMUM_CHARACTERRESIST_VALUE );
                }else{
                    self.set_plasma_current( new_value );
                }
            },
            MainResistType::Laser( v ) => {
                let old_value = self.get_laser_modifier();
                let new_value = old_value + v;
                self.set_laser_modifier( new_value );

                if new_value > MAXIMUM_CHARACTERRESIST_VALUE {
                    self.set_laser_current( MAXIMUM_CHARACTERRESIST_VALUE );
                }else{
                    self.set_laser_current( new_value );
                }
            },
        }
    }

    pub fn substruct( &mut self, resist: MainResistType ){
        match resist {
            MainResistType::Kinetic( v ) => {
                let old_value = self.get_kinetic_modifier();
                let new_value = old_value - v;
                self.set_kinetic_modifier( new_value );

                if new_value < MINIMUM_CHARACTERRESIST_VALUE {
                    self.set_kinetic_current( MINIMUM_CHARACTERRESIST_VALUE );
                }else{
                    self.set_kinetic_current( new_value );
                }
            },
            MainResistType::Electric( v ) => {
                let old_value = self.get_electric_modifier();
                let new_value = old_value - v;
                self.set_electric_modifier( new_value );

                if new_value < MINIMUM_CHARACTERRESIST_VALUE {
                    self.set_electric_current( MINIMUM_CHARACTERRESIST_VALUE );
                }else{
                    self.set_electric_current( new_value );
                }
            },
            MainResistType::Fire( v ) => {
                let old_value = self.get_fire_modifier();
                let new_value = old_value - v;
                self.set_fire_modifier( new_value );

                if new_value < MINIMUM_CHARACTERRESIST_VALUE {
                    self.set_fire_current( MINIMUM_CHARACTERRESIST_VALUE );
                }else{
                    self.set_fire_current( new_value );
                }
            },
            MainResistType::Plasma( v ) => {
                let old_value = self.get_plasma_modifier();
                let new_value = old_value - v;
                self.set_plasma_modifier( new_value );

                if new_value < MINIMUM_CHARACTERRESIST_VALUE {
                    self.set_plasma_current( MINIMUM_CHARACTERRESIST_VALUE );
                }else{
                    self.set_plasma_current( new_value );
                }
            },
            MainResistType::Laser( v ) => {
                let old_value = self.get_laser_modifier();
                let new_value = old_value - v;
                self.set_laser_modifier( new_value );

                if new_value < MINIMUM_CHARACTERRESIST_VALUE {
                    self.set_laser_current( MINIMUM_CHARACTERRESIST_VALUE );
                }else{
                    self.set_laser_current( new_value );
                }
            },
        }
    }

    pub fn get_kinetic_current( &self ) -> i8 {
        match self.kinetic.current {
            MainResistType::Kinetic( v ) => { return v; },
            _ => { panic!( "main_resists.get_kinetic_current. Wrong ENUM assigned" )}
        }
    }

    pub fn get_fire_current( &self ) -> i8 {
        match self.fire.current {
            MainResistType::Fire( v ) => { return v;},
            _ => { panic!( "main_resists.get_fire_current. Wrong ENUM assigned" )}
        }
    }

    pub fn get_electirc_current( &self ) -> i8 {
        match self.electric.current {
            MainResistType::Electric( v ) => { return v;},
            _ => { panic!( "main_resists.get_electric_current. Wrong ENUM assigned" )}
        }
    }

    pub fn get_plasma_current( &self ) -> i8 {
        match self.plasma.current {
            MainResistType::Plasma( v ) => { return v;},
            _ => { panic!( "main_resists.get_plasma_current. Wrong ENUM assigned" )}
        }
    }

    pub fn get_laser_current( &self ) -> i8 {
        match self.laser.current {
            MainResistType::Laser( v ) => { return v;},
            _ => { panic!( "main_resists.get_laser_current. Wrong ENUM assigned" )}
        }
    }

    pub fn get_kinetic_modifier( &self ) -> i8 {
        match self.kinetic.modifier {
            MainResistType::Kinetic( v ) => { return v; },
            _ => { panic!( "main_resists.get_kinetic_modifier. Wrong ENUM assigned" )}
        }
    }

    pub fn get_fire_modifier( &self ) -> i8 {
        match self.fire.modifier {
            MainResistType::Fire( v ) => { return v;},
            _ => { panic!( "main_resists.get_fire_modifier. Wrong ENUM assigned" )}
        }
    }

    pub fn get_electric_modifier( &self ) -> i8 {
        match self.electric.modifier {
            MainResistType::Electric( v ) => { return v;},
            _ => { panic!( "main_resists.get_electric_modifier. Wrong ENUM assigned" )}
        }
    }

    pub fn get_plasma_modifier( &self ) -> i8 {
        match self.plasma.modifier {
            MainResistType::Plasma( v ) => { return v;},
            _ => { panic!( "main_resists.get_plasma_modifier. Wrong ENUM assigned" )}
        }
    }

    pub fn get_laser_modifier( &self ) -> i8 {
        match self.laser.modifier {
            MainResistType::Laser( v ) => { return v;},
            _ => { panic!( "main_resists.get_laser_modifier. Wrong ENUM assigned" )}
        }
    }

    pub fn set_kinetic_current( &mut self, value: i8 ){
        self.kinetic.current = MainResistType::Kinetic( value );
    }
    pub fn set_fire_current( &mut self, value: i8 ){
        self.fire.current = MainResistType::Fire( value );
    }
    pub fn set_electric_current( &mut self, value: i8 ){
        self.electric.current = MainResistType::Electric( value );
    }
    pub fn set_plasma_current( &mut self, value: i8 ){
        self.plasma.current = MainResistType::Plasma( value );
    }
    pub fn set_laser_current( &mut self, value: i8 ){
        self.laser.current = MainResistType::Laser( value );
    }

    pub fn set_kinetic_modifier( &mut self, value: i8 ){
        self.kinetic.modifier = MainResistType::Kinetic( value );
    }
    pub fn set_fire_modifier( &mut self, value: i8 ){
        self.fire.modifier = MainResistType::Fire( value );
    }
    pub fn set_electric_modifier( &mut self, value: i8 ){
        self.electric.modifier = MainResistType::Electric( value );
    }
    pub fn set_plasma_modifier( &mut self, value: i8 ){
        self.plasma.modifier = MainResistType::Plasma( value );
    }
    pub fn set_laser_modifier( &mut self, value: i8 ){
        self.laser.modifier = MainResistType::Laser( value );
    }
}