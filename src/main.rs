use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_window_title_diagnostics::WindowTitleLoggerDiagnosticsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(WindowTitleLoggerDiagnosticsPlugin {
            // filter: Some(vec![FrameTimeDiagnosticsPlugin::FPS]),
            ..Default::default()
        })
        // .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin::default())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::CYAN,
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..Default::default()
        },
        ..default()
    });
}
