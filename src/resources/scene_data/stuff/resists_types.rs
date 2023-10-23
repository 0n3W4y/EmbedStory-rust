use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Default, Hash)]
pub enum ResistType {
    FireDamage,
    ColdDamage,
    ElectricDamage,
    AcidDamage,
    PoisonDamage,
    HealthDamage,
    StaminaDamage,
    #[default]
    PhisicalDamage,
    StunEffect,
    AcidEffect,
    MovelessEffect,
    SlowEffect,
    BleedingEffect,
    BurnEffect,
    ElectrificationEffect,
    FreezeEffect,
    BlindEffect,
    PoisonEffect,
    WetEffect,
    BrokenArmorEffect,
    BrokenWeaponEffect,
    IncreaseMovementEffect,
    FrostbiteEffect,
}