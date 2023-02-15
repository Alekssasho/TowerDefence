use bevy::prelude::*;
pub use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::utils::ldtk_pixel_coords_to_translation_pivoted;

pub struct WorldLoadingPlugin;

impl Plugin for WorldLoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(LdtkPlugin)
            .insert_resource(LevelSelection::Index(0))
            .register_ldtk_entity::<SpawnerBundle>("Spawner")
            .register_ldtk_entity::<TowerSlotBundle>("Tower_Slot");
    }
}

#[derive(Default, Clone, Reflect, FromReflect)]
#[reflect_value()]
pub enum SpawnType {
    #[default]
    Slow,
    Fast,
}

#[derive(Component, Clone, Default, Reflect, FromReflect)]
#[reflect(Component)]
pub struct Spawner {
    pub patrol: Vec<Vec2>,
    pub spawn_type: SpawnType,
    pub spawn_timer: Timer,
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
                _ => panic!("Unknown spawn type"),
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