use bevy::prelude::*;
use bevy_aseprite::*;
use resources::EnemyResources;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod world_loading;
mod resources;
mod sprites {
    use bevy_aseprite::aseprite;
    aseprite!(pub SlowEnemy, "Leafbug.aseprite");
}

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
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(world_loading::WorldLoadingPlugin)
        .add_plugin(AsepritePlugin)
        .register_type::<world_loading::Spawner>()
        .add_startup_system(setup)
        .add_system(spawn)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Load Level
    let world_id = commands.spawn(world_loading::LdtkWorldBundle {
        ldtk_handle: asset_server.load("TowerDefence.ldtk"),
        transform: Transform::from_xyz(-HALF_WIDTH_EXTENT, -HALF_HEIGHT_EXTENT, 0.0),
        ..Default::default()
    }).id();

    let slow_enemy_asset: Handle<Aseprite> = asset_server.load(sprites::SlowEnemy::PATH);
    commands.insert_resource(EnemyResources { world_id, slow_enemy_asset });
}

fn spawn(
    mut commands: Commands,
    time: Res<Time>,
    enemy_assets: Res<EnemyResources>,
    mut spawners: Query<(&Transform, &mut world_loading::Spawner)>,
) {
    for (transform, mut spawner) in spawners.iter_mut() {
        spawner.spawn_timer.tick(time.delta());

        if spawner.spawn_timer.just_finished() {
            let new_id = commands.spawn(AsepriteBundle {
                aseprite: enemy_assets.slow_enemy_asset.clone(),
                animation: anim::AsepriteAnimation::from(sprites::SlowEnemy::tags::MOVE),
                transform: Transform::from_xyz(transform.translation.x, transform.translation.y, 5.0),
                ..default()
            }).id();
            commands.entity(enemy_assets.world_id).push_children(&[new_id]);
        }
    }
}
