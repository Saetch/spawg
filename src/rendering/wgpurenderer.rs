use std::sync::{atomic::AtomicBool, Arc, RwLock};

use wgpu::{util::DeviceExt, ShaderModule, RenderPipeline, BindGroup};
use winit::{window::Window, event::WindowEvent};

use crate::rendering::vertex::Vertex;

#[derive(Debug)]
#[allow(unused)]
pub struct Renderer {
    pub(crate) surface: wgpu::Surface,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) config: wgpu::SurfaceConfiguration,
    pub(crate) size: winit::dpi::PhysicalSize<u32>,
    pub(crate) window: Window,
    pub(crate) running: Arc<AtomicBool>,  //<-- this is used to indicate whether the program should exit or not
    pub(crate) shader: ShaderModule,
    pub(crate) cam_pos: DummyPosition,
}

#[derive(Debug)]
pub struct DummyPosition {
    pub(crate) x: Arc<RwLock<f32>>,
    pub(crate) y: Arc<RwLock<f32>>,
}

impl Renderer {

    pub fn window(&self) -> &Window {
        &self.window
    }


    //this function is supposed to be used when the window is resized with a resize event and just adapts the configuration and configures the surface
    #[inline(always)]
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    //to indicate whether an event has been fully processed. If the method returns true, the main loop won't process the event any further. This will be implemented later
    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {

    }

    #[inline(always)]
    pub(crate) fn render(&mut self, render_pipeline: &RenderPipeline, bind_group: &BindGroup) -> Result<(), wgpu::SurfaceError> {


        let mut vertices: &[Vertex] = &[
            Vertex { position: [1.0, 0.0, 0.0], tex_coords: [1.0, 1.0], texture_id: 0 }, // A
            Vertex { position: [1.0, 1.0, 0.0], tex_coords: [1.0, 0.0], texture_id: 0 }, // B
            Vertex { position: [0.0, 1.0, 0.0], tex_coords: [0.0, 0.0], texture_id: 0 }, // C
            Vertex { position: [0.0, 0.0, 0.0], tex_coords: [0.0, 1.0], texture_id: 0 }, // D
        ];



        
        const INDICES: &[u16] = &[
            0, 1, 2,
            0, 2, 3,
        ];
        let num_indices = INDICES.len() as u32;



        //the surface is the inner part of the window, the output (surfaceTexture) is the actual texture that we will render to
        let output = self.surface.get_current_texture()?;
        
        //this is required to tell the code how the rendering is done
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        //this encoder is able to create a command buffer, which is a list of commands that will be executed by the GPU
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        let vertex_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let index_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        //these {} brackets are used, because begin_render_pass borrows encoder mutably and we need to return that borrow before we can call encoder.finish()
        {
            
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });


                // NEW!
            render_pass.set_pipeline(&render_pipeline);   //the correct pipeline tells the GPU what shaders will be used on the vertices
            render_pass.set_bind_group(0, bind_group, &[]);  //this bind group contains the textures we loaded, if we want to switch all of the textures at once, we can do that by switching to another bind group. Might create some interesting effects
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..num_indices, 0, 0..1);
        }
    
        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    
        Ok(())



    }
}