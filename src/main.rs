pub mod core;
pub mod engine;

use std::time::Duration;

use bevy::{
    camera::prelude::*,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    image::{
        ImageAddressMode,
        ImageFilterMode,
        ImageSampler,
        ImageSamplerDescriptor
    },
    input::mouse::MouseMotion,
    // post_process::motion_blur::MotionBlur,
    prelude::*,
    time::common_conditions::on_timer,
    window::{CursorGrabMode, CursorOptions},
    animation::{AnimationPlayer, AnimationClip},
    animation::graph::{AnimationNodeIndex},
    ecs::query::Added,
};
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin}, prelude::{Collider, CollisionEvent, LockedAxes, RapierConfiguration, RigidBody, Velocity}, render::RapierDebugRenderPlugin
};

/// Player Component for the moviment
#[derive(Component)]
struct Player;

/// Animation Player
#[derive(Resource)]
struct PlayerAnimationGraph {
    graph: Handle<AnimationGraph>,
    node: AnimationNodeIndex,
}

/// Part of Player Moviments
/// In the case for Player Camera Fps style
#[derive(Component)]
struct CameraPitch {
    #[allow(unused)]
    pitch: f32,
}

/// Sounds for Player
#[derive(Component)]
struct FootstepsTimer(Timer);

/// For print Gpu, Cpu usage
#[derive(Component)]
struct FpsText;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (
            draw_cursor,
            player_movement,
            mouse_look,
            // IF 'print_cpu_gpu_label_system' ONLY DEBUG SYSTEM, PLEASE
            print_cpu_gpu_label_system
                .run_if(on_timer(
                    Duration::from_secs_f32(0.25)
                )
            ),
            player_animation,
            player_collision_damage,
        ))
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // For Animation Players params
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
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
    let mut plane: Mesh = Plane3d::default().into();
    let uv_size = 4000.0;
    let uvs = vec![[uv_size, 0.0], [0.0, 0.0], [0.0, uv_size], [uv_size; 2]];
    plane.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    commands.spawn((
        Mesh3d(meshes.add(plane)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 1.0,
            base_color_texture: Some(images.add(uv_debug_texture())),
            ..default()
        })),
        Transform::from_xyz(0.0, -0.65, 0.0).with_scale(Vec3::splat(80.)),
    ));
    // Spawn Gpu and Cpu Text Label
    commands.spawn((
        Text::new("USAGE SYSTEM"),
        Node {
            position_type: PositionType::Absolute,
            top: px(5),
            left: px(5),
            ..Default::default()
        },
        FpsText,
    ));
    // Setup Animations from Player
    let clip: Handle<AnimationClip> =
        asset_server.load("models/helena/helena.glb#Animation0");
    let (graph, node) = AnimationGraph::from_clip(clip);
    let graph_handle = graphs.add(graph);
    commands.insert_resource(PlayerAnimationGraph {
        graph: graph_handle,
        node,
    });
    // Physics from global universe
    commands.spawn(RapierConfiguration {
        force_update_from_transform_changes: false,
        gravity: Vec3::new(0.0, -9.81, 0.0),
        physics_pipeline_active: true,
        scaled_shape_subdivision: 10
    });
    // Physics from (Wall, Ground, etc...)
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(50.0, 1.0, 50.0),
        Transform::from_xyz(0.0, -1.0, 0.0),
    ));

    // TEST: Spawn Simple Mesh with animation
    commands.spawn((
        SceneRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/helena/helena.glb#Scene0"))
        ),
        Transform::from_xyz(0.0, 0.0, 0.0)
            .looking_at(Vec3::ZERO, Vec3::Y)
            .with_scale(Vec3 { x: 0.01, y: 0.01, z: 0.01 }),
    ));
    // TEST: Boxes for test the Physics
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            unlit: true,
            base_color: Color::linear_rgb(0.1, 0.6, 1.0),
            ..default()
        })),
    ));

    // Spawn Player
    spawn_player(commands);
}

/// Spawn Player with seguiments of commands.spawn
/// It uses in the function 'setup'
fn spawn_player(
    mut commands: Commands,
) {
    commands
        .spawn((
            Player,
            RigidBody::Dynamic,
            Collider::capsule_y(0.9, 0.4),
            Velocity::default(),
            LockedAxes::ROTATION_LOCKED,
            Transform::from_xyz(0.0, 3.0, 5.0),
            GlobalTransform::default(),
            FootstepsTimer(Timer::from_seconds(0.45, TimerMode::Repeating)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Camera3d::default(),
                // MotionBlur {
                //     shutter_angle: 0.5,
                //     samples: 0,
                // },
                Transform::from_xyz(0.0, 0.6, 0.0),
                CameraPitch { pitch: 0.0 },
            ));
        }
    );
}

