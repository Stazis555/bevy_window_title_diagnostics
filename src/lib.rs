use bevy::diagnostic::{Diagnostic, DiagnosticPath, DiagnosticsStore};
use bevy::prelude::*;
use bevy::time::{Time, Timer};
use bevy::window::Window;
use core::time::Duration;
// use bevy::window::Windows;

/// An App Plugin that logs diagnostics to the primary window's title
pub struct WindowTitleLoggerDiagnosticsPlugin {
    pub wait_duration: Duration,
    pub filter: Option<Vec<DiagnosticPath>>,
}

/// State used by the [`WindowTitleLoggerDiagnosticsPlugin`]
#[derive(Resource)]
struct WindowTitleLoggerState {
    timer: Timer,
    filter: Option<Vec<DiagnosticPath>>,
}

impl Default for WindowTitleLoggerDiagnosticsPlugin {
    fn default() -> Self {
        WindowTitleLoggerDiagnosticsPlugin {
            wait_duration: Duration::from_secs(1),
            filter: None,
        }
    }
}

impl Plugin for WindowTitleLoggerDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowTitleLoggerState {
            timer: Timer::new(self.wait_duration, bevy::time::TimerMode::Repeating),
            filter: self.filter.clone(),
        });

        app.add_systems(PostUpdate, Self::log_diagnostics_system);
    }
}

impl WindowTitleLoggerDiagnosticsPlugin {
    pub fn filtered(filter: Vec<DiagnosticPath>) -> Self {
        WindowTitleLoggerDiagnosticsPlugin {
            filter: Some(filter),
            ..Default::default()
        }
    }

    fn format(diagnostic: &Diagnostic) -> String {
        if let Some(value) = diagnostic.value() {
            if let Some(average) = diagnostic.average() {
                return format!(" {}: {:.2} ({:.2}) |", diagnostic.path(), value, average);
            } else {
                return format!(" {}: {:.2} |", diagnostic.path(), value);
            }
        }

        "".to_owned()
    }

    fn log_diagnostics_system(
        mut state: ResMut<WindowTitleLoggerState>,
        time: Res<Time>,
        diagnostics: Res<DiagnosticsStore>,
        mut windows: Query<&mut Window>,
    ) {
        if state.timer.tick(time.delta()).is_finished() {
            let mut title = String::new();

            if let Some(ref filter) = state.filter {
                for diagnostic in filter.iter().map(|path| diagnostics.get(path).unwrap()) {
                    title = title + &Self::format(diagnostic);
                }
            } else {
                for diagnostic in diagnostics.iter() {
                    title = title + &Self::format(diagnostic);
                }
            }

            if let Some(mut window) = windows.iter_mut().next() {
                window.title = title[0..title.len() - 1].to_owned();
            }
        }
    }
}
