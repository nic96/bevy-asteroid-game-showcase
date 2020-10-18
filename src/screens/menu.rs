use bevy::prelude::*;

use super::{Menu, assets::ScreenAssets};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, screen_assets: Res<ScreenAssets>) {
    commands.spawn(NodeComponents {
        style: Style {
            margin: Rect::all(Val::Auto),
            ..Default::default()
        },
        material: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.4).into()),
        ..Default::default()
    }).with(Menu).with_children(|parent| {
        parent.spawn(TextComponents {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            text: Text {
                value: "Press space to begin".into(),
                font: screen_assets.default_font,
                style: TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            },
            ..Default::default()
        }).with(Menu);
    });
}
