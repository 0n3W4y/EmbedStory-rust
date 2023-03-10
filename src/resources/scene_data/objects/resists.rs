use serde::{Deserialize, Serialize};

pub const MAX_RESIST_VALUE: i16 = 75;
pub const MIN_RESIST_VALUE: i16 = -100;

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Resist {
    Kinetic(i16),
    Fire(i16),
    Electric(i16),
    Plasma(i16),
    Laser(i16),
    Poison(i16),
    Knockdown(i16),
    Bleed(i16),
    Disease(i16),
    Pain(i16),
    Fatigue(i16),
}

impl Default for Resist{
    fn default() -> Self {
        Resist::Kinetic(0)
    }
}

impl Resist{
    pub fn get_resist(&self) -> i16 {
        match *self {
            Resist::Bleed(v)
            | Resist::Disease(v)
            | Resist::Electric(v)
            | Resist::Fatigue(v)
            | Resist::Fire(v)
            | Resist::Kinetic(v)
            | Resist::Knockdown(v)
            | Resist::Laser(v)
            | Resist::Pain(v)
            | Resist::Plasma(v)
            | Resist::Poison(v) => v
        }
    }

    pub fn set_resist(&mut self, value: i16) {
        *self = match *self {
            Self::Bleed(_) => Self::Bleed(value),
            Self::Disease(_) => Self::Disease(value),
            Self::Electric(_) => Self::Electric(value),
            Self::Fatigue(_) => Self::Fatigue(value),
            Self::Fire(_) => Self::Fire(value),
            Self::Kinetic(_) => Self::Kinetic(value),
            Self::Knockdown(_) => Self::Knockdown(value),
            Self::Laser(_) => Self::Laser(value),
            Self::Pain(_) => Self::Pain(value),
            Self::Plasma(_) => Self::Plasma(value),
            Self::Poison(_) => Self::Poison(value),
        }
    }
}


