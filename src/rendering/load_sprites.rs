use std::{num::NonZeroU32, default};

use wgpu::{TextureUsages, Device, RenderPipeline, BindGroup, ShaderModule, SurfaceConfiguration, TextureView};

use crate::rendering::{vertex::Vertex};
use image::{GenericImageView, ImageBuffer};

use super::wgpurenderer::Renderer;

pub fn load_sprites(_i: u32, renderer: &Renderer) -> (RenderPipeline, BindGroup) {
            

        //consider moving these to a file to specify which textures to load and where to load them from, based on the level selected
        let diffuse_bytes = std::fs::read("./textures/Dwarf_BaseHouse.png").expect("Failed to read texture file");
        let dwarf_base_house_texture_view = load_sprite_from_memory(&diffuse_bytes, 135,45, 380, 517,  renderer);
        let diffuse_bytes = std::fs::read("./textures/Dwarf_BaseHouse_px9.png").expect("Failed to read texture file");
        let dwarf_base_house_texture_view2 = load_sprite_from_memory(&diffuse_bytes, 135,45, 380, 517, renderer);
        let diffuse_bytes = std::fs::read("./textures/black_pixel.png").expect("Failed to read texture file");
        let black_pixel_texture_view = load_sprite_from_memory_uncropped(&diffuse_bytes, renderer);


            
        let mut texture_view_array = Vec::new();
        texture_view_array.push(&dwarf_base_house_texture_view);
        texture_view_array.push(&dwarf_base_house_texture_view2);
        texture_view_array.push(&black_pixel_texture_view);

        
        let diffuse_sampler = renderer.device.create_sampler(&wgpu::SamplerDescriptor { //a sampler will accept coordinates (X/Y) and return the color data. So this object is asked when the texture is the source of any color operation
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()                        //rest of the fields are initialized with default values
        });
        
        //bind groups describe resources that a shaders has access to
        let texture_bind_group_layout = renderer.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[                     //2 Entries: 1st: Texture, 2nd: Sampler for texture
                wgpu::BindGroupLayoutEntry {    
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: NonZeroU32::new(texture_view_array.len() as u32),
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    // This should match the filterable field of the
                    // corresponding Texture entry above.
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        });



        //create the actual bind group based on the bind-group-layout. This looks almost identical tho, but it means you could switch these out at runtime, go for another bind group and thus change the textures
        let diffuse_bind_group = renderer.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureViewArray(&texture_view_array),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
                    }
                ],
                label: Some("diffuse_bind_group"),
            }
        );

        
        let render_pipeline_layout =
        renderer.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&texture_bind_group_layout],
            push_constant_ranges: &[],
        });


        let render_pipeline = renderer.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &renderer.shader,
                entry_point: "vs_main", // 1.
                buffers: &[
                    Vertex::desc(),                                 //insert the vertex buffer that was created above
                ], // 2.
            },
            fragment: Some(wgpu::FragmentState { // 3.              //fragment is optional and thus wrapped in Some(), this is needed for storing color on the surface
                module: &renderer.shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState { // 4.
                    format: renderer.config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),         //replace pixels instead of blending
                    write_mask: wgpu::ColorWrites::ALL,             //specify color channels (R, G, B or similiar) that can be written to. Others will be ignored 
                })],
            }),    
                primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.  //every 3 vertices in order are considered a triangle
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.            //Ccw: Counter-clockwise. This means, that if the vertices are ordered counter-clockwise, the triangle is facing us (only the front is visible)
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },    depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1, // 2.
                mask: !0, // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
        });

        (render_pipeline, diffuse_bind_group)
}







//convert the given image buffer from rgba into srgba
fn to_srgba(image: ImageBuffer<image::Rgba<u8>, Vec<u8>>) -> Vec<u8> {
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


fn load_sprite_from_memory_uncropped(diffuse_bytes: &[u8], renderer: &Renderer) -> TextureView{
    load_sprite_from_memory(diffuse_bytes, 0, 0, 0, 0, renderer)
}

fn load_sprite_from_memory(diffuse_bytes: &[u8], crop_x: u32, crop_y: u32, crop_width: u32, crop_height: u32, renderer: &Renderer)  -> TextureView{
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