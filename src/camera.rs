use crate::prelude::*;

pub const ZOOM_MAX: f32 = 1.0;

pub struct CameraPlugin;
pub struct MainCamera;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system());
    }
}

pub fn startup(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform.translation.x =
        1920.0 / 2.0 - (physics::PHYSICS_SCALE * map::WALL_SCALE) / 2.0;
    camera.transform.translation.y =
        1080.0 / 2.0 - (physics::PHYSICS_SCALE * map::WALL_SCALE) / 2.0;

    // = Vec3::new(64.0, 64.0, 999.9);
    // camera.transform.translation = Vec3::new(0.0, 0.0, 999.9);
    camera.transform.scale = Vec3::splat(ZOOM_MAX);

    commands.spawn_bundle(camera).insert(MainCamera);
}
