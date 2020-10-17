use bevy::prelude::*;

use crate::{gamedata::GameData, rocket::RocketSpecs};

const ASTEROID_TEXTURES: &[&str] = &[
    "assets/models/asteroids/asteroid1/Asteroid1Color.png",
    "assets/models/asteroids/asteroid2/Asteroid2Color.png",
    "assets/models/asteroids/asteroid3/Asteroid3Color.png",
];
const ASTEROID_MESHES: &[&str] = &[
    "assets/models/asteroids/asteroid1/Asteroid1.glb",
    "assets/models/asteroids/asteroid2/Asteroid2.glb",
    "assets/models/asteroids/asteroid3/Asteroid3.glb",
];

struct AsteroidSpawner {
    z_spawn_position: f32, // the z spawn position of asteroids, basically how far ahead of rocket asteroids will spawn
    last_z_position: f32,  // the z position of the last spawned asteroid
    z_interval: f32, // at what distance from last spawned asteroid can a new asteroid be spawned
    z_rand: f32,     // random z offset +/- of z_interval
    min_x_spacing: f32, // the closest two asteroids can be together on the x axis
    z_velocity: f32, // the speed the asteroids come at you
    x_velocity: f32, // the asteroids move left or right when you turn
    x_translation: f32, // the current x position for all asteroids
    max_x: f32,      // the furthest the asteroids can move left or right relative to the rocket
    distance_traveled: f32,
    material_handles: Vec<Handle<StandardMaterial>>,
    mesh_handles: Vec<Handle<Mesh>>,
}

struct AsteroidGroup;
pub struct AsteroidsPlugin;

impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(AsteroidSpawner {
            z_spawn_position: -300.0,
            last_z_position: 0.0,
            z_interval: 24.0,
            z_rand: 15.0,
            min_x_spacing: 2.0,
            z_velocity: 60.0,
            x_velocity: 0.0,
            max_x: 12.0,
            distance_traveled: 0.0,
            x_translation: 0.0,
            material_handles: vec![],
            mesh_handles: vec![],
        })
        .add_startup_system(load_assets.system())
        .add_system(spawn_asteroids.system())
        .add_system(despawn_asteroids.system())
        .add_system(asteroid_spawner_movement.system())
        .add_system(asteroid_movement.system());
    }
}

fn load_assets(
    asset_server: Res<AssetServer>,
    mut asteroid_spawner: ResMut<AsteroidSpawner>,
    mut textures: ResMut<Assets<Texture>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for path in ASTEROID_TEXTURES {
        let asteroid_texture_handle = asset_server.load_sync(&mut textures, path).unwrap();

        let asteroid_material_handle = materials.add(StandardMaterial {
            albedo: Color::rgb(1.0, 1.0, 1.0),
            albedo_texture: Some(asteroid_texture_handle),
            ..Default::default()
        });
        asteroid_spawner
            .material_handles
            .push(asteroid_material_handle);
    }

    for path in ASTEROID_MESHES {
        let mesh_handle = asset_server.load_sync(&mut meshes, path).unwrap();
        asteroid_spawner.mesh_handles.push(mesh_handle);
    }
}

fn spawn_asteroids(
    mut commands: Commands,
    game_data: Res<GameData>,
    mut asteroid_spawner: ResMut<AsteroidSpawner>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    match game_data.game_state {
        crate::gamestate::GameState::Menu => return,
        crate::gamestate::GameState::Playing => {}
        crate::gamestate::GameState::Dead => return,
    }
    if asteroid_spawner.last_z_position
        < asteroid_spawner.z_spawn_position + asteroid_spawner.z_interval
    {
        return;
    }

    let mut seed = (asteroid_spawner.distance_traveled * 100.0).round() as u64;
    fastrand::seed(seed);

    let asteroid_count = fastrand::i32(0..3);

    commands
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 0.0 })),
            material: materials.add(StandardMaterial {
                albedo: Color::rgba(1.0, 1.0, 1.0, 0.0),
                shaded: false,
                ..Default::default()
            }),
            transform: Transform::from_translation(Vec3::new(
                asteroid_spawner.x_translation,
                0.0,
                asteroid_spawner.z_spawn_position,
            )),
            ..Default::default()
        })
        .with(AsteroidGroup)
        .with_children(|parent| {
            let mut x_positions: Vec<f32> = vec![];
            for _ in 0..asteroid_count {
                seed += 1;
                fastrand::seed(seed);
                let asteroid = fastrand::usize(0..2);
                let z = fastrand::f32() * asteroid_spawner.z_rand;
                let mut x =
                    fastrand::f32() * asteroid_spawner.max_x * 2.0 - asteroid_spawner.max_x;
                for x1 in &x_positions {
                    while (x1 - x).abs() < asteroid_spawner.min_x_spacing {
                        seed += 1;
                        fastrand::seed(seed);
                        x = fastrand::f32() * asteroid_spawner.max_x * 2.0
                            - asteroid_spawner.max_x;
                    }
                }
                x_positions.push(x);

                parent.spawn(PbrComponents {
                    mesh: asteroid_spawner.mesh_handles[asteroid],
                    material: asteroid_spawner.material_handles[asteroid],
                    transform: Transform::from_translation_rotation_scale(
                        Vec3::new(x, 1.0, z),
                        Quat::from_axis_angle(
                            Vec3::new(fastrand::f32(), fastrand::f32(), 0.0).normalize(),
                            fastrand::f32() * 3.14,
                        ),
                        fastrand::f32() * 0.25 + 0.75,
                    ),
                    ..Default::default()
                });
            }

            // border asteroids
            let border_spacing = 4.0;
            let border_asteroids_count = (asteroid_spawner.z_interval / border_spacing) as usize;
            for z in 0..border_asteroids_count {
                parent.spawn(PbrComponents {
                    mesh: asteroid_spawner.mesh_handles[border_asteroids_count % 3],
                    material: asteroid_spawner.material_handles[border_asteroids_count % 3],
                    transform: Transform::from_translation_rotation_scale(
                        Vec3::new(-asteroid_spawner.max_x, 1.0, z as f32 * border_spacing),
                        Quat::from_axis_angle(
                            Vec3::new(fastrand::f32(), fastrand::f32(), 0.0).normalize(),
                            fastrand::f32() * 3.14,
                        ),
                        fastrand::f32() * 0.25 + 0.75,
                    ),
                    ..Default::default()
                }).spawn(PbrComponents {
                    mesh: asteroid_spawner.mesh_handles[border_asteroids_count % 3],
                    material: asteroid_spawner.material_handles[border_asteroids_count % 3],
                    transform: Transform::from_translation_rotation_scale(
                        Vec3::new(asteroid_spawner.max_x, 1.0, z as f32 * border_spacing),
                        Quat::from_axis_angle(
                            Vec3::new(fastrand::f32(), fastrand::f32(), 0.0).normalize(),
                            fastrand::f32() * 3.14,
                        ),
                        fastrand::f32() * 0.25 + 0.75,
                    ),
                    ..Default::default()
                });
            }
        });

    asteroid_spawner.last_z_position = asteroid_spawner.z_spawn_position; // last_z_position needs to be reduced in asteroid_spawner_movement
}

