# Bevy window title diagnostics

This crate writes [Bevy](https://github.com/bevyengine/bevy) diagnostics into primary window title.

Made for little bit more convenient usage of the FrameTimeDiagnosticsPlugin

# Install

Add to your `Cargo.toml`

```rust
bevy_window_title_diagnostics = 0.1
```

# Usage

Just add `WindowTitleLoggerDiagnosticsPlugin` from this crate alongside with default `FrameTimeDiagnosticsPlugin`

```rust
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_window_title_diagnostics::WindowTitleLoggerDiagnosticsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        // Insert same way as usual LogDiagnosticsPlugin
        .add_plugin(WindowTitleLoggerDiagnosticsPlugin {
            // It is possible to filter Diagnostics same way as default LogDiagnosticsPlugin
            // filter: Some(vec![FrameTimeDiagnosticsPlugin::FPS]),
            ..Default::default()
        })
        // Works with any diagnostics
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

```

# Notes
Tested on `bevy=0.7`