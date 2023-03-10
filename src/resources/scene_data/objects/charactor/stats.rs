use serde::{Serialize, Deserialize};

pub const MIN_STAT_VALUE: i16 = 1;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq )]
pub enum Stat{
    Strength(i16),
    Intellect(i16),
    Endurance(i16),
    Vitality(i16),    
    Agility(i16),    
    Mobility(i16),
    HealthPoints(i16),
}

impl Stat {
    pub fn get_stat(&self) -> i16 {
        match *self{
            Self::Strength(v)
            | Self::Endurance(v)
            | Self::Intellect(v)
            | Self::Agility(v)
            | Self::Vitality(v)
            | Self::Mobility(v)
            | Self::HealthPoints(v) => v
        }
    }

    pub fn set_stat(&mut self, value: i16) {
        *self = match self {
            Self::Strength(_) => Self::Strength(value),
            Self::Endurance(_) => Self::Endurance(value),
            Self::Intellect(_) => Self::Intellect(value),
            Self::Agility(_) => Self::Agility(value),
            Self::HealthPoints(_) => Self::HealthPoints(value),
            Self::Mobility(_) => Self::Mobility(value),
            Self::Vitality(_) => Self::Vitality(value),

        }
    }
}

impl Default for Stat{
    fn default() -> Self{
        Stat::HealthPoints(0)
    }
}