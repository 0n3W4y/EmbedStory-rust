use serde::{Serialize, Deserialize};

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