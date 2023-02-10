use serde::{Deserialize, Serialize};

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
