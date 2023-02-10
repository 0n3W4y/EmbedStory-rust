use serde::{ Serialize, Deserialize };

#[derive( PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy )]
pub enum StuffType {
    
}

#[derive( Serialize, Deserialize, Debug, Clone )]
pub struct Stuff;