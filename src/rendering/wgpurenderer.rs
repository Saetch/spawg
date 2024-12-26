use std::{sync::{atomic::AtomicBool, Arc, RwLock}, ops::DerefMut, cell::RefCell};

use async_std::task::{TaskId, Task, block_on};
use wgpu::{util::DeviceExt, ShaderModule, RenderPipeline, BindGroup, Device, CommandBuffer, Buffer};
use winit::{window::Window, event::WindowEvent};

use crate::{rendering::{vertex::Vertex, sprites::{sprite_mapping::Sprite, vertex_configration::{VertexConfigrationTrait, self}}}, controller::{position::Position, controller::SharablePosition}, model::model::GameObjectList, cam_organizer::cam_organizer::CamState};

use super::{sprite_instance::SpriteInstance, sprites::vertex_configration::{VertexConfigration, NUM_VERTEX_CONFIGURATIONS}};

#[allow(unused)]
pub(crate) type VertexBuffers = [wgpu::Buffer; NUM_VERTEX_CONFIGURATIONS];   //<--Update this. This will updated other uses as well, less error prone

pub(crate) type VertexBufferStructs = [VertexBufferStruct; NUM_VERTEX_CONFIGURATIONS];   //<--Update this. This will updated other uses as well, less error prone

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
    pub(crate) render_receiver: Option<flume::Receiver<(Vec<RenderChunk>, CamState)>>,
    pub(crate) index_buffer: wgpu::Buffer,
    pub(crate) vertex_structs: VertexBufferStructs,  //<--this needs to be updated if the number of vertex_configurations changes! this is used to store the vertex buffers for the sprites, so they dont have to be recreated!
    pub(crate) cam_size: [f32; 2],
    pub(crate) camera_buffer: wgpu::Buffer,
    pub(crate) to_upgrade_vec: RefCell<Vec<(usize, Vec<SpriteInstance>)>>,
}

#[derive(Debug)]
pub struct RenderChunk{
    pub(crate) vertex_conf: VertexConfigration,
    pub(crate) instance_buffer: Vec<SpriteInstance>,
}
pub struct RenderChunkRaw<'a>{
    pub(crate) vertex_buffer: &'a wgpu::Buffer,
    pub(crate) instance_buffer: &'a wgpu::Buffer,
    pub(crate) num_indices: u32,
    pub(crate) instances_len: usize,
    pub(crate) index_buffer: &'a wgpu::Buffer,
}


#[derive(Debug)]
pub(crate) struct VertexBufferStruct{
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) instance_state: InstanceBufferState,
}

#[derive(Debug)]
pub(crate) struct InstanceBufferState{
    pub(crate) instance_buffer: wgpu::Buffer,
    pub(crate) num_instance_size: RefCell<u32>,
}


#[derive(Debug)]
struct UpdateBufferStruct{
    pub(crate) buffer: wgpu::Buffer,
    pub(crate) offset: usize,
}


impl Renderer {




    #[inline(always)]
    fn chunk_to_raw<'a>(&'a self, chunk: RenderChunk, current_override_len: usize) -> (RenderChunkRaw, Option<UpdateBufferStruct>){
        let len = chunk.instance_buffer.len();
        let id = chunk.vertex_conf as usize;
        let max_amount_to_render = self.vertex_structs[id].instance_state.num_instance_size.borrow().clone() as usize;
        let amount_to_render = if max_amount_to_render > len {len} else {max_amount_to_render};
        let ret = self.update_instance_buffer(chunk.instance_buffer, id);
        (RenderChunkRaw{
            vertex_buffer: &self.vertex_structs[id].vertex_buffer,
            index_buffer: &self.index_buffer,
            instance_buffer:  &self.vertex_structs[id].instance_state.instance_buffer,
            num_indices: NUM_INDICES_PER_SPRITE,
            instances_len: amount_to_render,
        }, ret)
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

        let (render_ops, cam_state) = self.render_receiver.as_ref().unwrap().recv().unwrap();
  

        self.update_camera_buffer(&cam_state);
        let mut chunk_raw_vec = Vec::with_capacity(render_ops.len());
        let mut to_update_vec = Vec::new();
        for op in render_ops.into_iter(){
            let (raw, to_update)  = self.chunk_to_raw(op, to_update_vec.len());
            chunk_raw_vec.push(raw);
            if let Some(to_update) = to_update{
                to_update_vec.push(to_update);
            }
        }
        
        for u in to_update_vec.iter(){
            println!("updating instance buffer for vertex configuration list {:?}", u.offset);
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

                render_pass.set_vertex_buffer(1, render_op.instance_buffer.slice(..));
                render_pass.set_index_buffer(render_op.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..render_op.num_indices, 0, 0..render_op.instances_len as u32);
            }




        }
        

        
        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        self.set_instance_buffers(to_update_vec);
    
        Ok(())



    }


#[inline(always)]
    fn update_camera_buffer(&mut self, cam_state: &CamState){
        if self.cam_size == cam_state.cam_size{
            self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[cam_state.cam_pos]));
            return;
        }
        self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[*cam_state]));
        self.cam_size = cam_state.cam_size;
    }

    #[inline(always)]
    fn update_instance_buffer<'a>(&'a self, instances_buffer: Vec<SpriteInstance>, id: usize) -> Option<UpdateBufferStruct>{
        let len = instances_buffer.len();
        let mut borrow = self.vertex_structs[id].instance_state.num_instance_size.borrow_mut();
        let size = borrow.deref_mut();
        if len < *size as usize {
            self.queue.write_buffer(&self.vertex_structs[id].instance_state.instance_buffer, 0, bytemuck::cast_slice(&instances_buffer));
            return None;
        }
        let new_size = len as u32 + 1300;
        *size = new_size;
        let buf = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Instance Buffer"),
            size: (new_size * std::mem::size_of::<SpriteInstance>() as u32) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        println!("New buffer size: {}", new_size);

        self.queue.write_buffer(&buf, 0, bytemuck::cast_slice(&instances_buffer));

        Some(UpdateBufferStruct { buffer: buf, offset: id })

    }   


    fn set_instance_buffers(&mut self, to_update_vec: Vec<UpdateBufferStruct>){
        for (u) in to_update_vec.into_iter(){
            println!("updating instance buffer {}", u.offset);
            self.vertex_structs[u.offset].instance_state.instance_buffer = u.buffer;
        }
    }


}




