use asteroids::AsteroidsPlugin;
use bevy::prelude::*;
use gamedata::{GameData, GameDataPlugin};
use gamestate::{GameState, GameStatePlugin};
use rocket::RocketPlugin;
mod asteroids;
mod gamedata;
mod gamestate;
mod rocket;

// the app entry point. hopefully you recognize it from the examples above!
fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    env_logger::init();

    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    #[cfg(target_arch = "wasm32")]
    console_log::init_with_level(log::Level::Info).expect("cannot initialize console_log");

    App::build()
        .add_resource(WindowDescriptor {
            title: "Ha! a Title".into(),
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.005)))
        // .add_resource(Msaa { samples: 8 })
        .add_default_plugins()
        .add_plugin(GameStatePlugin)
        .add_plugin(RocketPlugin)
        .add_plugin(AsteroidsPlugin)
        .add_plugin(GameDataPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera3dComponents {
            transform: Transform::new(Mat4::face_toward(
                Vec3::new(0.0, 9.0, 20.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            )),
            ..Default::default()
        }) // light
        .spawn(LightComponents {
            light: Light {
                color: Color::rgb(1.0, 1.0, 1.0),
                fov: 180.0,
                depth: (0.0..10000.0),
            },
            transform: Transform::from_translation(Vec3::new(400.0, 400.0, 100.0)),
            ..Default::default()
        });
}
