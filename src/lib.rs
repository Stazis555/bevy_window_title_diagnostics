use bevy::diagnostic::{Diagnostic, DiagnosticId, Diagnostics};
use bevy::prelude::{App, CoreStage, Plugin, Res, ResMut, Resource};
use bevy::time::{Time, Timer};
use bevy::utils::Duration;
use bevy::window::Windows;

/// An App Plugin that logs diagnostics to the primary window's title
pub struct WindowTitleLoggerDiagnosticsPlugin {
    pub wait_duration: Duration,
    pub filter: Option<Vec<DiagnosticId>>,
}

/// State used by the [`WindowTitleLoggerDiagnosticsPlugin`]
#[derive(Resource)]
struct WindowTitleLoggerState {
    timer: Timer,
    filter: Option<Vec<DiagnosticId>>,
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

        app.add_system_to_stage(CoreStage::PostUpdate, Self::log_diagnostics_system);
    }
}

impl WindowTitleLoggerDiagnosticsPlugin {
    pub fn filtered(filter: Vec<DiagnosticId>) -> Self {
        WindowTitleLoggerDiagnosticsPlugin {
            filter: Some(filter),
            ..Default::default()
        }
    }

    fn format(diagnostic: &Diagnostic) -> String {
        if let Some(value) = diagnostic.value() {
            if let Some(average) = diagnostic.average() {
                return format!(" {}: {:.2} ({:.2}) |", diagnostic.name, value, average);
            } else {
                return format!(" {}: {:.2} |", diagnostic.name, value);
            }
        }

        "".to_owned()
    }

    fn log_diagnostics_system(
        mut state: ResMut<WindowTitleLoggerState>,
        time: Res<Time>,
        diagnostics: Res<Diagnostics>,
        mut windows: ResMut<Windows>,
    ) {
        if state.timer.tick(time.delta()).finished() {
            let mut title = String::new();

            if let Some(ref filter) = state.filter {
                for diagnostic in filter.iter().map(|id| diagnostics.get(*id).unwrap()) {
                    title = title + &Self::format(diagnostic);
                }
            } else {
                for diagnostic in diagnostics.iter() {
                    title = title + &Self::format(diagnostic);
                }
            }

            windows
                .primary_mut()
                .set_title(title[0..title.len() - 1].to_owned());
        }
    }
}
