use bevy::prelude::*;
use bevy_aseprite::Aseprite;

#[derive(Resource)]
pub struct EnemyResources {
    pub world_id: Entity,
    pub slow_enemy_asset: Handle<Aseprite>,
}
