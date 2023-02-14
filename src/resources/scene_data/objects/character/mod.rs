use serde::{ Serialize, Deserialize };

pub mod stats;

#[derive( PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy )]
pub enum CharacterType{
    
}

#[derive( Serialize, Deserialize, Debug, Clone )]
pub struct Character;
