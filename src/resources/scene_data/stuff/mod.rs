use std::collections::HashMap;

use serde::{ Serialize, Deserialize };

pub mod stuff_attributes;

use crate::scenes::game_scenes::tilemap::tile::Position;
use crate::resources::scene_data::charactor::StuffWearSlot;

use super::charactor::effects::EffectType;
use super::charactor::skills::{ActiveSkillType, PassiveSkillType, PassiveSkill};
use super::{Stat, Ability, Resist, Damage};

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

    //pub weapon: Option<Weapon>,
    //pub armor: Option<Armor>,
    pub critical_hit_chance: i16,
    pub critical_hit_multiplier: i16,
    pub attack_cooldown: i16, // how often charactor may attack; per 100 from delta (100 == 1 sec);

    pub effects: HashMap<EffectType, u8>,
    pub active_skills: Vec<ActiveSkillType>,
    pub passive_skills: HashMap<PassiveSkillType, (PassiveSkill, u8)>,

    pub damage: HashMap<Damage, i16>,
    pub stats: HashMap<Stat, i16>,
    pub resists: HashMap<Resist, i16>,
    pub abilities: HashMap<Ability, i16>,

    pub price: u32,

    //TODO: Stuff attributes with values
    //attributes: HashMap<StuffAttribute>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StuffConfig {
    pub stuff_type: StuffType,
    pub max_amount: usize,
    pub wear_slot:Option<StuffWearSlot>,
    pub damage: HashMap<Damage, i16>,
    pub critical_hit_chance: i16,
    pub critical_hit_multiplier: i16,
    pub attack_cooldown: i16,
    pub effects: HashMap<EffectType, u8>,
    pub active_skills: Vec<ActiveSkillType>,
    pub passive_skills: HashMap<PassiveSkillType, u8>,
    pub price: u32,
    //pub params for add or substruct stats, resists, abilities;
}