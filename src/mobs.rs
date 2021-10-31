use crate::prelude::*;

pub struct MobsPlugin;

pub struct Spawner;
pub struct Mob;
pub struct Escape;

impl Plugin for MobsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(spawners.system())
            .add_system(update.system());
    }
}

pub fn spawners(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut positions: Query<(&Transform, &mut Timer), With<Spawner>>,
) {
    let texture_handle = asset_server.load("tiles2.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(physics::PHYSICS_SCALE, physics::PHYSICS_SCALE),
        6,
        1,
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for (pos, mut timer) in positions.iter_mut() {
        if timer.tick(time.delta()).just_finished() {
            const PER_FRAME: u32 = 10;

            for _ in 0..PER_FRAME {
                spawn_mob_at(&mut commands, pos.translation, texture_atlas_handle.clone());
            }
        }
    }
}

pub fn update(
    time: Res<Time>,
    mut positions: Query<(&mut RigidBodyVelocity, &mut Timer), With<Mob>>,
) {
    for (mut vel, mut timer) in positions.iter_mut() {
        if timer.tick(time.delta()).just_finished() {
            vel.linvel = vel.linvel.normalize() * (20.0 + random::<f32>() * 10.0);
        }
    }
}

pub fn spawn_mob_at(commands: &mut Commands, pos: Vec3, texture_atlas: Handle<TextureAtlas>) {
    commands
        .spawn()
        .insert(Mob)
        .insert_bundle(SpriteSheetBundle {
            texture_atlas,
            sprite: TextureAtlasSprite {
                index: 0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            ccd: RigidBodyCcd {
                ccd_enabled: true,
                ..Default::default()
            },
            body_type: RigidBodyType::Dynamic,
            position: pos.into(),
            velocity: RigidBodyVelocity {
                linvel: Vec2::new(random::<f32>() * 20.0 - 10.0, 0.0).into(),
                angvel: 0.0,
            },
            forces: RigidBodyForces {
                gravity_scale: 5.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            flags: ColliderFlags {
                collision_groups: InteractionGroups::new(0b0010, 0b0001),
                ..Default::default()
            },
            // shape: ColliderShape::ball(0.5),
            shape: ColliderShape::cuboid(0.5, 0.5),
            material: ColliderMaterial {
                restitution: 0.1,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        // .insert(ColliderDebugRender::with_id(1))
        .insert(Timer::from_seconds(0.5, true));
}
