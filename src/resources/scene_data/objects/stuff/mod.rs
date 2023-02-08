use serde::{ Serialize, Deserialize };

#[derive( PartialEq, Eq, Clone, Serialize, Deserialize, Debug )]
pub enum StuffType {
    
}

#[derive( Serialize, Deserialize, Debug, Clone )]
pub struct Stuff;