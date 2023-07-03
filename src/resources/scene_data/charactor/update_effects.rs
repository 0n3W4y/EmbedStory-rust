use bevy::prelude::*;

use crate::components::charactor_component::{EffectComponent, StatsComponent, ExtraStatsComponent, ResistsComponent, AbilityComponent, CharactorComponent};

pub fn update_effects( 
    mut charactors_query: Query<(&mut EffectComponent, &mut StatsComponent, &mut ExtraStatsComponent, &mut ResistsComponent, &mut AbilityComponent), With<CharactorComponent>>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();
}