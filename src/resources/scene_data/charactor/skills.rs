use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum ActiveSkillType {
    #[default]
    BasicWeaponAttack,
    BasicTrinketAttack,
}


#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum PassiveSkillType {
    LifeLich,
    StaminaLich,
    #[default]
    Stun,
    Slow,
    Moveless,
    Bleed,
    Burn,
    Electrification,
    Freeze,
    Blind,
    Poison,
    Acid,
    Wet,
}

pub struct ActiveSkill;
pub struct PassiveSkill;