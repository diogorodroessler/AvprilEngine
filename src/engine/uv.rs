use bevy::{ecs::component::Component, image::{Image, ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor}};

#[derive(Component)]
pub struct UvDebug;

impl UvDebug {
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
