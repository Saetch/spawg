use std::{sync::{atomic::AtomicBool, Arc, RwLock}, ops::DerefMut};

use async_std::task::{TaskId, Task, block_on};
use wgpu::{util::DeviceExt, ShaderModule, RenderPipeline, BindGroup, Device};
use winit::{window::Window, event::WindowEvent};

use crate::{rendering::{vertex::Vertex, sprites::{sprite_mapping::Sprite, vertex_configration::{VertexConfigrationTrait, self}}}, controller::{position::Position, controller::SharablePosition}, model::model::GameObjectList};

use super::{sprite_instance::SpriteInstance, sprites::vertex_configration::VertexConfigration};



const NUM_INDICES_PER_SPRITE: u32 = 6;
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
    pub(crate) render_receiver: Option<flume::Receiver<Vec<RenderChunk>>>,
    pub(crate) index_buffer: wgpu::Buffer,
}

#[derive(Debug)]
pub struct RenderChunk{
    pub(crate) vertex_conf: VertexConfigration,
    pub(crate) vertex_buffer: Vec<Vertex>,
    pub(crate) instance_buffer: Vec<SpriteInstance>,
}
pub struct RenderChunkRaw<'a>{
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) instance_buffer: wgpu::Buffer,
    pub(crate) num_indices: u32,
    pub(crate) instances_len: usize,
    pub(crate) index_buffer: &'a wgpu::Buffer,
}



impl PartialEq for RenderChunk {
    fn eq(&self, other: &Self) -> bool {
        self.vertex_conf as u32 == other.vertex_conf as u32
    }
}

#[derive(Debug)]
pub struct SpriteBuffer {
    pub(crate) instances_buffer: Vec<SpriteInstance>,
}



pub struct SpriteBufferRaw{
    pub(crate) instances_buffer: wgpu::Buffer,
}



impl Renderer {

    #[inline(always)]
    pub(crate) fn create_instance_buffer_from_instance_vector(&self, instances_buffer: Vec<SpriteInstance>) -> wgpu::Buffer {
        self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instances_buffer),
                usage: wgpu::BufferUsages::VERTEX,
            }
        )
    }

    #[inline(always)]
    fn create_vertex_buffer_from_vector(&self, vertex_buffer: Vec<Vertex>) -> wgpu::Buffer {
        self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertex_buffer),
                usage: wgpu::BufferUsages::VERTEX,
            }
        )
    }

    #[inline(always)]
    fn chunk_to_raw(&self, chunk: RenderChunk) -> RenderChunkRaw{
        let len = chunk.instance_buffer.len();

        RenderChunkRaw{
            vertex_buffer: self.create_vertex_buffer_from_vector(chunk.vertex_buffer),
            index_buffer: &self.index_buffer,
            instance_buffer: self.create_instance_buffer_from_instance_vector(chunk.instance_buffer),
            num_indices: NUM_INDICES_PER_SPRITE,
            instances_len: len,
        }
    }


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

    #[inline(always)]
    pub(crate) fn render(&mut self, render_pipeline: &RenderPipeline, bind_group: &BindGroup, camera_bind_group: &BindGroup) -> Result<(), wgpu::SurfaceError> {


        if self.render_receiver.is_none(){
            return Ok(());
        }

        let render_ops = self.render_receiver.as_ref().unwrap().recv().unwrap();
  
        let mut chunk_raw_vec = Vec::with_capacity(render_ops.len());


        for op in render_ops.into_iter(){
            let raw = self.chunk_to_raw(op);
            chunk_raw_vec.push(raw);
        }
        
        
        //the surface is the inner part of the window, the output (surfaceTexture) is the actual texture that we will render to
        let output = self.surface.get_current_texture()?;
        
        //this is required to tell the code how the rendering is done
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        //this encoder is able to create a command buffer, which is a list of commands that will be executed by the GPU
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        
        
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


            for render_op in chunk_raw_vec.iter() {


                render_pass.set_pipeline(&render_pipeline);   //the correct pipeline tells the GPU what shaders will be used on the vertices
                render_pass.set_bind_group(0, bind_group, &[]);  //this bind group contains the textures we loaded, if we want to switch all of the textures at once, we can do that by switching to another bind group. Might create some interesting effects
                render_pass.set_bind_group(1, camera_bind_group, &[]);
                render_pass.set_vertex_buffer(0, render_op.vertex_buffer.slice(..));
                render_pass.set_vertex_buffer(1, render_op.instance_buffer.slice(..));
                render_pass.set_index_buffer(render_op.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..render_op.num_indices, 0, 0..render_op.instances_len as u32);
            }




        }
        

        
        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    
        Ok(())



    }
}

