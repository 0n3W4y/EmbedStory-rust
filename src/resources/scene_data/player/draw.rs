use bevy::prelude::*;

use crate::config::TILE_SIZE;
use crate::resources::scene_manager::SceneManager;
use crate::materials::material_manager::MaterialManager;
use crate::scenes::game_scenes::game_scene::GameScene;
use crate::resources::scene_data::objects::charactor::{Charactor, CharactorType};
use crate::components::charactor_component::{CharactorComponent, PlayerComponent, NPCComponent, MonsterComponent};
use crate::resources::profile::Profile;

pub const Z_POSITION: f32 = 3.9; // fourth layer;

pub fn draw(
    mut commands: Commands,
    scene_manager: Res<SceneManager>,
    material_manager: Res<MaterialManager>,
){
    let scene: &GameScene = scene_manager.get_current_game_scene();

    for charactor in scene.charactors.iter(){
        let x: f32 = (charactor.position.x * TILE_SIZE as i32) as f32;
        let y: f32 = (charactor.position.y * TILE_SIZE as i32) as f32;
        let charactor_type: &CharactorType = &charactor.charactor_type;
        let charactor_gender = &charactor.gender_type;
        let charactor_racetype = &charactor.race_type;


        let texture: Handle<TextureAtlas> = material_manager.game_scene.charactors.get_atlas(charactor_racetype, charactor_gender);

        let new_z_position = Z_POSITION - y as f32 / 1000.0;
        let transform = Transform::from_xyz(x, y, new_z_position);

        let mut charactor_component: CharactorComponent = Default::default();
        copy_from_charactor_to_component(charactor, &mut charactor_component);

        match *charactor_type {
            CharactorType::Player => {
                commands.spawn_bundle(SpriteSheetBundle{
                    texture_atlas: texture,
                    transform,
                    ..default()
                })
                .insert(charactor_component)
                .insert(PlayerComponent);
            },
            CharactorType::NPC => {
                commands.spawn_bundle(SpriteSheetBundle{
                    texture_atlas: texture,
                    transform,
                    ..default()
                })
                .insert(charactor_component)
                .insert(NPCComponent);
            },
            CharactorType::Monster => {
                commands.spawn_bundle(SpriteSheetBundle{
                    texture_atlas: texture,
                    transform,
                    ..default()
                })
                .insert(charactor_component)
                .insert(MonsterComponent);
            },
        };        
    }

}

pub fn draw_player(
    mut commands: Commands,
    profile: Res<Profile>,
    material_manager: Res<MaterialManager>,
){
    let player: &Charactor = &profile.charactor;
    let x: f32 = (player.position.x * TILE_SIZE as i32) as f32;
    let y: f32 = (player.position.y * TILE_SIZE as i32) as f32;
    let charactor_gender = &player.gender_type;
    let charactor_racetype = &player.race_type;


    let texture: Handle<TextureAtlas> = material_manager.game_scene.charactors.get_atlas(charactor_racetype, charactor_gender);

    let new_z_position = Z_POSITION - y as f32 / 1000.0;
    let transform = Transform::from_xyz(x, y, new_z_position);

    let mut charactor_component: CharactorComponent = Default::default();
    copy_from_charactor_to_component(player, &mut charactor_component);

    commands.spawn_bundle(SpriteSheetBundle{
        texture_atlas: texture,
        transform,
        ..default()
    })
    .insert(charactor_component)
    .insert(PlayerComponent);
}

pub fn copy_from_charactor_to_component(
    charactor: &Charactor,
    charactor_component: &mut CharactorComponent,
){
    charactor_component.id = charactor.id;
    charactor_component.charactor_type = charactor.charactor_type.clone();
    charactor_component.charactor_subtype = charactor.charactor_subtype.clone();
    charactor_component.attitude_to_player = charactor.attitude_to_player.clone();
    charactor_component.fraction = charactor.fraction.clone();
    charactor_component.race_type = charactor.race_type.clone();

    charactor_component.position = charactor.position.clone();
    charactor_component.destination_point = charactor.destination_point.clone();
    charactor_component.destination_path = charactor.destination_path.to_vec();
    charactor_component.destination_direction = charactor.destination_direction.clone();

    charactor_component.resists = charactor.resists.clone();
    charactor_component.resists_cache = charactor.resists_cache.clone();
    charactor_component.resist_min_value = charactor.resist_min_value;
    charactor_component.resist_max_value = charactor.resist_max_value;

    charactor_component.stats = charactor.stats.clone();
    charactor_component.stats_cache = charactor.stats_cache.clone();
    charactor_component.stat_min_value = charactor.stat_min_value;

    charactor_component.skills = charactor.skills.clone();
    charactor_component.skills_cache = charactor.skills_cache.clone();

    charactor_component.stuff_storage = charactor.stuff_storage.to_vec();
    charactor_component.stuff_storage_max_slots = charactor.stuff_storage_max_slots;
    charactor_component.stuff_wear = charactor.stuff_wear.clone();

    //charactor_component.charactor_effect: Vec<CharactorEffect>,

    charactor_component.body_structure = charactor.body_structure.clone();
    charactor_component.current_health_points = charactor.current_health_points;
    charactor_component.total_health_points = charactor.total_health_points;

}