use serde::{ Serialize, Deserialize };

pub mod character_resists;


#[derive( PartialEq, Eq, Clone, Serialize, Deserialize, Debug )]
pub enum CharacterType{
    
}

#[derive( Serialize, Deserialize, Debug, Clone )]
pub struct Character;
