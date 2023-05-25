use bevy::prelude::*;

use crate::components::charactor_component::{CharactorComponent, NPCComponent, PlayerComponent, MonsterComponent};
use crate::resources::scene_data::objects::body_part::HealthPoints;
use crate::resources::scene_manager::SceneManager;
use crate::resources::scene_data::objects::body_part::BodyPartType;

use super::CharactorStatus;

pub fn killed_charactor_monster_handler(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut charactor_query: Query<(
        &mut CharactorComponent, 
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    ), With<MonsterComponent>>
){
    //TODO: change sprite to dead, run timer to despawn creature;
    for (mut component, mut sprite, texture_atlas_handle) in charactor_query.iter_mut(){
        if component.status == CharactorStatus::Dead {
            continue;
        };

        match component.body_structure.get(&BodyPartType::Brain){
            Some(v) => {
                match v.health_points.get(&HealthPoints::Current) {
                    Some(t) => {
                        if *t <= 0 {
                            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                            //sprite.index = 9;
                            component.status = CharactorStatus::Dead;
                            //do death function;
                        }
                    },
                    None => {},
                }
            },
            None => {},
        };

        match component.body_structure.get(&BodyPartType::Heart){
            Some(v) => {
                match v.health_points.get(&HealthPoints::Current) {
                    Some(t) => {
                        if *t <= 0 {
                            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                            //sprite.index = 9;
                            component.status = CharactorStatus::Dead;
                            //do death function;
                        }
                    },
                    None => {},
                }
            },
            None => {},
        };

    }
}

pub fn killed_charactor_npc_handler(
    mut commands: Commands,
    mut scene_manager: ResMut<SceneManager>,
    mut charactor_query: Query<(Entity, &CharactorComponent), With<NPCComponent>>
){

}

pub fn killed_charactor_player_handler(
    mut charactor_query: Query<(Entity, &CharactorComponent), With<PlayerComponent>>
){

}