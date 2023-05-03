use serde::{ Serialize, Deserialize };

pub mod stuff_attributes;

use crate::scenes::game_scenes::tilemap::tile::Position;

use super::stuff::stuff_attributes::StuffAttribute;
use crate::resources::scene_data::objects::charactor::StuffWearSlot;

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
    WeaponUpgrade,
    ArmorUpgrade,
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
    ElectronickPicklock,
    // разные баффы, наркотики
    // апгрейды для оружия
    // апгрейды для брони
    // собственно сама броня
    // разные оружия
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Stuff{
    pub id: usize,
    pub stuff_type: StuffType,
    pub stuff_subtype: StuffSubtype,

    pub max_amount: usize,
    pub amount: usize,

    pub position: Position<i32>,
    pub graphic_position: Position<f32>,

    pub wear_slot: StuffWearSlot,

    //TODO: Stuff attributes with values
    //attributes: HashMap<StuffAttribute>,
}