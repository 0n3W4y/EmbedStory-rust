use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone, Copy, Hash, Default)]
pub enum Resist {
    Kinetic,
    Fire,
    Electric,
    Plasma,
    Laser,
    Poison,
    Knockdown,
    Bleed,
    Disease,
    #[default]
    Pain,
    Fatigue,
}


