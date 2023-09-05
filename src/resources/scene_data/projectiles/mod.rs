use serde::Deserialize;

pub mod update_projectile;

#[derive(Default, Debug, Deserialize, PartialEq, Eq, Clone)]
pub enum ProjectileType {
    #[default]
    Arrow,
    Bullet,
    Sphere,
    None,
}

