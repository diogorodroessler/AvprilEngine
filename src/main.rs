pub mod core;
pub mod engine;

// use std::time::Duration; // For Fps Text Debug

use bevy::{
    // diagnostic::{FrameTimeDiagnosticsPlugin}, // Uses only for FpsFrametimeDebugTextWriter in Debug Text writer in the screen
    prelude::*,
    /* time::common_conditions::on_timer, // For Fps Text Debug */
    window::{CursorGrabMode, CursorOptions},
};
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    prelude::{Collider, RapierConfiguration, RigidBody},
    render::RapierDebugRenderPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                draw_cursor,
                engine::player_controller::PlayerCharacter::player_movement,
                engine::player_controller::PlayerCharacter::mouse_look,
                engine::player_controller::PlayerCharacter::player_animation,
                engine::player_controller::PlayerCharacter::player_collision_damage,
            ),
        )
        .run();
}

/// set up a simple 3D scene
fn setup(
    #[allow(unused_variables, unused_mut)]
    // For Uv Debug
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    graphs: ResMut<Assets<AnimationGraph>>,
) {
    let mut commands = Commands::reborrow(&mut commands);

    //Directional Light
    commands.insert_resource(GlobalAmbientLight {
        color: Color::WHITE,
        brightness: 300.0,
        ..default()
    });
    commands.spawn((
        DirectionalLight {
            illuminance: 3_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::default().looking_to(Vec3::new(-1.0, -0.7, -1.0), Vec3::X),
    ));
    // Sky
    commands.spawn((
        Mesh3d(meshes.add(Sphere::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            unlit: true,
            base_color: Color::linear_rgb(0.1, 0.6, 1.0),
            ..default()
        })),
        Transform::default().with_scale(Vec3::splat(-4000.0)),
    ));
    // Ground
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 1.0,
            ..default()
        })),
        Transform::from_xyz(0.0, -0.65, 0.0).with_scale(Vec3::splat(80.)),
    ));
    // Physics from global universe
    commands.spawn(RapierConfiguration {
        force_update_from_transform_changes: false,
        gravity: Vec3::new(0.0, -9.81, 0.0),
        physics_pipeline_active: true,
        scaled_shape_subdivision: 10,
    });
    // Physics from (Wall, Ground, etc...)
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(50.0, 1.0, 50.0),
        Transform::from_xyz(0.0, -1.0, 0.0),
    ));

    // --------------- TESTS ---------------

    // TEST: Boxes for test the Physics
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            unlit: true,
            base_color: Color::linear_rgb(0.1, 0.6, 1.0),
            ..default()
        })),
        RigidBody::Fixed,
    ));

    // --------------- LATEST Commands Functions ---------------

    // Spawn only player camera follow
    engine::player_controller::PlayerCharacter::spawn_player_camera(&mut commands);

    // Spawn Player Character
    engine::player_controller::PlayerCharacter::player_character(
        &mut commands,
        asset_server,
        graphs,
    );
}

/// Enable the cursor in the Game Screen
fn draw_cursor(
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    mut cursor: Query<&mut CursorOptions>,
) {
    let mut cur_ops = cursor.single_mut().unwrap();

    if key.just_pressed(KeyCode::Escape) {
        cur_ops.visible = true;
        cur_ops.grab_mode = CursorGrabMode::None;
    }

    if mouse.just_pressed(MouseButton::Left) {
        cur_ops.visible = false;
        cur_ops.grab_mode = CursorGrabMode::Locked;
    }
}