// moves the spawner left and right based on keyboard input
fn asteroid_spawner_movement(
    time: Res<Time>,
    game_data: Res<GameData>,
    mut asteroid_spawner: ResMut<AsteroidSpawner>,
    keyboard_input: Res<Input<KeyCode>>,
    rocket_specs: Res<RocketSpecs>,
) {
    match game_data.game_state {
        crate::gamestate::GameState::Menu => return,
        crate::gamestate::GameState::Playing => {}
        crate::gamestate::GameState::Dead => return,
    }

    // calculate angle rocket will turn and accelerate accordingly
    let angle = time.delta_seconds * rocket_specs.steering_speed;
    let percent_turned = angle / rocket_specs.max_steering_angle;
    let mut x_velocity_change = percent_turned * rocket_specs.max_x_velocity;
    if keyboard_input.pressed(KeyCode::A) {
        if asteroid_spawner.x_translation > asteroid_spawner.max_x {
            x_velocity_change = 0.0;
        }
    } else if keyboard_input.pressed(KeyCode::D) {
        if asteroid_spawner.x_translation < -asteroid_spawner.max_x {
            x_velocity_change = 0.0;
        } else {
            x_velocity_change = -x_velocity_change;
        }
    } else {
        // don't translate if x_velocity is close enough to zero;
        // otherwise slow down velocity
        if x_velocity_change.abs() > asteroid_spawner.x_velocity.abs() {
            asteroid_spawner.x_velocity = 0.0;
            x_velocity_change = 0.0;
        }

        if asteroid_spawner.x_velocity < 0.0 {
            x_velocity_change = x_velocity_change.abs();
        } else if asteroid_spawner.x_velocity > 0.0 {
            x_velocity_change = -x_velocity_change.abs();
        }
    }

    asteroid_spawner.x_velocity += x_velocity_change;

    asteroid_spawner.x_velocity = clamp(
        asteroid_spawner.x_velocity,
        -rocket_specs.max_x_velocity,
        rocket_specs.max_x_velocity,
    );

    asteroid_spawner.x_translation = clamp(
        asteroid_spawner.x_translation,
        -asteroid_spawner.max_x,
        asteroid_spawner.max_x,
    );

    asteroid_spawner.x_translation += asteroid_spawner.x_velocity * time.delta_seconds;
    asteroid_spawner.last_z_position += asteroid_spawner.z_velocity * time.delta_seconds;
    asteroid_spawner.distance_traveled += asteroid_spawner.z_velocity * time.delta_seconds;
}

fn asteroid_movement(
    time: Res<Time>,
    asteroid_spawner: Res<AsteroidSpawner>,
    _asteroid_group: Mut<AsteroidGroup>,
    mut transform: Mut<Transform>,
) {
    transform.translate(Vec3::new(
        0.0,
        0.0,
        asteroid_spawner.z_velocity * time.delta_seconds,
    ));
    let trans = transform.translation().clone();
    transform.set_translation(Vec3::new(asteroid_spawner.x_translation, 0.0, trans.z()));
}

fn despawn_asteroids(
    mut commands: Commands,
    mut entity_query: Query<Entity>,
    asteroid_group_query: Query<(&AsteroidGroup, &Transform)>,
) {
    for entity in &mut entity_query.iter() {
        if let Ok(_asteroid_group) = asteroid_group_query.get::<AsteroidGroup>(entity) {
            // the current entity has an asteroid group component
            if let Ok(transform) = asteroid_group_query.get::<Transform>(entity) {
                // the current entity has a transform component
                if transform.translation().z() > 0.0 {
                    commands.despawn_recursive(entity);
                }
            }
        }
    }
}

fn clamp(input: f32, min: f32, max: f32) -> f32 {
    if input < min {
        min
    } else if input > max {
        max
    } else {
        input
    }
}
