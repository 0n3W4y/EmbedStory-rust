use bevy::prelude::*;

use self::{tile_material::TileMaterial, thing_material::ThingMaterial, charactors_material::CharactorsMaterial, projectiles_material::ProjectilesMaterial};

pub mod charactors_material;
pub mod projectiles_material;
pub mod thing_material;
pub mod tile_material;

#[derive(Debug, Clone)]
pub struct GameSceneMaterial {
    pub tile: TileMaterial,
    pub things: ThingMaterial,
    pub charactors: CharactorsMaterial,
    pub projectiles: ProjectilesMaterial,
}

impl GameSceneMaterial {
    pub fn load_ground_scene_material(
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) -> Self {
        let tile: TileMaterial = TileMaterial::new(asset_server, texture_atlases);
        let things: ThingMaterial = ThingMaterial::new(asset_server, texture_atlases);
        let charactors: CharactorsMaterial = CharactorsMaterial::new(asset_server, texture_atlases);
        let projectiles: ProjectilesMaterial = ProjectilesMaterial::new(asset_server, texture_atlases);

        return GameSceneMaterial {
            tile,
            things,
            charactors,
            projectiles
        };
    }
}