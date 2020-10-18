use bevy::prelude::*;

use assets::ScreenAssets;

use self::{components::fps_counter::FpsCounterPlugin, menu::MenuPlugin};
pub mod menu;
pub mod components;
pub mod assets;

pub struct ScreensPlugin;
pub struct Menu;


impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
        .add_resource(ScreenAssets {
            default_font: Handle::default(),
        })
        .add_plugin(MenuPlugin)
        .add_plugin(FpsCounterPlugin);
    }
}


pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut screen_assets: ResMut<ScreenAssets>) {
    screen_assets.default_font = asset_server.load("assets/fonts/NotoSans-Regular.ttf").unwrap();
    commands.spawn(UiCameraComponents::default());
}
