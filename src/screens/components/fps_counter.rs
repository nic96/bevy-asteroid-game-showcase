use bevy::{diagnostic::Diagnostics, diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

use crate::screens::assets::ScreenAssets;

pub struct FpsCounterPlugin;
struct FpsCounter;

impl Plugin for FpsCounterPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FrameTimeDiagnosticsPlugin)
        .add_startup_system(setup.system())
        .add_system(fps_counter.system());
    }
}


fn setup(mut commands: Commands, screen_assets: Res<ScreenAssets>) {
    commands.spawn(TextComponents {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(10.0),
                    right: Val::Px(10.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                value: "- fps".into(),
                font: screen_assets.default_font,
                style: TextStyle {
                    font_size: 18.0,
                    color: Color::WHITE,
                },
            },
            ..Default::default()
    }).with(FpsCounter);
}

fn fps_counter(diagnostics: Res<Diagnostics>, mut text: Mut<Text>, _fps_counter: &FpsCounter) {
    if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps) = fps_diagnostic.value() {
            text.value = format!("{:.1} fps", fps);
        }
    }
}
