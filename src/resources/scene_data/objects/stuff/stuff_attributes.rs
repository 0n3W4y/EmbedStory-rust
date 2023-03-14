use serde::{Serialize, Deserialize};

use crate::resources::scene_data::objects::charactor::stats::Stat;

use super::StuffSubtype;

#[derive( PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy, Default )]
pub enum WeaponDamageType {
    #[default]
    Kinetic,
    Fire,
    Plasma,
    Laser,
    Electric,
}

#[derive( PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy )]
pub enum StuffAttribute {
    Spread(i16),
    AimTime(i16),
    AttackCooldown(i16),
    Damage(i16),
    DamageType(WeaponDamageType),
    AmmoType(StuffSubtype),
    AmmoCapacity(i16),
    AmmoConsumption(i16),
    ReloadTime(i16),
    Price(i16),
    Resist(Stat),
    Weight(i16),
}

impl Default for StuffAttribute {
    fn default() -> Self{
        StuffAttribute::Price(0)
    }
}