use std::time::Duration;

use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::prelude::Res;
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    ecs::{component::Component, entity::Entity, query::With, system::Query},
    time::common_conditions::on_timer,
};
use bevy_ui::widget::TextUiWriter;

/// For print Gpu, Cpu usage
#[derive(Component)]
pub struct FpsFrametimeDebugTextWriter;

impl FpsFrametimeDebugTextWriter {
    /// Print Gpu and Cpu usage
    pub fn print_cpu_gpu_label_system(
        diagnostic: Res<DiagnosticsStore>,
        mut query: Query<Entity, With<FpsFrametimeDebugTextWriter>>,
        mut writer: TextUiWriter,
    ) {
        let entity = query.single_mut().unwrap();

        let fps = diagnostic
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|d| d.smoothed());
        let frame_ms = diagnostic
            .get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
            .and_then(|d| d.smoothed());

        if let (Some(fps), Some(ms)) = (fps, frame_ms) {
            // Try Add an delay/sleep for the pause game and show details
            *writer.text(entity, 0) = format!("Fps [{:.0}] | FrameTime [{:.2}] ms", fps, ms);
        }
    }

    /// IF 'print_cpu_gpu_label_system' on engine::info_debug::FpsFrametimeDebugTextWriter ONLY DEBUG SYSTEM, PLEASE
    pub fn add_to_system() {
        Self::print_cpu_gpu_label_system
            .run_if(on_timer(
                Duration::from_secs_f32(0.25)
            )
        );
    }
}
