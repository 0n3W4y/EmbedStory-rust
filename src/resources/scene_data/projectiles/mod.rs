use serde::{Deserialize, Serialize};

pub mod update_projectile;

#[derive(Default, Debug, Deserialize, PartialEq, Eq, Clone, Serialize)]
pub enum ProjectileType {
    #[default]
    Arrow,
    Bullet,
    FireSphere,
    None,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProjectileConfig{
    pub projectile_type: ProjectileType,
    pub can_pierce: bool,
    pub pierce_chance: u8,
    pub velocity: u16,
}

