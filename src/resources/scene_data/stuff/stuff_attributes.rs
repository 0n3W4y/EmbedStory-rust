use serde::{Serialize, Deserialize};

#[derive( PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy, Default)]
pub enum WeaponDamageType {
    #[default]
    Kinetic,
    Fire,
    Plasma,
    Laser,
    Electric,
}

#[derive( PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy, Default)]
pub enum StuffAttribute {
    Spread,
    AimTime,
    AttackCooldown,
    AttackMinDistance,
    AttackMaxDistance,
    AmmoCapacity,
    AmmoConsumption,
    ReloadTime,
    #[default]
    Price,
    Weight,
}
