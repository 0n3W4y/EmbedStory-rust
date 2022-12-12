use serde::{ Serialize, Deserialize };

pub mod body_part;
pub mod health_stats;
pub mod resist_stats;

#[derive( Serialize, Deserialize )]
pub struct Character;
