use serde::{ Serialize, Deserialize };

pub mod character_resists;

#[derive( Serialize, Deserialize, Debug, Clone )]
pub struct Character;
