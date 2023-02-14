use bevy::prelude::*;
use bevy_aseprite::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::utils::ldtk_pixel_coords_to_translation_pivoted;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

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
        .add_plugin(LdtkPlugin)
        .add_plugin(AsepritePlugin)
        .register_type::<Spawner>()
        .add_startup_system(setup)
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_entity::<SpawnerBundle>("Spawner")
        .register_ldtk_entity::<TowerSlotBundle>("Tower_Slot")
        .add_system(spawn)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Load Level
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("TowerDefence.ldtk"),
        transform: Transform::from_xyz(-HALF_WIDTH_EXTENT, -HALF_HEIGHT_EXTENT, 0.0),
        ..Default::default()
    });

    let slow_enemy_handle: Handle<Aseprite> = asset_server.load(sprites::SlowEnemy::PATH);
    commands.insert_resource(slow_enemy_handle); // TODO:
}

fn spawn(mut commands: Commands, time: Res<Time>, mut spawners: Query<(&Transform, &mut Spawner)>)
{
    for (transform, mut spawner) in spawners.iter_mut() {
            spawner.spawn_timer.tick(time.delta());

            if spawner.spawn_timer.just_finished() {
                commands.spawn(AsepriteBundle {
                    aseprite: asset_server.load(sprites::SlowEnemy::PATH), // TODO:
                    animation: anim::AsepriteAnimation::from(sprites::SlowEnemy::tags::MOVE),
                    transform: Transform::from_xyz(0., 0., 5.0),
                    ..default()
                });
            }
    }
}

#[derive(Default, Clone, Reflect, FromReflect)]
#[reflect_value()]
enum SpawnType {
    #[default]
    Slow,
    Fast,
}

#[derive(Component, Clone, Default, Reflect, FromReflect)]
#[reflect(Component)]
struct Spawner {
    patrol: Vec<Vec2>,
    spawn_type: SpawnType,
    spawn_timer: Timer,
}


#[derive(Component, Default)]
struct TowerSlot;

#[derive(Bundle, LdtkEntity)]
pub struct SpawnerBundle {
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,

    #[ldtk_entity]
    spawner: Spawner,
}

#[derive(Bundle, LdtkEntity)]
pub struct TowerSlotBundle {
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,

    slot: TowerSlot,
}

impl LdtkEntity for Spawner {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        layer_instance: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        _: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> Self {
        let mut points = Vec::new();
        points.push(ldtk_pixel_coords_to_translation_pivoted(
            entity_instance.px,
            layer_instance.c_hei * layer_instance.grid_size,
            IVec2::new(entity_instance.width, entity_instance.height),
            entity_instance.pivot,
        ));

        let ldtk_patrol = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == *"Patrol")
            .unwrap();
        if let FieldValue::Points(ldtk_points) = &ldtk_patrol.value {
            for ldtk_point in ldtk_points {
                if let Some(ldtk_point) = ldtk_point {
                    let pixel_coords = (ldtk_point.as_vec2() + Vec2::new(0.5, 0.5))
                        * Vec2::splat(layer_instance.grid_size as f32);

                    points.push(ldtk_pixel_coords_to_translation_pivoted(
                        pixel_coords.as_ivec2(),
                        layer_instance.c_hei * layer_instance.grid_size,
                        IVec2::new(entity_instance.width, entity_instance.height),
                        entity_instance.pivot,
                    ));
                }
            }
        }

        let ldtk_spawn_type = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == *"Enemy_Type")
            .unwrap();
        let spawn_type = if let FieldValue::Enum(enum_value) = &ldtk_spawn_type.value {
            match enum_value.as_ref().unwrap().as_ref() {
                "Fast" => SpawnType::Fast,
                "Slow" => SpawnType::Slow,
                _ => panic!("Unknown spawn type")
            }
        } else {
            panic!("Unknown spawn type");
        };

        Spawner {
            patrol: points,
            spawn_type,
            spawn_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        }
    }
}
