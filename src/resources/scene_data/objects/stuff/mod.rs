use serde::{ Serialize, Deserialize };

pub mod stuff_attributes;

use crate::scenes::game_scenes::tilemap::tile::Position;

use super::stuff::stuff_attributes::StuffAttribute;

#[derive( PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy )]
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

#[derive( PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy )]
pub enum StuffSubtype {
    Bandage, // stop bleeding
    HealthPack, // add health to part
    DoctorBag, // vs trauma
    Painkillers, // vs pain
    Antibiotic, //vs disease
    Vitamins, // vs fatigue
    Picklock,
    ElectronickPicklock,
    // разные баффы, наркотики
    
}


#[derive( Serialize, Deserialize, Debug, Clone )]
pub struct Stuff{
    id: usize,
    stuff_type: StuffType,
    stuff_subtype: StuffSubtype,

    max_stack_size: usize,
    current_stack_size: usize,

    position: Position<i32>,
    graphic_position: Position<f32>,

    attributes: Vec<StuffAttribute>,

}