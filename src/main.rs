use bevy::{prelude::*, ecs::entity::EntityLocation};
use bevy_ecs_ldtk::prelude::*;

static HALF_WIDTH_EXTENT: f32 = 1280.0 / 2.0;
static HALF_HEIGHT_EXTENT: f32 = 768.0 / 2.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: HALF_WIDTH_EXTENT * 2.0,
                height: HALF_HEIGHT_EXTENT * 2.0,
                title: "Tower Defence".to_string(),
                ..default()
            },
            ..default()
        }))
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup)
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_entity::<SpawnerBundle>("Spawner")
        .register_ldtk_entity::<TowerSlotBundle>("Tower_Slot")
        .add_system(spawn)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("TowerDefence.ldtk"),
        transform: Transform::from_xyz(-HALF_WIDTH_EXTENT, -HALF_HEIGHT_EXTENT, 0.0),
        ..Default::default()
    });
}

fn spawn(mut commands: Commands, spawners: Query<&Transform, With<TowerSlot>>)
{
    for transform in spawners.iter() {
            //println!("Spawner Position {:?}", transform.translation);
    }
}

#[derive(Component, Default)]
struct Spawner;


#[derive(Component, Default)]
struct TowerSlot;

#[derive(Bundle, LdtkEntity)]
pub struct SpawnerBundle {
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,

    spawner: Spawner,
}

#[derive(Bundle, LdtkEntity)]
pub struct TowerSlotBundle {
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,

    slot: TowerSlot,
}
