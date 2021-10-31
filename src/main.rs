use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

use crate::prelude::*;

pub mod camera;
pub mod map;
pub mod mobs;
pub mod physics;
pub mod prelude;
pub mod ui;
pub mod utils;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            width: 1920.0,
            height: 1080.0,
            title: "Rapier Crash".to_string(),
            ..Default::default()
        })
        // .insert_resource(bevy::ecs::schedule::ReportExecutionOrderAmbiguities)
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(camera::CameraPlugin)
        .add_plugin(map::MapPlugin)
        .add_plugin(mobs::MobsPlugin)
        .add_plugin(physics::PhysicsPlugin)
        .add_plugin(utils::UtilsPlugin)
        .add_plugin(ui::UiPlugin)
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}
