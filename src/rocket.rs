use bevy::prelude::*;

use crate::gamedata::GameData;

pub struct Rocket;

pub struct RocketSpecs {
    pub steering_speed: f32,
    pub max_x_velocity: f32,
    pub max_steering_angle: f32,
}

pub struct RocketPlugin;

impl Plugin for RocketPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(RocketSpecs {
            max_x_velocity: 18.0,
            steering_speed: 1.0,
            max_steering_angle: 2.0 * 30.0 / 180.0,
        })
        .add_startup_system(spawn_rocket.system())
        .add_system(steer_rocket.system());
    }
}

fn steer_rocket(
    time: Res<Time>,
    game_data: Res<GameData>,
    keyboard_input: Res<Input<KeyCode>>,
    rocket_specs: Res<RocketSpecs>,
    _rocket: &Rocket,
    mut transform: Mut<Transform>,
) {
    match game_data.game_state {
        crate::gamestate::GameState::Menu => return,
        crate::gamestate::GameState::Playing => {}
        crate::gamestate::GameState::Dead => return,
    }

    let axis_angle = transform.rotation().to_axis_angle();
    let y_rotation = axis_angle.0.y() * axis_angle.1;
    let mut angle = time.delta_seconds * rocket_specs.steering_speed;
    if keyboard_input.pressed(KeyCode::A) {
        if y_rotation >= rocket_specs.max_steering_angle {
            return;
        };
    } else if keyboard_input.pressed(KeyCode::D) {
        if y_rotation <= -rocket_specs.max_steering_angle {
            return;
        };
        angle = -angle;
    } else {
        if y_rotation == 0.0 {
            return;
        }
        if y_rotation.abs() < angle {
            transform.set_rotation(Quat::default());
            return;
        }
        if y_rotation > 0.0 {
            angle = -angle;
        }
    }
    transform.rotate(Quat::from_rotation_y(angle));
}

fn spawn_rocket(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let texture_handle = asset_server
        .load("models/rocket/RocketColor.png");

    let material_handle = materials.add(StandardMaterial {
        albedo_texture: Some(texture_handle),
        ..Default::default()
    });

    commands
        .spawn(PbrComponents {
            mesh: asset_server
                .load("models/rocket/Rocket.gltf#Mesh0/Primitive0"),
            material: material_handle,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .with(Rocket);
}
