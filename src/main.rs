pub mod core;
pub mod engine;

use bevy::{
    camera::prelude::*,
    image::{
        ImageAddressMode,
        ImageFilterMode,
        ImageSampler,
        ImageSamplerDescriptor
    },
    input::mouse::MouseMotion,
    post_process::motion_blur::MotionBlur,
    prelude::*,
    window::{CursorGrabMode, CursorOptions}
};

#[derive(Component)]
struct Player;

#[derive(Component)]
struct CameraPitch {
    #[allow(unused)]
    pitch: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (
            draw_cursor,
            player_movement,
            mouse_look,
        ))
        .run();
}

/// set up a simple 3D scene
fn setup(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
    // Spawn Player sequence of commands.spawn
    spawn_player(asset_server, commands);
}

/// Spawn Player with seguiments of commands.spawn
/// It uses in the function 'setup'
fn spawn_player(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands
        .spawn((
            Player,
            Transform::from_xyz(0.0, 1.0, 5.0),
            GlobalTransform::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                Camera3d::default(),
                MotionBlur {
                    shutter_angle: 1.0,
                    samples: 2,
                },
                Transform::default(),
                CameraPitch { pitch: 0.0 },
            ));
        }
    );
    // Load Big Boss 'Resource Assets'
    commands.spawn((
        SceneRoot(
            asset_server.load("models/bigboss/bb.glb#Scene0")
        ),
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
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
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut transform = query.single_mut().unwrap();
    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        direction += *transform.forward();
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction -= *transform.forward();
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction -= *transform.right();
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction += *transform.right();
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

/// Write Text in the Screen Commands Keyboard
// fn keyboard_inputs(
//     presses: Res<ButtonInput<KeyCode>>,
//     text: Single<Entity, With<Text>>,
//     mut writer: TextUiWriter,
//     // mut camera: ResMut<CameraMode>,
// ) {

//     let entity = *text;
//     *writer.text(entity, 1) = format!("GetPosition: {:.2}\n", inst.get_position());
// }

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
