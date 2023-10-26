use serde::{Deserialize, Serialize};

pub mod update_projectile;

#[derive(Default, Debug, Deserialize, PartialEq, Eq, Clone, Serialize)]
pub enum ProjectileType {
    #[default]
    Arrow,
    Bullet,
    Sphere,
    None,
}

