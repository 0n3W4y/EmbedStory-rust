use serde::{Serialize, Deserialize};

use crate::resources::scene_data::objects::charactor::stats::Stat;

use super::StuffSubtype;

#[derive( PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy)]
pub enum WeaponDamageType {
    Kinetic(i16),
    Fire(i16),
    Plasma(i16),
    Laser(i16),
    Electric(i16),
}

impl Default for WeaponDamageType{
    fn default() -> Self{
        WeaponDamageType::Kinetic(0)
    }
}

#[derive( PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy )]
pub enum StuffAttribute {
    Spread(i16),
    AimTime(i16),
    AttackCooldown(i16),
    Damage(WeaponDamageType),
    AttackMinDistance(i16),
    AttackMaxDistance(i16),
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