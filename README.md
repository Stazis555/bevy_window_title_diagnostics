# Bevy window title diagnostics

This crate writes [Bevy](https://github.com/bevyengine/bevy) diagnostics into primary window title.

Made for little bit more convenient usage of the FrameTimeDiagnosticsPlugin

# Install

Add to your `Cargo.toml`

For bevy 0.15:

```rust
bevy_window_title_diagnostics = 0.15

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
        .add_systems(Startup, setup)
        .add_plugins((
            FrameTimeDiagnosticsPlugin,
            // Insert same way as usual LogDiagnosticsPlugin
            WindowTitleLoggerDiagnosticsPlugin {
                // It is possible to filter Diagnostics same way as default LogDiagnosticsPlugin
                // filter: Some(vec![FrameTimeDiagnosticsPlugin::FPS]),
                ..Default::default()
            },
            // Works with any diagnostics
            // bevy::diagnostic::EntityCountDiagnosticsPlugin::default(),
        ))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    commands.spawn(Sprite {
        color: Color::srgb(0.0, 0.0, 1.0),
        custom_size: Some(Vec2::new(50.0, 50.0)),
        ..default()
    });
}

```

# Bevy compatibility table

| Bevy version | bevy_window_title_diagnostics version |
| ------------ | ------------------------------------- | --- | ---- |
| 0.15         | 0.15                                  |
| 0.14         | 0.14                                  |
| 0.13         | 0.13                                  |
| 0.12         | 0.6                                   |     | 0.12 |
| 0.11         | 0.5                                   |
| 0.10         | 0.4                                   |
| 0.9          | 0.3                                   |
| 0.8          | 0.2                                   |

Starting from bevy 0.12 library will have the same semantic version as bevy
