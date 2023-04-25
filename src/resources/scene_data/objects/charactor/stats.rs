use serde::{Serialize, Deserialize};

pub const MIN_STAT_VALUE: i16 = 1;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Default, Hash)]
pub enum Stat{
    Strength,
    Intellect,
    Endurance,
    Vitality,    
    Agility,    
    Mobility,
    #[default]
    HealthPoints,
}