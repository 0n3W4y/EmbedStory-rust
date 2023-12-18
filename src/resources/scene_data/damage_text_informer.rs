use bevy::prelude::*;
use serde::Deserialize;

use crate::{resources::scene_data::stuff::damage_type::DamageType, components::DamageTextComponent};


const WHITE_DAMAGE_TEXT: Color = Color::Rgba{ red:( 255.0 / 255.0 ), green:( 255.0 / 255.0 ) , blue:( 255.0 / 255.0 ) , alpha: 1.0 };
//const WHITERED_DAMAGE_TEXT: Color = Color::Rgba{ red:( 255.0 / 255.0 ), green:( 255.0 / 255.0 ) , blue:( 255.0 / 255.0 ) , alpha: 1.0 };
const RED_DAMAGE_TEXT: Color = Color::Rgba{ red:( 255.0 / 255.0 ), green:( 255.0 / 255.0 ) , blue:( 255.0 / 255.0 ) , alpha: 1.0 };
const GREEN_DAMAGE_TEXT: Color = Color::Rgba{ red:( 150.0 / 255.0 ), green:( 75.0 / 255.0 ), blue:( 45.0 / 255.0 ), alpha: 1.0 };
const BLUE_DAMAGE_TEXT: Color = Color::Rgba{ red:( 100.0 / 255.0 ), green:( 50.0 / 255.0 ) , blue:( 20.0 / 255.0 ) , alpha: 1.0 };
const GRAY_DAMAGE_TEXT: Color = Color::Rgba { red: ( 100.0 / 255.0 ), green: ( 100.0 / 255.0 ), blue: ( 100.0 / 255.0 ), alpha: 1.0 };
const ORANGE_DAMAGE_TEXT: Color = Color::Rgba { red: ( 100.0 / 255.0 ), green: ( 100.0 / 255.0 ), blue: ( 100.0 / 255.0 ), alpha: 1.0 };
const CYAN_DAMAGE_TEXT: Color = Color::Rgba { red: ( 100.0 / 255.0 ), green: ( 100.0 / 255.0 ), blue: ( 100.0 / 255.0 ), alpha: 1.0 };
const PURPULE_DAMAGE_TEXT: Color = Color::Rgba { red: ( 100.0 / 255.0 ), green: ( 100.0 / 255.0 ), blue: ( 100.0 / 255.0 ), alpha: 1.0 };
const LIGHTGREEN_DAMAGE_TEXT: Color = Color::Rgba { red: ( 100.0 / 255.0 ), green: ( 100.0 / 255.0 ), blue: ( 100.0 / 255.0 ), alpha: 1.0 };


#[derive(Deserialize, Debug)]
pub struct DamageTextInformer {
    pub text: String,
    pub bold: bool,
    pub color: Color,
} 

impl DamageTextInformer {
    pub fn new (damage_value: i16, string: Option<String>, bold: bool, damage_type: Option<&DamageType>) -> Self {
        let text = match string {
            Some(v) => v,
            None => {
                let mut new_text = if damage_value < 0 {
                    "+".to_string()
                } else {
                    "-".to_string()
                };
                let damage_text = damage_value.to_string();
                new_text.push_str(&damage_text);
                new_text
            }
        };
        

        let new_color: Color = match damage_type {
            Some(v) => { match *v {
                DamageType::Fire => ORANGE_DAMAGE_TEXT,
                DamageType::Cold => BLUE_DAMAGE_TEXT,
                DamageType::Electric => PURPULE_DAMAGE_TEXT,
                DamageType::Phisical => WHITE_DAMAGE_TEXT,
                DamageType::Water => CYAN_DAMAGE_TEXT,
                DamageType::Acid => LIGHTGREEN_DAMAGE_TEXT,
                DamageType::Poison => GREEN_DAMAGE_TEXT,
                DamageType::Health => RED_DAMAGE_TEXT,
                DamageType::Stamina => GRAY_DAMAGE_TEXT,
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

pub fn update_damage_text_informer(
    objects_query: Query<&DamageTextComponent>
){
    
}