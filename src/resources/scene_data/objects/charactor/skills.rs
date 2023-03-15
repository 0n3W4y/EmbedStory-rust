use serde::{Deserialize, Serialize};

pub const MAX_SKILL_VALUE: u16 = 10000;

#[derive( Serialize, Deserialize, Debug, Clone, Eq, PartialEq )]
pub enum Skill{
    Movement(u16),
    CarryWeight(u16),
    MeleeAccuracy(u16),
    RangedAccuracy(u16),
    BandageSpeed(u16),
    DoctorSpeed(u16),
    ThrowingAccuracy(u16),

}

impl Default for Skill{
    fn default() -> Self{
        Skill::Movement(1000)
    }
}