use bevy::prelude::*;
use bevy_aseprite::*;
use bevy_ecs_ldtk::prelude::*;

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
        .add_plugin(LdtkPlugin)
        .add_plugin(AsepritePlugin)
        .add_startup_system(setup)
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_entity::<SpawnerBundle>("Spawner")
        .register_ldtk_entity::<TowerSlotBundle>("Tower_Slot")
        .register_ldtk_entity::<PatrolRoutePointBundle>("Patrol_Route_Point")
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

    // Load sprite animations
    commands.spawn(AsepriteBundle {
        aseprite: asset_server.load(sprites::SlowEnemy::PATH),
        animation: anim::AsepriteAnimation::from(sprites::SlowEnemy::tags::MOVE),
        transform: Transform::from_xyz(0., 0., 5.0),
        ..default()
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

#[derive(Component)]
pub struct PatrolRoutePoint {
    next: Entity,
}

#[derive(Bundle, LdtkEntity)]
pub struct PatrolRoutePointBundle {
    #[ldtk_entity]
    data: PatrolRoutePoint,
}

impl LdtkEntity for PatrolRoutePoint {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        layer_instance: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        _: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> Self {
        let next_patrol_field = entity_instance.field_instances.iter().find(|f| f.identifier == *"Next_Point").unwrap();
        if let FieldValue::EntityRef(next_patrol) = &next_patrol_field.value {
            if let Some(next) = next_patrol {
                layer_instance.entity_instances
            } else {
                panic!("Patrol route without next point");
            }
        }
        PatrolRoutePoint {  

        }
    }
}
