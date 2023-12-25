use std::collections::HashMap;

use serde::{ Serialize, Deserialize };

pub mod stuff_attributes;

use crate::scenes::game_scenes::tilemap::tile::Position;
use crate::resources::scene_data::charactor::StuffWearSlot;

use super::charactor::effects::EffectType;
use super::charactor::skills::{ActiveSkillType, PassiveSkillType};
use super::Damage;

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug)]
pub enum StuffType{
    Weapon(Weapon),
    Item,
    Armor,
    Ammo,
    Food,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug)]
pub enum WeaponType {
    Sword,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug)]
pub struct Weapon {
    pub weapon_type: WeaponType,
    pub critical_hit_chance: i16,
    pub critical_hit_multiplier: i16,
    pub attack_cooldown: i16, // how often charactor may attack; per 100 from delta (100 == 1 sec);
    pub wear_slot: StuffWearSlot,
    pub weapon_range: u8,
    pub damage: HashMap<Damage, i16>,
    pub effects: HashMap<EffectType, u8>,
    pub active_skills: Vec<ActiveSkillType>,
    pub passive_skills: HashMap<PassiveSkillType, u8>,
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Stuff{
    pub id: usize,
    pub stuff_type: StuffType,

    pub max_amount: usize,
    pub amount: usize,

    pub position: Option<Position<i32>>,
    
    //pub stats: HashMap<Stat, i16>,
    //pub resists: HashMap<Resist, i16>,
    //pub abilities: HashMap<Ability, i16>,

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