use bevy::prelude::*;

use crate::components::{charactor_component::{CharactorTargetComponent, CharactorComponent}, IdentificationComponent, PositionComponent, ObjectType};

pub fn update_target_position(
    mut source_query: Query<&mut CharactorTargetComponent, With<CharactorComponent>>,
    target_query: Query<(&IdentificationComponent, &PositionComponent), With<CharactorComponent>>,
){
    for mut source_target in source_query.iter_mut() {
        match source_target.target {
            Some(target_id) => {
                for (target_identification, target_position) in target_query.iter() {
                    match target_identification.object_type {
                        ObjectType::Charactor(v) => {
                            if v == target_id {
                                source_target.target_position = Some(target_position.position.clone());
                            }
                        },
                        _ => continue,
                    };
                }
            },
            None => continue,
        }      
    }
}