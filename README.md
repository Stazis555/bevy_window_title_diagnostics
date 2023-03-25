# Bevy window title diagnostics

This crate writes [Bevy](https://github.com/bevyengine/bevy) diagnostics into primary window title.

Made for little bit more convenient usage of the FrameTimeDiagnosticsPlugin

# Install

Add to your `Cargo.toml`

For bevy 0.10:
```rust
bevy_window_title_diagnostics = 0.4

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
    commands.spawn(Camera2dBundle::new_with_far(10.0));
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::CYAN,
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        ..default()
    });
}
```

# Bevy compatibility table
Bevy version | bevy_window_title_diagnostics version
--- | ---
0.8 | 0.2
0.9 | 0.3
0.10 | 0.4