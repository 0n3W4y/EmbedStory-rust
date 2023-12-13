use bevy::prelude::*;

use crate::{resources::scene_data::projectiles::ProjectileType, config::TILE_SIZE};

#[derive(Debug, Clone)]
pub struct ProjectilesMaterial {
    pub arrow_atlas: Handle<TextureAtlas>,
    pub bullet_atlas: Handle<TextureAtlas>,
    pub sphere_atlas: Handle<TextureAtlas>,
}

impl ProjectilesMaterial {
    pub fn new(
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) -> Self {
        load_projectiles_material(asset_server, texture_atlases)
    }

    pub fn get_texture_atlas(&self, projectile_type: &ProjectileType) -> Handle<TextureAtlas> {
        match projectile_type {
            ProjectileType::Arrow => self.arrow_atlas.clone_weak(),
            ProjectileType::Bullet => self.bullet_atlas.clone_weak(),
            ProjectileType::FireSphere => self.sphere_atlas.clone_weak(),
            ProjectileType::None => panic!("Try to get projectile texture without projectile type"),
        }
    }
}



fn load_projectiles_material(
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> ProjectilesMaterial {
    let arrow_texture_handle: Handle<Image> = asset_server.load("textures/projectile/arrow.png");
    let bullet_texture_handle: Handle<Image> = asset_server.load("textures/projectile/bullet.png");
    let sphere_texture_handle: Handle<Image> = asset_server.load("textures/projectile/sphere.png");

    let arrow_texture_atlas = TextureAtlas::from_grid(
        arrow_texture_handle,
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
        1,
        1,
        None,
        None,
    );
    let bullet_texture_atlas = TextureAtlas::from_grid(
        bullet_texture_handle,
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
        1,
        1,
        None,
        None,
    );
    let sphere_texture_atlas = TextureAtlas::from_grid(
        sphere_texture_handle,
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
        1,
        1,
        None,
        None,
    );

    let arrow_atlas = texture_atlases.add(arrow_texture_atlas);
    let bullet_atlas = texture_atlases.add(bullet_texture_atlas);
    let sphere_atlas = texture_atlases.add(sphere_texture_atlas);

    ProjectilesMaterial {
        arrow_atlas,
        bullet_atlas,
        sphere_atlas,
    }
}