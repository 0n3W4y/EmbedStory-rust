use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::resources::scene_data::{stuff::{damage_type::DamageType, Stuff}, projectiles::ProjectileType};

use super::{effects::EffectType, abilities::{AbilityType, self}, StuffWearSlot};

pub const MINIMAL_TIME_FOR_COOLDOWN_BASIC_SKILL: f32 = 0.25;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum SkillType {
    #[default]
    BaseSkill
}

#[derive(Serialize, Deserialize, Default, Debug, Eq, PartialEq, Clone)]
pub enum CastSource {
    Mouse,
    #[default]
    Itself
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum TargetType {
    #[default]
    Allies,
    Enemies,
    All,
}

#[derive(Serialize, Deserialize, Default, Debug, Eq, PartialEq, Clone)]
pub enum SkillDirectionType {
    #[default]
    Line,
    Arc15,
    Arc30,
    Arc45,
    Arc60,
    Arc90,
    Arc180,
    Arc360,
    Point,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Skill {
    pub skill_type: SkillType,
    pub skill_name: String,
    pub stuff_id: Option<usize>, // link to stuff in wear slot;
    pub is_passive_skill: bool,

    //for active skill;
    pub is_activated: bool, // activeated skill will start logic to dealt damage to target;
    pub on_cooldown: bool, // can use this skill now;
    pub cooldown_time: f32, // base;
    pub current_time_duration: f32, // == 0.0;
    

    //for passive skills
    pub trigger_chance: u8, // chanse to trigger that skill
    pub trigger_time_frequency: f32, // 1 per second
    pub life_time: f32, // full time to skill live;
    pub total_time_duration: f32,

    pub projectiles: u8,
    pub projectile_type: ProjectileType,
    pub range: u8, // max range; min range = 1;
    pub cast_source: CastSource,
    pub skill_direction: SkillDirectionType,
    pub stamina_cost: i16,
    pub target: TargetType,

    pub crit_chance: i16,
    pub crit_multiplier: i16,

    pub damage: HashMap<DamageType, i16>,
    pub effect: HashMap<EffectType, u8>,
    pub extra_skill: HashMap<SkillType, u8>,
}

impl Skill {
    pub fn new (config: &SkillDeploy) -> Self {
        Skill {
            skill_type: config.skill_type.clone(),
            skill_name: config.skill_name.clone(),
            stuff_id: None,
            is_activated: false,
            is_passive_skill: config.is_passive_skill,
            trigger_chance: config.trigger_chance,
            trigger_time_frequency: config.trigger_time_frequency as f32 / 10.0,
            life_time: config.life_time as f32 / 10.0,
            cooldown_time: config.cooldown_time as f32 / 10.0,
            on_cooldown: false,
            current_time_duration: 0.0,
            total_time_duration: 0.0,
            projectiles: config.projectiles,
            projectile_type: config.projectile_type.clone(),
            range: config.range,
            cast_source: config.cast_source.clone(),
            skill_direction: config.skill_direction.clone(),
            stamina_cost: config.stamina_cost,
            target: config.target.clone(),
            crit_chance: config.crit_chance,
            crit_multiplier: config.crit_multiplier,
            damage: config.damage.clone(),
            effect: config.effect.clone(),
            extra_skill: config.extra_skill.clone(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct SkillDeploy {
    pub skill_type: SkillType,
    pub skill_name: String,
    pub is_passive_skill: bool,

    //for active skill;
    pub cooldown_time: i16, // base;    

    //for passive skills
    pub trigger_chance: u8, // chanse to trigger that skill
    pub trigger_time_frequency: f32, // 1 per second
    pub life_time: f32, // full time to skill live;

    pub projectiles: u8,
    pub projectile_type: ProjectileType,
    pub range: u8, // max range; min range = 1;
    pub cast_source: CastSource,
    pub skill_direction: SkillDirectionType,
    pub stamina_cost: i16,
    pub target: TargetType,

    pub crit_chance: i16,
    pub crit_multiplier: i16,

    pub damage: HashMap<DamageType, i16>,
    pub effect: HashMap<EffectType, u8>,
    pub extra_skill: HashMap<SkillType, u8>,
}


pub fn update_basic_skill_by_changes_in_ability(base_skill: Option<&mut Skill>, ability_storage: &HashMap<AbilityType, i16>, wear_stuff: &HashMap<StuffWearSlot, Option<Stuff>>) {
    match base_skill {
        Some(skill) => {
            //clear for new entries;
            skill.damage.clear();
            skill.effect.clear();
            skill.extra_skill.clear();
            
            let critical_hit_chance_from_ability = match ability_storage.get(&AbilityType::CriticalHitChanse) {     //get critical hit chance from ability;
                Some(v) => *v,
                None => {
                    println!("Can not get Critical Chance from ability, use 0 instead");
                    0
                },
            };
            let critical_hit_multiplier_from_ability = match ability_storage.get(&AbilityType::CriticalHitMultiplier) {     //get critical hit multiplier from ability;
                Some(v) => *v,
                None => {
                    println!("Can not get Critical Multiplier from ability, use 0 instead");
                    0
                }
            };
            let attack_speed_from_ability = match ability_storage.get(&AbilityType::AttackSpeed) {      //get atk speed from ability;
                Some(v) => *v,
                None => {
                    println!("Can not get Atack Speed from ability, use 100% instead");
                    100
                }
            };

            let mut skill_cooldown: i16 = 0;
            let mut critical_chance: i16 = 0;
            let mut critical_multiplier: i16 = 0;
            let mut damage_from_weapon: HashMap<DamageType, i16> = HashMap::new();
            let mut effects_from_weapon: HashMap<EffectType, u8> = HashMap::new();
            let mut extra_skills_from_weapon: HashMap<SkillType, u8> = HashMap::new();
            let mut skip_left_hand: bool = false;             //check for TwoHanded weapon;

            match wear_stuff.get(&StuffWearSlot::RightHand).unwrap() {          //get weapon from right hand
                Some(weapon) => {
                    critical_chance = weapon.critical_hit_chance;
                    critical_multiplier = weapon.critical_hit_multiplier;
                    skill_cooldown = weapon.attack_cooldown;
                    damage_from_weapon = weapon.damage.clone();
                    effects_from_weapon = weapon.effects.clone();
                    extra_skills_from_weapon = weapon.extra_skills.clone();
                    if weapon.wear_slot.unwrap() == StuffWearSlot::RightAndLeftHand {                        //safe unwrap;
                        skip_left_hand = true;
                    }

                },
                None => {
                    skill_cooldown = 100;                                                                       // by default 1 shot per second;
                    damage_from_weapon.insert(DamageType::Phisical, 5);                                 // default punch;
                },
            };

            if !skip_left_hand {
                match wear_stuff.get(&StuffWearSlot::LeftHand).unwrap() {                                       //get weapon from left hand
                    Some(weapon) => {
                        critical_chance = (critical_chance + weapon.critical_hit_chance) / 2;                       //middle value from 2 weapons;
                        critical_multiplier = (critical_multiplier + weapon.critical_hit_multiplier) / 2;           //middle value from 2 weapons;
                        skill_cooldown = (skill_cooldown + weapon.attack_cooldown) / 2;                                //middle value from 2 weapons;
    
                        for (damage_type, value) in weapon.damage.iter() {                     //stocking damage values into 1 hashmap
                            damage_from_weapon.entry(damage_type.clone()).and_modify(|x| {*x += *value}).or_insert(*value);
                        }
    
                        for(effect_type, value) in weapon.effects.iter() {                      //stocking effects into 1 hashmap;
                            effects_from_weapon.entry(effect_type.clone()).and_modify(|x| {*x += *value}).or_insert(*value);
                        }
    
                        for(skill_type, value) in weapon.extra_skills.iter() {                           //stocking passive skills into 1 hashmap;
                            extra_skills_from_weapon.entry(skill_type.clone()).and_modify(|x| {*x += *value}).or_insert(*value);
                        }
                    },
                    None => {},
                };
            }            

            let interim_cooldown = (skill_cooldown as f32 / 100.0) - ((skill_cooldown * attack_speed_from_ability) as f32 / 100.0);               //calculate cooldown value;
            skill.cooldown_time = if interim_cooldown >= MINIMAL_TIME_FOR_COOLDOWN_BASIC_SKILL {          //check for cooldown value;
                MINIMAL_TIME_FOR_COOLDOWN_BASIC_SKILL
            } else {
                interim_cooldown
            };

            skill.crit_chance = critical_chance + critical_hit_chance_from_ability;             //set to skill crit chance;
            skill.crit_multiplier = critical_multiplier + critical_hit_multiplier_from_ability;             //set to skill crit multiplier;


            for (damage_type, value) in damage_from_weapon.iter() {             //collect damage from abilities and calculate new values;
                let ability_type = abilities::get_ability_type_from_damage_type(damage_type);
                let damage_multiplier_from_ability = match ability_storage.get(&ability_type){
                    Some(v) => *v,
                    None => 0,
                };
                let new_value = *value + (*value * damage_multiplier_from_ability / 100);
                skill.damage.insert(damage_type.clone(), new_value);            //insert new damage values into skill;
            }

            skill.effect = effects_from_weapon;
            skill.extra_skill = extra_skills_from_weapon;

        },
        None => println!("Can not udapte basic skill, because basic skill not found"),
    }
}
