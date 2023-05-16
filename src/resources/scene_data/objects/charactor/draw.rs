use bevy::prelude::*;

use crate::resources::scene_manager::SceneManager;
use crate::materials::material_manager::MaterialManager;
use crate::scenes::game_scenes::game_scene::GameScene;
use crate::resources::scene_data::objects::charactor::{Charactor, CharactorType, CharactorSubType, GenderType};
use crate::components::charactor_component::{CharactorComponent, PlayerComponent, NPCComponent, MonsterComponent};
use crate::resources::profile::Profile;

pub const Z_POSITION: f32 = 3.9; // fourth layer;

pub fn draw(
    mut commands: Commands,
    scene_manager: Res<SceneManager>,
    material_manager: Res<MaterialManager>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
){
    let scene: &GameScene = scene_manager.get_current_game_scene();
    let total_tiles = scene.tilemap.get_total_tiles();

    for charactor in scene.charactors.iter(){
        let x: f32 = charactor.graphic_position.x;
        let y: f32 = charactor.graphic_position.y;
        let charactor_type: &CharactorType = &charactor.charactor_type;
        let charactor_subtype = &charactor.charactor_subtype;
        let charactor_gender = &charactor.gender_type;


        let texture_handle: Handle<Image> = material_manager.game_scene.charactors.get_image(charactor_type, charactor_subtype, charactor_gender);
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(128.0, 128.0), 3, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let new_z_position = Z_POSITION - y as f32 / 1000.0;
        let transform = Transform::from_xyz(x, y, new_z_position);

        let mut charactor_component: CharactorComponent = Default::default();
        copy_from_charactor_to_component(charactor, &mut charactor_component);

        match *charactor_type {
            CharactorType::Player => {
                commands.spawn_bundle(SpriteSheetBundle{
                    texture_atlas: texture_atlas_handle,
                    transform,
                    ..default()
                })
                .insert(charactor_component)
                .insert(PlayerComponent);
            },
            CharactorType::NPC => {
                commands.spawn_bundle(SpriteSheetBundle{
                    texture_atlas: texture_atlas_handle,
                    transform,
                    ..default()
                })
                .insert(charactor_component)
                .insert(NPCComponent);
            },
            CharactorType::Monster => {
                commands.spawn_bundle(SpriteSheetBundle{
                    texture_atlas: texture_atlas_handle,
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
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
){
    let player: &Charactor = &profile.charactor;
    let x: f32 = player.graphic_position.x;
    let y: f32 = player.graphic_position.y;
    let charactor_type: &CharactorType = &player.charactor_type;
    let charactor_subtype = &player.charactor_subtype;
    let charactor_gender = &player.gender_type;


    let texture_handle: Handle<Image> = material_manager.game_scene.charactors.get_image(charactor_type, charactor_subtype, charactor_gender);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(128.0, 128.0), 3, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let new_z_position = Z_POSITION - y as f32 / 1000.0;
    let transform = Transform::from_xyz(x, y, new_z_position);

    let mut charactor_component: CharactorComponent = Default::default();
    copy_from_charactor_to_component(player, &mut charactor_component);

    commands.spawn_bundle(SpriteSheetBundle{
        texture_atlas: texture_atlas_handle,
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
    charactor_component.attitude_to_player = charactor.attitude_to_player.clone();
    charactor_component.fraction = charactor.fraction.clone();
    charactor_component.race_type = charactor.race_type.clone();

    charactor_component.position = charactor.position.clone();
    charactor_component.destination_point = charactor.destination_point.clone();
    charactor_component.graphic_position = charactor.graphic_position.clone();

    charactor_component.resists = charactor.resists.clone();
    charactor_component.resists_cache = charactor.resists_cache.clone();
    charactor_component.resist_min_value = charactor.resist_min_value;
    charactor_component.resist_max_value = charactor.resist_max_value;

    charactor_component.stats = charactor.stats.clone();
    charactor_component.stats_cache = charactor.stats_cache.clone();
    charactor_component.stat_min_value = charactor.stat_min_value;

    //charactor_component.skills: Vec<Skill>,
    //charactor_component.skills_cache: Vec<Skill>,

    charactor_component.stuff_storage = charactor.stuff_storage.to_vec();
    charactor_component.stuff_storage_max_slots = charactor.stuff_storage_max_slots;
    charactor_component.stuff_wear = charactor.stuff_wear.clone();

    //charactor_component.charactor_effect: Vec<CharactorEffect>,

    charactor_component.body_structure = charactor.body_structure.clone();
    charactor_component.current_health_points = charactor.current_health_points;
    charactor_component.total_health_points = charactor.total_health_points;

}
pub fn copy_from_component_to_charactor(
    charactor: &mut Charactor,
    charactor_component: CharactorComponent,
){
    charactor.id = charactor_component.id;
    charactor.charactor_type = charactor_component.charactor_type.clone();
    charactor.attitude_to_player = charactor_component.attitude_to_player.clone();
    charactor.fraction = charactor_component.fraction.clone();
    charactor.race_type = charactor_component.race_type.clone();

    charactor.position = charactor_component.position.clone();
    charactor.destination_point = charactor_component.destination_point.clone();
    charactor.graphic_position = charactor_component.graphic_position.clone();

    charactor.resists = charactor_component.resists.clone();
    charactor.resists_cache = charactor_component.resists_cache.clone();
    charactor.resist_min_value = charactor_component.resist_min_value;
    charactor.resist_max_value = charactor_component.resist_max_value;

    charactor.stats = charactor_component.stats.clone();
    charactor.stats_cache = charactor_component.stats_cache.clone();
    charactor.stat_min_value = charactor_component.stat_min_value;

    //charactor.skills: Vec<Skill>,
    //charactor.skills_cache: Vec<Skill>,

    charactor.stuff_storage = charactor_component.stuff_storage.to_vec();
    charactor.stuff_storage_max_slots = charactor_component.stuff_storage_max_slots;
    charactor.stuff_wear = charactor_component.stuff_wear.clone();

    //charactor.charactor_effect: Vec<CharactorEffect>,

    charactor.body_structure = charactor_component.body_structure.clone();
    charactor.current_health_points = charactor_component.current_health_points;
    charactor.total_health_points = charactor_component.total_health_points;
}