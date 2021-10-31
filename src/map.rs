use crate::prelude::*;
use image::Rgb;

const MAP1: &[u8] = include_bytes!("../assets/map1.png");

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system());
    }
}

pub const WALL_SCALE: f32 = 6.0;

pub fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("tiles-60.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle.clone(),
        Vec2::new(physics::PHYSICS_SCALE, physics::PHYSICS_SCALE) * WALL_SCALE,
        4,
        1,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let _material_handle = materials.add(ColorMaterial::texture(texture_handle.clone()));

    let _type_empty: Rgb<u8> = Rgb::from([255, 255, 255]);
    let type_wall: Rgb<u8> = Rgb::from([34, 32, 52]);
    let type_spawner: Rgb<u8> = Rgb::from([172, 50, 50]);
    let type_end: Rgb<u8> = Rgb::from([215, 123, 186]);

    let img = image::load_from_memory(MAP1).unwrap().into_rgb8();

    let dim = img.dimensions();
    let dim = (dim.0 as i32, dim.1 as i32);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite {
                index: 1,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Transform::from_scale(Vec3::new(50.0, 50.0, 1.0)));

    for x in 0..dim.0 {
        for y in 0..dim.1 {
            let pixel = *img.get_pixel(x as u32, y as u32);
            let pos = Vec2::new(x as f32, (dim.1 - y - 1) as f32);

            let is_valid = |at: IVec2| at.x >= 0 && at.x < dim.0 && at.y >= 0 && at.y < dim.1;

            let neighbours = [
                IVec2::new(x - 1, y),
                IVec2::new(x + 1, y),
                IVec2::new(x, y - 1),
                IVec2::new(x, y + 1),
            ];

            let sprite_transform = Transform::from_xyz(
                pos.x as f32 * physics::PHYSICS_SCALE * WALL_SCALE,
                pos.y as f32 * physics::PHYSICS_SCALE * WALL_SCALE,
                0.0,
            );

            let physics_pos = pos * WALL_SCALE;

            if pixel == type_wall {
                let mut wall_entity = commands.spawn_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    ..Default::default()
                });

                if !neighbours.iter().all(|pos| {
                    is_valid(*pos) && *img.get_pixel(pos.x as u32, pos.y as u32) == type_wall
                }) {
                    wall_entity
                        .insert_bundle(ColliderBundle {
                            flags: ColliderFlags {
                                collision_groups: InteractionGroups::new(0b0011, 0b0011),
                                ..Default::default()
                            },
                            shape: ColliderShape::cuboid(0.5 * WALL_SCALE, 0.5 * WALL_SCALE),
                            position: ColliderPosition(physics_pos.into()),
                            ..Default::default()
                        })
                        .insert(ColliderPositionSync::Discrete);
                // .insert(ColliderDebugRender::with_id((x + 16 * y + 10) as usize));
                } else {
                    wall_entity.insert(sprite_transform);
                }
            } else if pixel == type_end {
                let mut escape_entity = commands.spawn_bundle(ColliderBundle {
                    flags: ColliderFlags {
                        collision_groups: InteractionGroups::new(0b0101, 0b0010),
                        active_events: ActiveEvents::INTERSECTION_EVENTS,
                        ..Default::default()
                    },

                    position: ColliderPosition(physics_pos.into()),
                    shape: ColliderShape::ball(physics::PHYSICS_SCALE / 2.0),
                    collider_type: ColliderType::Sensor,
                    ..Default::default()
                });

                escape_entity
                    .insert(ColliderDebugRender::with_id(
                        escape_entity.id().to_bits() as usize
                    ))
                    .insert(mobs::Escape)
                    .insert(ColliderPositionSync::Discrete);
            } else if pixel == type_spawner {
                commands
                    .spawn()
                    .insert(mobs::Spawner)
                    .insert(Timer::from_seconds(0.1, true))
                    .insert(Transform::from_xyz(physics_pos.x, physics_pos.y, 0.0));
            }
        }
    }
}
