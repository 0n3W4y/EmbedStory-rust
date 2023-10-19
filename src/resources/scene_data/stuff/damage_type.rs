use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Default, Hash)]
pub enum DamageType {
    Fire,
    Cold,
    Electric,
    Cutting,
    Piercing,
    Crushing,
    Water,
    Acid,
    Poison,
    Health,
    Stamina,
    #[default]
    Melee,
    Ranged,
    Magic,
}