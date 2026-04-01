use bevy::prelude::App;
use bevy::{
    app::Update,
    asset::Assets,
    ecs::{
        component::Component,
        system::{Commands, ResMut},
    },
    image::{Image, ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor},
    mesh::Mesh,
    pbr::StandardMaterial,
};

/// Insert Textures on the Ground, or in somethings using the commands.spawn
/// # Example
/// ```
#[allow(unused)]
fn main() {
    App::new().add_systems(Update, uv_example_run).run();
}
#[allow(unused)]
fn uv_example_run(
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    use bevy::prelude::StandardMaterial;
    use bevy::prelude::*;

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
}
#[allow(unused)]
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
/// ```
#[derive(Component)]
pub struct UvDebug;

impl UvDebug {
    /// Insert Textures on the Ground, or in somethings using the add_systems(Update) directly!
    pub fn uv_debug_texture() -> Image {
        use bevy::{asset::RenderAssetUsages, render::render_resource::*};
        const TEXTURE_SIZE: usize = 7;

        let mut palette = [
            164, 164, 164, 255, 168, 168, 168, 255, 153, 153, 153, 255, 139, 139, 139, 255, 153,
            153, 153, 255, 177, 177, 177, 255, 159, 159, 159, 255,
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
}
