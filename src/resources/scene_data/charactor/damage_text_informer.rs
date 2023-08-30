use bevy::prelude::*;
use serde::Deserialize;


const RED_DAMAGE_TEXT: Color = Color::Rgba{ red:( 10.0 / 255.0 ), green:( 200.0 / 255.0 ) , blue:( 70.0 / 255.0 ) , alpha: 1.0 };
const GREEN_DAMAGE_TEXT: Color = Color::Rgba{ red:( 150.0 / 255.0 ), green:( 75.0 / 255.0 ), blue:( 45.0 / 255.0 ), alpha: 1.0 };
const BLUE_DAMAGE_TEXT: Color = Color::Rgba{ red:( 100.0 / 255.0 ), green:( 50.0 / 255.0 ) , blue:( 20.0 / 255.0 ) , alpha: 1.0 };

pub enum DamageColorType {
    Red,
    Green,
    Blue,
    Purpule,
    Yellow,
    Orange,
    Gray,
    White,
}

#[derive(Deserialize, Debug)]
pub struct DamageTextInformer {
    pub text: String,
    pub bold: bool,
    pub color: Color,
} 

impl DamageTextInformer {
    pub fn new( text: String, bold: bool, color: DamageColorType) -> Self {
        let new_color: Color = match color {
            DamageColorType::Red => RED_DAMAGE_TEXT,
            DamageColorType::Green => GREEN_DAMAGE_TEXT,
            DamageColorType::Blue => BLUE_DAMAGE_TEXT,
            DamageColorType::Purpule => todo!(),
            DamageColorType::Yellow => todo!(),
            DamageColorType::Orange => todo!(),
            DamageColorType::Gray => todo!(),
            DamageColorType::White => todo!(),
        };

        DamageTextInformer {
            text,
            bold,
            color: new_color,
        }
    }
}