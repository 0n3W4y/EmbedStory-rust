use serde::{Deserialize, Serialize};

#[derive( Serialize, Deserialize, Debug, Clone, Eq, PartialEq )]
pub enum Skill{
    Movement(u16),
}

impl Default for Skill{
    fn default() -> Self{
        Skill::Movement(1000)
    }
}