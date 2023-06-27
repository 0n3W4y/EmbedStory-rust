use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Default, Hash)]
pub enum Stat{
    #[default]
    Strength,
    Dexterity,
    Mobility,
    Wisdom,
    Intellect,
    Luck,
    Vitality,
    Endurance,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Default, Hash)]
pub enum ExtraStat{
    #[default]
    HealthPoints,
    StaminaPoints,
}
