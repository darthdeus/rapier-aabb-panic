use crate::prelude::*;

pub const PHYSICS_SCALE: f32 = 10.0;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(RapierRenderPlugin)
            .insert_resource(RapierConfiguration {
                scale: PHYSICS_SCALE,
                ..Default::default()
            });
    }
}
