use image::{GenericImageView, ImageBuffer};
use wgpu::{TextureUsages, TextureView};

use crate::rendering::wgpurenderer::Renderer;

pub(super) fn load_sprite_from_memory_uncropped(diffuse_bytes: &[u8], renderer: &Renderer) -> TextureView{
    load_sprite_from_memory(diffuse_bytes, 0, 0, 0, 0, renderer)
}

pub(super) fn load_sprite_from_memory(diffuse_bytes: &[u8], crop_x: u32, crop_y: u32, crop_width: u32, crop_height: u32, renderer: &Renderer)  -> TextureView{
    //loading an image from a file
    let diffuse_image = image::load_from_memory(diffuse_bytes).unwrap();
    let cropped_image;

    if crop_width == 0 || crop_height == 0  {
        cropped_image = diffuse_image;
    } else {
        // Crop the image to remove dead spaces
        cropped_image = diffuse_image.crop_imm(crop_x,crop_y, crop_width, crop_height);   //this crops the image, in this case we just shave off the empty space to the sides etc. This is most likely specific (if needed at all) for every texture
    }

    
    let dimensions = cropped_image.dimensions();
    
    let diffuse_rgba: ImageBuffer<image::Rgba<u8>, Vec<u8>> = cropped_image.to_rgba8();
    let diffuse_rgba: Vec<u8> = to_srgba(diffuse_rgba);    //this is necessary for images that are in rgba format, if the image is in srgba format, this should not be done, otherwise the colors will get distorted
    
    let texture_size = wgpu::Extent3d {
        width: dimensions.0,
        height: dimensions.1,
        depth_or_array_layers: 1,
    };

    let diffuse_texture = renderer.device.create_texture(&wgpu::TextureDescriptor {

        label: Some("distinct texture"),
        size: texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: TextureUsages::COPY_DST | TextureUsages::COPY_SRC | TextureUsages::TEXTURE_BINDING,
        view_formats: &[],  //Default, RGBA8Unorm is always supported
    
        
    });


    //this execute a write on the gpu from the loaded image pixel data into our created texture
    renderer.queue.write_texture(
        // Tells wgpu where to copy the pixel data
        wgpu::ImageCopyTexture {
            texture: &diffuse_texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        // The actual pixel data
        diffuse_rgba.as_slice(),
        // The layout of the texture
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(4 * dimensions.0),
            rows_per_image: Some(dimensions.1),
        },
        texture_size,
    );
    // We don't need to configure the texture view much, so let's
    // let wgpu define it.
    let texture_view = diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());        //create a handle to access the texture we just created
    texture_view


        /*      This is another way to load an image, but it is not as easy to use as the one above (not as flexible), tho more slim
        
            let diffuse_texture = {
                let img_data = include_bytes!("../image_img.png");
                let decoder = png::Decoder::new(std::io::Cursor::new(img_data));
                let mut reader = decoder.read_info().unwrap();
                let mut buf = vec![0; reader.output_buffer_size()];
                let info = reader.next_frame(&mut buf).unwrap();
    
                let size = wgpu::Extent3d {
                    width: info.width,
                    height: info.height,
                    depth_or_array_layers: 1,
                };
                let texture_format = wgpu::TextureFormat::Rgba8UnormSrgb;
                let texture = device.create_texture(&wgpu::TextureDescriptor {
                    label: None,
                    size,
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: texture_format,
                    usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
                });
                queue.write_texture(
                    texture.as_image_copy(),
                    &buf,
                    wgpu::ImageDataLayout {
                        offset: 0,
                        bytes_per_row: std::num::NonZeroU32::new(info.width * 4),
                        rows_per_image: None,
                    },
                    size,
                );
                texture
            };*/
}


//convert the given image buffer from rgba into srgba
pub(super) fn to_srgba(image: ImageBuffer<image::Rgba<u8>, Vec<u8>>) -> Vec<u8> {
    image.chunks_exact(4)
    .flat_map(|rgba| {
        let r = rgba[0] as f32 / 255.0;
        let g = rgba[1] as f32 / 255.0;
        let b = rgba[2] as f32 / 255.0;
        let a = rgba[3] as f32 / 255.0;

        // Apply gamma correction (from linear RGB to sRGB)
        let r_gamma = r.powf(2.2);
        let g_gamma = g.powf(2.2);
        let b_gamma = b.powf(2.2);

        let r_gamma_u8 = (r_gamma * 255.0) as u8;
        let g_gamma_u8 = (g_gamma * 255.0) as u8;
        let b_gamma_u8 = (b_gamma * 255.0) as u8;
        let a_u8 = (a * 255.0) as u8;

        vec![r_gamma_u8, g_gamma_u8, b_gamma_u8, a_u8]
    })
    .collect()
}