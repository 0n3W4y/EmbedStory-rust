use std::collections::HashMap;

use serde::{ Serialize, Deserialize };

pub mod stuff_attributes;
pub mod damage_type;
pub mod resists_types;

use crate::scenes::game_scenes::tilemap::tile::Position;

use crate::resources::scene_data::charactor::StuffWearSlot;

use self::damage_type::DamageType;
use self::resists_types::ResistType;

use super::charactor::abilities::AbilityType;
use super::charactor::skills::SkillType;
use super::charactor::{stats::Stat, effects::EffectType};

#[derive( PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Hash )]
pub enum StuffType{
    Weapon(WeaponType),
    Item,
    Armor,
    Ammo,
    Food,
}

#[derive( PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Hash )]
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

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Hash)]
pub enum WeaponType {
    Sword,
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Stuff{
    pub id: usize,
    pub stuff_type: StuffType,

    pub max_amount: usize,
    pub amount: usize,

    pub position: Option<Position<i32>>,
    pub wear_slot: Option<StuffWearSlot>,

    pub damage: HashMap<DamageType, i16>,
    pub critical_hit_chance: i16,
    pub critical_hit_multiplier: i16,
    pub attack_cooldown: i16, // how often charactor may attack; per 100 from delta (100 == 1 sec);

    pub effects: HashMap<EffectType, u8>,
    pub skills: Vec<SkillType>,
    pub extra_skills: HashMap<SkillType, u8>,

    pub stats: HashMap<Stat, i16>,
    pub resists: HashMap<ResistType, i16>,
    pub abilities: HashMap<AbilityType, i16>,

    pub price: u32,

    //TODO: Stuff attributes with values
    //attributes: HashMap<StuffAttribute>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StuffConfig {
    pub stuff_type: StuffType,
    pub max_amount: usize,
    pub wear_slot:Option<StuffWearSlot>,
    pub damage: HashMap<DamageType, i16>,
    pub critical_hit_chance: i16,
    pub critical_hit_multiplier: i16,
    pub attack_cooldown: i16,
    pub effects: HashMap<EffectType, u8>,
    pub skills: Vec<SkillType>,
    pub extra_skills: HashMap<SkillType, i16>,
    pub price: u32,
    //pub params for add or substruct stats, resists, abilities;
}