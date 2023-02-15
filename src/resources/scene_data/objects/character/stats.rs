use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy )]
pub enum Stat{
    HealthPoints( i16 ),
}

impl Default for Stat{
    fn default() -> Self{
        Stat::HealthPoints(0)
    }
}