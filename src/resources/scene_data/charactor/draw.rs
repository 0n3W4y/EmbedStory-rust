use bevy::prelude::*;

use crate::components::charactor_component::{
    CharactorComponent,
    InventoryComponent,
    PlayerComponent, NPCComponent, MonsterComponent, CompanionComponent, SkillAndEffectComponent
};
use crate::components::{IdentificationComponent, ObjectType, PositionComponent, StatsComponent, TakenDamageComponent};
use crate::config::TILE_SIZE;
use crate::materials::material_manager::MaterialManager;
use crate::resources::scene_data::charactor::{Charactor, CharactorType};
use crate::resources::scene_manager::SceneManager;
use crate::scenes::game_scenes::game_scene::GameScene;

pub const Z_POSITION: f32 = 3.0; // fourth layer;

pub fn draw(
    mut commands: Commands,
    scene_manager: Res<SceneManager>,
    material_manager: Res<MaterialManager>,
) {
    let scene: &GameScene = scene_manager.get_current_game_scene();

    for charactor in scene.charactors.get_all_charactors().iter() {
        let x: f32 = (charactor.position.x * TILE_SIZE as i32) as f32;
        let y: f32 = (charactor.position.y * TILE_SIZE as i32) as f32;
        let charactor_type: &CharactorType = &charactor.charactor_type;
        let charactor_gender = &charactor.gender_type;
        let charactor_racetype = &charactor.race_type;

        let new_z_position = Z_POSITION - y as f32 / 1000.0;
        let transform = Transform::from_xyz(x, y, new_z_position);

        let mut identefication_component: IdentificationComponent = Default::default();
        let mut charactor_component: CharactorComponent = Default::default();
        let mut skill_and_effect_component: SkillAndEffectComponent = Default::default();
        let damage_taken_component: TakenDamageComponent = Default::default();
        let mut position_component: PositionComponent = Default::default();
        let mut stats_component: StatsComponent = Default::default();
        let mut inventory_component: InventoryComponent = Default::default();
        copy_from_charactor_to_component(
            charactor,
            &mut identefication_component,
            &mut charactor_component,
            &mut skill_and_effect_component,
            &mut position_component,
            &mut stats_component,
            &mut inventory_component,
        );

        let texture: Handle<TextureAtlas> = material_manager
            .game_scene
            .charactors
            .get_atlas(charactor_racetype, charactor_gender);

        match *charactor_type {
            CharactorType::Player => {
                commands
                    .spawn((SpriteSheetBundle {
                        texture_atlas: texture,
                        transform,
                        ..default()
                    },
                    identefication_component,
                    charactor_component,
                    skill_and_effect_component,
                    damage_taken_component,
                    position_component,
                    stats_component,
                    inventory_component,
                    PlayerComponent,
                    ))
            },
            CharactorType::NPC => {
                commands
                    .spawn((SpriteSheetBundle {
                        texture_atlas: texture,
                        transform,
                        ..default()
                    },
                    identefication_component,
                    charactor_component,
                    skill_and_effect_component,
                    damage_taken_component,
                    position_component,
                    stats_component,
                    inventory_component,
                    NPCComponent,
                    ))
            },
            CharactorType::Monster => {
                commands
                    .spawn((SpriteSheetBundle {
                        texture_atlas: texture,
                        transform,
                        ..default()
                    },
                    identefication_component,
                    charactor_component,
                    skill_and_effect_component,
                    damage_taken_component,
                    position_component,
                    stats_component,
                    inventory_component,
                    MonsterComponent,
                    ))
            },
            CharactorType::Companion => {
                commands
                    .spawn((SpriteSheetBundle {
                        texture_atlas: texture,
                        transform,
                        ..default()
                    },
                    identefication_component,
                    charactor_component,
                    skill_and_effect_component,
                    damage_taken_component,
                    position_component,
                    stats_component,
                    inventory_component,
                    CompanionComponent,
                    ))
            },
        };
        
    }
}

pub fn copy_from_charactor_to_component(
    charactor: &Charactor,
    identefication_component: &mut IdentificationComponent,
    charactor_component: &mut CharactorComponent,
    skill_and_effect_component: &mut SkillAndEffectComponent,
    position_component: &mut PositionComponent,
    stats_component: &mut StatsComponent,
    inventory_component: &mut InventoryComponent,
) {
    identefication_component.object_type = ObjectType::Charactor(charactor.charactor_type.clone() , charactor.id);

    charactor_component.charactor_type = charactor.charactor_type.clone();
    charactor_component.race_type = charactor.race_type.clone();
    charactor_component.gender_type = charactor.gender_type.clone();
    charactor_component.strength = charactor.strength.clone();
    charactor_component.status = charactor.status.clone();
    //charactor_component.fraction: charactor.fraction.clone();
    charactor_component.level = charactor.level;
    charactor_component.experience = charactor.experience;

    stats_component.resists = charactor.resists.clone();

    skill_and_effect_component.base_skill = charactor.base_skill.clone();
    skill_and_effect_component.active_skills = charactor.active_skills.clone();
    skill_and_effect_component.passive_skills = charactor.passive_skills.clone();

    position_component.position = charactor.position.clone();
    position_component.destination_direction = charactor.destination_direction.clone();
    position_component.destination_path = charactor.destination_path.clone();
    position_component.destination_point = charactor.destination_point.clone();

    skill_and_effect_component.effects = charactor.effects.clone();
    skill_and_effect_component.effect_immunes = charactor.effects_immunes.clone();

    stats_component.stats = charactor.stats.clone();
    stats_component.stats_cache = charactor.stats_cache.clone();

    stats_component.attributes = charactor.attributes.clone();
    stats_component.attributes_cache = charactor.attributes.clone();

    stats_component.ability = charactor.ability.clone();

    inventory_component.stuff_storage = charactor.stuff_storage.clone();
    inventory_component.stuff_wear = charactor.stuff_wear.clone();
    inventory_component.stuff_storage_max_slots = charactor.stuff_storage_max_slots;
}
