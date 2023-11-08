use bevy::prelude::*;

use crate::config::TILE_SIZE;
use crate::resources::scene_manager::SceneManager;
use crate::materials::material_manager::MaterialManager;
use crate::scenes::game_scenes::game_scene::GameScene;
use crate::resources::scene_data::charactor::{Charactor, CharactorType};
use crate::components::charactor_component::{CharactorComponent, PlayerComponent, NPCComponent, MonsterComponent, ResistsComponent, SkillComponent, CharactorTextComponent, PositionComponent, EffectComponent, CharactorAnimationComponent, StatsComponent, AbilityComponent, InventoryComponent, CompanionComponent};
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
        let mut resist_component: ResistsComponent = Default::default();
        let mut skill_component: SkillComponent = Default::default();
        let mut text_component: CharactorTextComponent = Default::default();
        let mut position_component: PositionComponent = Default::default();
        let mut effect_component: EffectComponent = Default::default();
        let mut animation_component: CharactorAnimationComponent = Default::default();
        let mut stats_component: StatsComponent = Default::default();
        let mut ability_component: AbilityComponent = Default::default();
        let mut inventory_component: InventoryComponent = Default::default();
        copy_from_charactor_to_component(
            charactor,
            &mut charactor_component,
            &mut resist_component,
            &mut skill_component,
            &mut position_component,
            &mut effect_component,
            &mut stats_component,
            &mut ability_component,
            &mut inventory_component,
        );

        let charactor_type_component = match *charactor_type {
            CharactorType::Player => {
                commands.spawn_bundle(SpriteSheetBundle{
                    texture_atlas: texture,
                    transform,
                    ..default()
                })
                .insert(charactor_component)
                .insert(resist_component)
                .insert(skill_component)
                .insert(text_component)
                .insert(position_component)
                .insert(effect_component)
                .insert(animation_component)
                .insert(stats_component)
                .insert(ability_component)
                .insert(inventory_component)
                .insert(PlayerComponent); 
            },
            CharactorType::NPC => {
                commands.spawn_bundle(SpriteSheetBundle{
                    texture_atlas: texture,
                    transform,
                    ..default()
                })
                .insert(charactor_component)
                .insert(resist_component)
                .insert(skill_component)
                .insert(text_component)
                .insert(position_component)
                .insert(effect_component)
                .insert(animation_component)
                .insert(stats_component)
                .insert(ability_component)
                .insert(inventory_component)
                .insert(NPCComponent); 
            },
            CharactorType::Monster => {
                commands.spawn_bundle(SpriteSheetBundle{
                    texture_atlas: texture,
                    transform,
                    ..default()
                })
                .insert(charactor_component)
                .insert(resist_component)
                .insert(skill_component)
                .insert(text_component)
                .insert(position_component)
                .insert(effect_component)
                .insert(animation_component)
                .insert(stats_component)
                .insert(ability_component)
                .insert(inventory_component)
                .insert(MonsterComponent); 
            },
            CharactorType::Companion => {
                commands.spawn_bundle(SpriteSheetBundle{
                    texture_atlas: texture,
                    transform,
                    ..default()
                })
                .insert(charactor_component)
                .insert(resist_component)
                .insert(skill_component)
                .insert(text_component)
                .insert(position_component)
                .insert(effect_component)
                .insert(animation_component)
                .insert(stats_component)
                .insert(ability_component)
                .insert(inventory_component)
                .insert(CompanionComponent); 
            }
        }; 
    }
}

pub fn draw_player(
    mut commands: Commands,
    profile: Res<Profile>,
    material_manager: Res<MaterialManager>,
){
    let player: &Charactor = &profile.charactor.unwrap();
    let x: f32 = (player.position.x * TILE_SIZE as i32) as f32;
    let y: f32 = (player.position.y * TILE_SIZE as i32) as f32;
    let charactor_gender = &player.gender_type;
    let charactor_racetype = &player.race_type;


    let texture: Handle<TextureAtlas> = material_manager.game_scene.charactors.get_atlas(charactor_racetype, charactor_gender);

    let new_z_position = Z_POSITION - y as f32 / 1000.0;
    let transform = Transform::from_xyz(x, y, new_z_position);

    let mut charactor_component: CharactorComponent = Default::default();
    let mut resist_component: ResistsComponent = Default::default();
    let mut skill_component: SkillComponent = Default::default();
    let mut text_component: CharactorTextComponent = Default::default();
    let mut position_component: PositionComponent = Default::default();
    let mut effect_component: EffectComponent = Default::default();
    let mut animation_component: CharactorAnimationComponent = Default::default();
    let mut stats_component: StatsComponent = Default::default();
    let mut ability_component: AbilityComponent = Default::default();
    let mut inventory_component: InventoryComponent = Default::default();

    copy_from_charactor_to_component(
        player, 
        &mut charactor_component,
        &mut resist_component,
        &mut skill_component,
        &mut position_component,
        &mut effect_component,
        &mut stats_component,
        &mut ability_component,
        &mut inventory_component,
    );

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
    resist_component: &mut ResistsComponent,
    skill_component: &mut SkillComponent,
    position_component: &mut PositionComponent,
    effect_component: &mut EffectComponent,
    stats_component: &mut StatsComponent,
    ability_component: &mut AbilityComponent,
    inventory_component: &mut InventoryComponent,
){
    charactor_component.id = charactor.id;
    charactor_component.charactor_type = charactor.charactor_type.clone();
    charactor_component.race_type = charactor.race_type.clone();
    charactor_component.gender_type = charactor.gender_type.clone();
    charactor_component.status = charactor.status.clone();
    //charactor_component.fraction: charactor.fraction.clone();
    charactor_component.level = charactor.level;
    charactor_component.experience = charactor.experience;

    resist_component.resists = charactor.resists.clone();

    skill_component.skills = charactor.skills.clone();
    skill_component.passive_skills = charactor.passive_skills.clone();

    position_component.position = charactor.position.clone();
    position_component.destination_direction = charactor.destination_direction.clone();
    position_component.destination_path = charactor.destination_path.clone();
    position_component.destination_point = charactor.destination_point.clone();

    effect_component.temporary_effect = charactor.temporary_effect.clone();
    effect_component.endless_effect = charactor.endless_effect.clone();

    stats_component.stats = charactor.stats.clone();
    stats_component.stats_cache = charactor.stats_cache.clone();

    ability_component.ability = charactor.ability.clone();

    inventory_component.stuff_storage = charactor.stuff_storage.clone();
    inventory_component.stuff_wear = charactor.stuff_wear.clone();
    inventory_component.stuff_storage_max_slots = charactor.stuff_storage_max_slots;
}