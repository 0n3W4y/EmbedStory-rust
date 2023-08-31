use serde::Deserialize;
use std::collections::HashMap;

use crate::resources::scene_data::stuff::{damage_type::DamageType, Stuff};

use super::{effects::EffectType, CharactorType, abilities::AbilityType, StuffWearSlot};

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum SkillType {
    #[default]
    Melee,
    Ranged,
    Magic,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum SkillSubtype {
    #[default]
    SomeSkill,
}

#[derive(Deserialize, Default, Debug, Eq, PartialEq, Clone)]
pub enum CastSource {
    Mouse,
    #[default]
    Itself,
    Target,
}

#[derive(Deserialize, Default, Debug, Eq, PartialEq, Clone)]
pub enum SkillDirectionType {
    #[default]
    Line,
    Arc45,
    Arc90,
    Arc180,
    Arc360,
    Point,
}

#[derive(Default, Debug)]
pub struct Skill {
    pub skill_type: SkillType,
    pub skill_subtype: SkillSubtype,
    pub skill_name: String,
    pub stuff_id: Option<usize>, // link to stuff in wear slot;
    pub is_basic: bool,
    pub is_activated: bool, // activeated skill will start logic to dealt damage to target;
   
    //for passive skill;
    pub is_passive_skill: bool,
    pub trigger_chanse: u8, // chanse to trigger that skill
    pub trigger_time: f32, // 1 per second
    pub trigger_duration: f32, // full time to skill live;
    //-----------------

    pub base_cooldown: i16, // base;
    pub cooldown: f32, // current with multiplier from ability;
    pub on_cooldown: bool, // can use this skill now;
    pub current_duration: f32, // == 0.0;
    pub total_duration: f32,
    //pub activated: bool,

    pub projectiles: u8,
    pub range: u8, // max range; min range = 1;
    pub cast_source: CastSource,
    pub skill_direction: SkillDirectionType,
    pub stamina_cost: i16,
    pub target: CharactorType,

    pub crit_chance: i16,
    pub crit_multiplier: i16,

    pub damage: HashMap<DamageType, i16>,
    pub effect: HashMap<EffectType, u8>,
    pub passive_skill: HashMap<SkillType, u8>,
}

impl Skill {
    pub fn new (config: &SkillDeploy) -> Self {
        Skill {
            skill_type: config.skill_type.clone(),
            skill_subtype: config.skill_subtype.clone(),
            skill_name: config.skill_name.clone(),
            stuff_id: None,
            is_basic: config.is_basic,
            is_activated: false,
            is_passive_skill: config.is_passive_skill,
            trigger_chanse: config.trigger_chanse,
            trigger_time: config.trigger_time as f32 / 10.0,
            trigger_duration: config.trigger_duration as f32 / 10.0,
            base_cooldown: config.cooldown,
            cooldown: config.cooldown as f32 / 100.0,
            on_cooldown: false,
            current_duration: 0.0,
            total_duration: 0.0,
            projectiles: config.projectiles,
            range: config.range,
            cast_source: config.cast_source,
            skill_direction: config.skill_direction.clone(),
            stamina_cost: config.stamina_cost,
            target: config.target,
            crit_chance: config.crit_chance,
            crit_multiplier: config.crit_multiplier,
            damage: config.damage.clone(),
            effect: config.effect.clone(),
            passive_skill: config.passive_skill.clone(),
        }
    }
}

#[derive(Deserialize)]
pub struct SkillDeploy {
    pub skill_type: SkillType,
    pub skill_subtype: SkillSubtype,
    pub skill_name: String,
    pub is_passive_skill: bool,
    pub is_basic: bool,
    //pub stuff_id: usize, // link to stuff in wear slot;

    pub trigger_chanse: u8,
    pub trigger_time: u16,
    pub trigger_duration: u16,
    pub cooldown: i16,

    pub projectiles: u8,
    pub range: u8,
    pub cast_source: CastSource,
    pub skill_direction: SkillDirectionType,
    pub stamina_cost: i16,
    pub target: CharactorType,

    pub crit_chance: i16,
    pub crit_multiplier: i16,

    pub damage: HashMap<DamageType, i16>, 
    pub effect: HashMap<EffectType, u8>, // effect type and effect trigger chanse;
    pub passive_skill: HashMap<SkillType, u8>, // skill type and skill trigger chanse;
}


pub fn update_basic_skill_by_changes_in_ability(base_skill: Option<&mut Skill>, ability_storage: &HashMap<AbilityType, i16>, wear_stuff: &HashMap<StuffWearSlot, Option<Stuff>>) {
    match base_skill {
        Some(skill) => {
            //clear for new entries;
            skill.damage.clear();
            skill.effect.clear();
            skill.passive_skill.clear();

            //first get damage multiplier by weapon type;
            let skill_type_multiplier = match skill.skill_type {
                SkillType::Melee => { 
                    match ability_storage.get(&AbilityType::MeleeDamage) {
                        Some(v) => *v,
                        None => {
                            println!("Can not get Melee damage from ability, use 100% instead");
                            100
                        },
                    }
                },
                SkillType::Ranged => {
                    match ability_storage.get(&AbilityType::RangedDamage) {
                        Some(v) => *v,
                        None => {
                            println!("Can not get Ranged damage from ability, use 100% instead");
                            100
                        },
                    }
                },
                SkillType::Magic => {
                    match ability_storage.get(&AbilityType::MagicDamage) {
                        Some(v) => *v,
                        None => {
                            println!("Can not get Magic damage from ability, use 100% instead");
                            100
                        },
                    }
                },
            };

            //get critical hit chanse from ability;
            let critical_chanse_from_ability = match ability_storage.get(&AbilityType::CriticalHitChanse) {
                Some(v) => *v,
                None => {
                    println!("Can not get Critical Chanse from ability, use 0 instead");
                    0
                },
            };

            //get critical hit multiplier from ability;
            let critical_multiplier_from_ability = match ability_storage.get(&AbilityType::CriticalHitMultiplier) {
                Some(v) => *v,
                None => {
                    println!("Can not get Critical Multiplier from ability, use 0 instead");
                    0
                }
            };

            let attack_speed_from_ability = match ability_storage.get(&AbilityType::AttackSpeed) {
                Some(v) => *v,
                None => {
                    println!("Can not get Atack Speed from ability, use 100% instead");
                    100
                }
            };

            let mut skill_cooldown: i16 = 0;
            let mut critical_chanse: i16 = 0;
            let mut critical_multiplier: i16 = 0;
            let mut damage_from_weapon: HashMap<DamageType, i16> = HashMap::new();
            let mut effects_from_weapon: HashMap<EffectType, i16> = HashMap::new();
            let mut passive_skills_from_weapon: HashMap<SkillType, i16> = HashMap::new();

            //get weapon from right hand
            match wear_stuff.get(&StuffWearSlot::RightHand).unwrap() {
                Some(v) => {
                    critical_chanse = v.critical_hit_chanse;
                    critical_multiplier = v.critical_multiplier;
                    skill_cooldown = v.cooldown;
                    for (damage_type, value) in v.damage.iter() {
                        damage_from_weapon.insert(damage_type.clone(), *value);
                    }

                    for(effect_type, value) in v.effects.iter() {
                        effects_from_weapon.insert(effect_type.clone(), *value);
                    }

                    for(skill_type, value) in v.passive_skills.iter() {
                        passive_skills_from_weapon.insert(skill_type.clone(), *value);
                    }
                },
                None => {
                    skill_cooldown = 100; // by default 1 shot per second;
                    damage_from_weapon.insert(DamageType::Crushing, 5); // default punch;
                },
            };

            //get weapon from left hand
            match wear_stuff.get(&StuffWearSlot::LeftHand).unwrap() {
                Some(v) => {
                    //middle value from 2 weapons;
                    critical_chanse =  if v.critical_hit_chanse > 0 {
                        (critical_chanse + v.critical_hit_chanse) /2 
                    } else {
                        critical_chanse
                    };

                    critical_multiplier = if v.critical_multiplier > 0 {
                        (critical_multiplier + v.critical_multiplier) /2
                    } else {
                        critical_multiplier
                    };
                    // middle value from 2 weapons;
                    skill_cooldown = if v.cooldown > 0 {
                        (skill_cooldown + v.cooldown) / 2 // if 0 = we r have buckler or something same;
                    } else {
                        skill_cooldown
                    };

                    for (damage_type, value) in v.damage.iter() {
                        damage_from_weapon.entry(damage_type.clone()).and_modify(|x| {*x += *value}).or_insert(*value);
                    }

                    for(effect_type, value) in v.effects.iter() {
                        effects_from_weapon.entry(effect_type.clone()).and_modify(|x| {*x += *value}).or_insert(*value);
                    }

                    for(skill_type, value) in v.passive_skills.iter() {
                        passive_skills_from_weapon.entry(skill_type.clone()).and_modify(|x| {*x += *value}).or_insert(*value);
                    }
                },
                None => {},
            };

            skill.cooldown = (skill_cooldown as f32 / 100.0) / (attack_speed_from_ability as f32 / 100.0); // weapon cooldown like 122 / 100; and attack speed like 100% / 100;
            skill.crit_chance = critical_chanse + critical_chanse_from_ability;
            skill.crit_multiplier = critical_multiplier + critical_multiplier_from_ability;


            for (damage_type, value) in damage_from_weapon.iter() {
                let new_value = match damage_type {
                    DamageType::Fire => {
                        match ability_storage.get(&AbilityType::FireDamage) {
                            Some(v) => *value * (*v / 100),
                            None => *value,
                        }
                    },
                    DamageType::Cold => todo!(),
                    DamageType::Electric => todo!(),
                    DamageType::Cutting => todo!(),
                    DamageType::Piercing => todo!(),
                    DamageType::Crushing => todo!(),
                    DamageType::Water => todo!(),
                    DamageType::Acid => todo!(),
                    DamageType::Poison => todo!(),
                };
                skill.damage.insert(damage_type.clone(), new_value);
            }

            for (effect_type, value) in effects_from_weapon.iter() {
                let new_value = match *effect_type {
                    EffectType::Stun => { 
                        let temp_value = match ability_storage.get(&AbilityType::Stun) {
                            Some(v) => {
                                *value + v
                            },
                            None => *value,
                        };
    
                        if temp_value < 0 {
                            0
                        } else if  temp_value > 100 {
                            100
                        } else {
                            temp_value as u8
                        }
                    },
                    EffectType::Acid => todo!(),
                    EffectType::Moveless => todo!(),
                    EffectType::Slow => todo!(),
                    EffectType::Bleeding => todo!(),
                    EffectType::Burn => todo!(),
                    EffectType::Electrification => todo!(),
                    EffectType::Freeze => todo!(),
                    EffectType::Blind => todo!(),
                    EffectType::Poison => todo!(),
                    EffectType::Wet => todo!(),
                    EffectType::BrokeArmor => todo!(),
                    EffectType::BrokeWeapon => todo!(),
                    EffectType::IncreaseMovement => todo!(),
                    EffectType::Frostbite => todo!(),
                };
    
                skill.effect.insert(effect_type.clone(), new_value);
            }
    
            for (passive_skill_type, value) in passive_skills_from_weapon.iter(){
                let new_value = if *value < 0 {
                    0
                } else if *value > 100 {
                    100
                } else {
                    *value as u8
                };
    
                skill.passive_skill.insert(passive_skill_type.clone(), new_value);
            }
        },
        None => println!("Can not udapte basic skill, because basic skill not found"),
    }
}
