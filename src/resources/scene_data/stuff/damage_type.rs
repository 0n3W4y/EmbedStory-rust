use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Default, Hash)]
pub enum DamageType {
    Fire,
    Cold,
    Electric,
    Acid,
    Poison,
    Health,
    Stamina,
    #[default]
    Phisical,
    Water,
}