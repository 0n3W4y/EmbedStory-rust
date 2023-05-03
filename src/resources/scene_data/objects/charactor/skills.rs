use serde::{Deserialize, Serialize};

pub const MAX_SKILL_VALUE: u16 = 20000;

#[derive( Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, Default )]
pub enum Skill{
    #[default]
    Movement,
    CarryWeight,
    MeleeAccuracy,
    RangedAccuracy,
    BandageSpeed,
    DoctorSpeed,
    ThrowingAccuracy,
    RangedDistance,
}