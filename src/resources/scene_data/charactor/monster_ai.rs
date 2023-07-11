use bevy::prelude::*;

pub fn monster_ai(
    time: Res<Time>,
    monsters_queue: Query<(&CharactorComponent, &PositionComponent), With<MonsterComponent>>,
    player_queue: Query<>,
    companion_queue: Query<>,
){}