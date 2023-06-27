use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Default, Hash)]
pub enum EffectResistType {
    Moveless,
    Bleeding,
    Slow,
    #[default]
    Stun,
    Freeze,
    Burn,
    Electifical,
    Blind,
    Acid,
    Poison,
    Wet,
}


#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Default, Hash)]
pub enum DamageResistType {
    Fire,
    Cold,
    #[default]
    Electric,
    Kinetic,
    Water,
    Acid,
    SacredEnergy,
    DeathEnergy,
}