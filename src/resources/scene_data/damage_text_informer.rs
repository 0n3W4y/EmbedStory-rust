use bevy::prelude::*;
use serde::Deserialize;

use super::Damage;


const WHITE_DAMAGE_TEXT: Color = Color::Rgba{ red:( 255.0 / 255.0 ), green:( 255.0 / 255.0 ) , blue:( 255.0 / 255.0 ) , alpha: 1.0 };
//const WHITERED_DAMAGE_TEXT: Color = Color::Rgba{ red:( 255.0 / 255.0 ), green:( 255.0 / 255.0 ) , blue:( 255.0 / 255.0 ) , alpha: 1.0 };
const RED_DAMAGE_TEXT: Color = Color::Rgba{ red:( 255.0 / 255.0 ), green:( 255.0 / 255.0 ) , blue:( 255.0 / 255.0 ) , alpha: 1.0 };
const DARK_GREEN_DAMAGE_TEXT: Color = Color::Rgba{ red:( 50.0 / 255.0 ), green:( 250.0 / 255.0 ), blue:( 50.0 / 255.0 ), alpha: 1.0 };
const BLUE_DAMAGE_TEXT: Color = Color::Rgba{ red:( 100.0 / 255.0 ), green:( 50.0 / 255.0 ) , blue:( 20.0 / 255.0 ) , alpha: 1.0 };
const GRAY_DAMAGE_TEXT: Color = Color::Rgba { red: ( 100.0 / 255.0 ), green: ( 100.0 / 255.0 ), blue: ( 100.0 / 255.0 ), alpha: 1.0 };
const ORANGE_DAMAGE_TEXT: Color = Color::Rgba { red: ( 100.0 / 255.0 ), green: ( 100.0 / 255.0 ), blue: ( 100.0 / 255.0 ), alpha: 1.0 };
const CYAN_DAMAGE_TEXT: Color = Color::Rgba { red: ( 100.0 / 255.0 ), green: ( 100.0 / 255.0 ), blue: ( 100.0 / 255.0 ), alpha: 1.0 };
const PURPULE_DAMAGE_TEXT: Color = Color::Rgba { red: ( 100.0 / 255.0 ), green: ( 100.0 / 255.0 ), blue: ( 100.0 / 255.0 ), alpha: 1.0 };
const LIGHTGREEN_DAMAGE_TEXT: Color = Color::Rgba { red: ( 100.0 / 255.0 ), green: ( 100.0 / 255.0 ), blue: ( 100.0 / 255.0 ), alpha: 1.0 };

#[derive(Clone, Deserialize, Debug)]
pub enum DamageIgnored {
    Evaded,
    Missed,
}

#[derive(Deserialize, Debug)]
pub struct DamageTextInformer {
    pub text: String,
    pub bold: bool,
    pub color: Color,
} 

impl DamageTextInformer {
    pub fn new (text: String, bold: bool, damage_type: Option<&Damage>) -> Self {     
        let new_color: Color = match damage_type {
            Some(v) => { match *v {
                Damage::Fire => ORANGE_DAMAGE_TEXT,
                Damage::Cold => BLUE_DAMAGE_TEXT,
                Damage::Electric => PURPULE_DAMAGE_TEXT,
                Damage::Phisical => WHITE_DAMAGE_TEXT,
                Damage::Water => CYAN_DAMAGE_TEXT,
                Damage::Acid => LIGHTGREEN_DAMAGE_TEXT,
                Damage::Poison => DARK_GREEN_DAMAGE_TEXT,
                Damage::Health => RED_DAMAGE_TEXT,
                Damage::Stamina => GRAY_DAMAGE_TEXT,
            }

            },
            None => GRAY_DAMAGE_TEXT,
        };

        DamageTextInformer {
            text,
            bold,
            color: new_color,
        }
    }
}

pub fn update_damage_text_informer(){

}