/// Player damage for collision
fn player_collision_damage(
    mut collision_events: MessageReader<CollisionEvent>,
    velocities: Query<&Velocity>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            if let (Ok(v1), Ok(v2)) = (velocities.get(*e1), velocities.get(*e2)) {
                let impact = (v1.linvel - v2.linvel).length();
                if impact >= 8.0 {
                    // Take Hit On Damage here
                    // Damage for Impact
                    println!("You got the Hit for Impact");
                }
            }
        }
    }
}

/// Player Animation abstractor
fn player_animation(
    mut commands: Commands,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
    graph: Res<PlayerAnimationGraph>,
) {
    for (entity, mut player) in &mut players {
        commands.entity(entity).insert(AnimationGraphHandle(graph.graph.clone()));
        player.play(graph.node).repeat();
    }
}

/// Mouse Look with angle of 90 degree
fn mouse_look(
    mut mouse_motion: MessageReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut transform = query.single_mut().unwrap();
    let mut delta = Vec2::ZERO;

    for motion in mouse_motion.read() {
        delta += motion.delta;
    }

    let sensitivity = 0.002;
    let yaw = -delta.x * sensitivity;
    let pitch = -delta.y * sensitivity;
    let (mut yaw_rot, mut pitch_rot, _) = transform.rotation.to_euler(EulerRot::YXZ);

    yaw_rot += yaw;
    pitch_rot += pitch;

    // limit to 90 degree angle
    pitch_rot = pitch_rot.clamp(-1.54, 1.54);

    transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw_rot, pitch_rot, 0.0);
}

/// Player moviments with Keyboard Commands ('W,A,S,D')
fn player_movement(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    mut query_sounds: Query<&mut FootstepsTimer, With<Player>>,
    time: Res<Time>,
) {
    let mut transform = query.single_mut().unwrap();
    let mut direction = Vec3::ZERO;

    let mut timer_tick = query_sounds.single_mut().unwrap();

    if keyboard.pressed(KeyCode::KeyW) {
        direction += *transform.forward();

        timer_tick.0.tick(time.delta());
        if timer_tick.0.just_finished() {
            commands.spawn(AudioPlayer::new(
                asset_server.load("sounds/steps/data_pion-st3-footstep-sfx-323056.ogg")
            ));
        }
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction -= *transform.forward();

        timer_tick.0.tick(time.delta());
        if timer_tick.0.just_finished() {
            commands.spawn(AudioPlayer::new(
                asset_server.load("sounds/steps/data_pion-st2-footstep-sfx-323055.ogg")
            ));
        }
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction -= *transform.right();

        timer_tick.0.tick(time.delta());
        if timer_tick.0.just_finished() {
            commands.spawn(AudioPlayer::new(
                asset_server.load("sounds/steps/data_pion-st2-footstep-sfx-323055.ogg")
            ));
        }
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction += *transform.right();

        timer_tick.0.tick(time.delta());
        if timer_tick.0.just_finished() {
            commands.spawn(AudioPlayer::new(
                asset_server.load("sounds/steps/data_pion-st1-footstep-sfx-323053.ogg")
            ));
        }
    }

    direction.y = 0.0;
    let speed = 5.0;

    transform.translation += direction.normalize_or_zero() * speed * time.delta_secs();
}

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

fn uv_debug_texture() -> Image {
    use bevy::{asset::RenderAssetUsages, render::render_resource::*};
    const TEXTURE_SIZE: usize = 7;

    let mut palette = [
        164, 164, 164, 255, 168, 168, 168, 255, 153, 153, 153, 255, 139, 139, 139, 255, 153, 153,
        153, 255, 177, 177, 177, 255, 159, 159, 159, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(12);
    }

    let mut img = Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );
    img.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::MirrorRepeat,
        mag_filter: ImageFilterMode::Nearest,
        ..ImageSamplerDescriptor::linear()
    });
    img
}

/// Print Gpu and Cpu usage
fn print_cpu_gpu_label_system(
    diagnostic: Res<DiagnosticsStore>,
    mut query: Query<Entity, With<FpsText>>,
    mut writer: TextUiWriter,
) {
    let entity = query.single_mut().unwrap();

    let fps = diagnostic
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|d| d.smoothed())
    ;
    let frame_ms = diagnostic
        .get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
        .and_then(|d| d.smoothed())
    ;

    if let (Some(fps), Some(ms)) = (fps, frame_ms) {
        // Try Add an delay/sleep for the pause game and show details
        *writer.text(entity, 0) =
            format!("Fps [{:.0}] | FrameTime [{:.2}] ms", fps, ms);
    }
}