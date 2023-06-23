use std::{num::NonZeroU32, default};

use wgpu::{TextureUsages, Device, RenderPipeline, BindGroup, ShaderModule, SurfaceConfiguration, TextureView};

use crate::rendering::{vertex::Vertex, wgpurenderer::Renderer};
use image::{GenericImageView, ImageBuffer};

use super::load_level_sprites::initial_level::load_initial_level_sprites;


pub fn load_sprites(_i: u32, renderer: &Renderer) -> (RenderPipeline, BindGroup) {
        

        let texture_view_array = load_initial_level_sprites(renderer);
        
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


        let texture_view_array = texture_view_array.iter().collect::<Vec<&TextureView>>();
        
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










