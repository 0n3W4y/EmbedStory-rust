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
        match *self {
            Resist::Bleed(mut v)
            | Resist::Disease(mut v)
            | Resist::Electric(mut v)
            | Resist::Fatigue(mut v)
            | Resist::Fire(mut v)
            | Resist::Kinetic( mut v)
            | Resist::Knockdown(mut v)
            | Resist::Laser(mut v)
            | Resist::Pain(mut v)
            | Resist::Plasma(mut v)
            | Resist::Poison(mut v) => v = value
        }
    }
}


