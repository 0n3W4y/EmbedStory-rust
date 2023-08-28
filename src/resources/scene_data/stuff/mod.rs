use std::collections::HashMap;

use serde::{ Serialize, Deserialize };

pub mod stuff_attributes;
pub mod damage_type;

use crate::scenes::game_scenes::tilemap::tile::Position;

use crate::resources::scene_data::charactor::StuffWearSlot;

use self::damage_type::DamageType;

use super::charactor::abilities::AbilityType;
use super::charactor::skills::SkillType;
use super::charactor::{stats::{Stat, ExtraStat}, effects::EffectType};

#[derive( PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy, Hash )]
pub enum StuffType{
    MeleeWeapon,
    RangedWeapon,
    ThrowingWeapon,
    TheftKit,
    Item,
    HeadArmor,
    TorsoArmor,
    PantsArmor,
    ShoesArmor,
    GlovesArmor,
    Ammo,
    HealthPackKit,
    Food,
}

#[derive( PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy, Hash )]
pub enum StuffSubtype {
    Bandage, // stop bleeding
    HealthPack, // add health to part
    DoctorsBag, // vs trauma
    Painkillers, // vs pain
    Antibiotic, //vs disease
    Vitamins, // vs fatigue
    Picklock,
    // разные баффы, наркотики
    // апгрейды для оружия
    // апгрейды для брони
    // собственно сама броня
    // разные оружия
}


#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Stuff{
    pub id: usize,
    pub stuff_type: StuffType,
    pub stuff_subtype: StuffSubtype,

    pub max_amount: usize,
    pub amount: usize,

    pub position: Position<i32>,
    pub wear_slot: StuffWearSlot,

    pub base_damage: HashMap<DamageType, i16>,
    pub base_critical_hit_chanse: i16,
    pub base_critical_multiplier: i16,
    pub base_cooldown: i16,

    pub effects: HashMap<EffectType, i16>,
    pub passive_skills: HashMap<SkillType, i16>,

    pub stats: HashMap<Stat, i16>,
    pub extra_stats: HashMap<ExtraStat, i16>,
    pub effect_resists: HashMap<EffectType, i16>,
    pub damage_resists: HashMap<DamageType, i16>,
    pub abilities: HashMap<AbilityType, i16>,

    pub skills: Vec<SkillType>,

    pub price: u32,

    //TODO: Stuff attributes with values
    //attributes: HashMap<StuffAttribute>,
}