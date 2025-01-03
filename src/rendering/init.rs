use std::{sync::{atomic::AtomicBool, Arc}, num::NonZeroU32, io::Read, cell::RefCell};

use wgpu::{Queue, Surface, Device, SurfaceConfiguration, RenderPipeline, util::DeviceExt, ShaderModule};
use winit::{window::{Window, WindowBuilder, Fullscreen}, event_loop::{EventLoop, self, EventLoopBuilder}, dpi::PhysicalSize, event::WindowEvent};

use crate::{controller::{position::Position, controller::{SharablePosition, CAM_INITIAL_WIDTH, CAM_INITIAL_HEIGHT}}, rendering::{sprites::load_sprites::Camera, wgpurenderer::VertexBufferStructs}};

use super::{wgpurenderer::{Renderer, VertexBuffers, VertexBufferStruct, InstanceBufferState}, vertex::Vertex, sprites::vertex_configration::{VertexConfigration, VertexConfigrationTrait}, sprite_instance::SpriteInstance};

// Creating some of the wgpu types requires async code
pub async fn init(running: Arc<AtomicBool>, cam_position: SharablePosition) -> (Renderer, event_loop::EventLoop<WindowEvent<'static>>) {
    let event_loop = EventLoopBuilder::<WindowEvent>::with_user_event().build();          //event loop is the basic loop of a window. A window needs one, otherwise it does nothing
    const FORMAT: f64 = 16.0 / 9.0;                  //the aspect ratio of the window
    let requested_size = PhysicalSize::new(1400, (1400.0 / FORMAT) as u32);
    let window = WindowBuilder::new().with_inner_size(requested_size).with_title("spawg").build(&event_loop).unwrap();     //builds a window with the event loop. We could open multiple windows from a single program, but for now we don't need to
    let size = window.inner_size();
    // the instance is an actual wgpu object that we use to do everything in
    // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        dx12_shader_compiler: Default::default(),
    });
    
    // # Safety
    //
    // The surface needs to live as long as the window that created it.
    // State owns the window so this should be safe. This unsafe block is required, otherwise it won't work
    let surface = unsafe { instance.create_surface(&window) }.unwrap();


    //an adapter means a physical connection (to a GPU or other device) that supports the given Options
    let adapter = instance.request_adapter(
        &wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        },
    ).await.unwrap();


    let (device, queue) = adapter.request_device(      //this is the actual device (logical device) that we will use to render, this communicates with the physical device. The queue is what is used to execute command buffers (groups of commands to the GPU)
        &wgpu::DeviceDescriptor {
            features: (wgpu::Features::ADDRESS_MODE_CLAMP_TO_BORDER | /* <-- this is a bitwise operator, not a logical OR, which practically means AND here */ wgpu::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING
                | wgpu::Features::TEXTURE_BINDING_ARRAY),            //you can get a list of supported features by calling adapter.features() or device.features()
            // WebGL doesn't support all of wgpu's features, so if
            // we're building for the web we'll have to disable some.
            limits: wgpu::Limits::default()
            ,
            label: None,
        },
        None, // Trace path
    ).await.unwrap();

    let surface_caps = surface.get_capabilities(&adapter);

    //println!("Surface capabilities: {:?}", surface_caps);   //<-- just print out what our device can do


    // The basic shader code we're using expects an sRGB surface so using a non-sRGB
    // one will result all the colors coming out darker. If you want to support non
    // sRGB surfaces, you'll need to account for that when drawing to the frame.
    let surface_format = surface_caps.formats.iter()
        .copied()
        .find(|f: &wgpu::TextureFormat| f.is_srgb())            
        .unwrap_or(surface_caps.formats[0]);

    println!("Surface capabilities: {:?}", surface_caps);   //<-- just print out what our device can do

    let present_mode = surface_caps.present_modes.iter().find(|mode| mode == &&wgpu::PresentMode::Mailbox).unwrap_or(&wgpu::PresentMode::Fifo);
    //here we create a surface configuration, this is basically just a configuration we need so we can tell the GPU
    //what to do with the surface. We need to tell it what format to use, what size to use, what present mode to use, etc.
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,                        //the format we just got from the surface_caps, most likely Bgra8UnormSrgb
        width: size.width,                             //this is the size of the window
        height: size.height,
        present_mode: *present_mode,   //this basically is wgpu::PresentMode::Fifo (FIFO = First In First Out), if Mailbox is not supported
        alpha_mode: surface_caps.alpha_modes[0],       //this basically is wgpu::AlphaMode::Opaque, since this is always supported and always the first
        view_formats: vec![],
    };
    surface.configure(&device, &config);


    let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

    //I thought at first to load the sprites here, but this is not a good idea, since we need to load them every time we change the sprite sheet

    const INDICES: &[u16] = &[
        0, 1, 2,  // Triangle ABC
        0, 2, 3,  // Triangle ACD
    ];
    let index_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&INDICES),
            usage: wgpu::BufferUsages::INDEX,
        }
    );


    let cam_size = [CAM_INITIAL_WIDTH, CAM_INITIAL_HEIGHT];
    let camera: Camera = Camera{
        size: cam_size,
        position: [0.0, 0.0],
    };
    
    
    let uniform_camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Camera Buffer"),
        contents: bytemuck::cast_slice(&[camera]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let vertex_structs: VertexBufferStructs = load_buffer_structs(&device);


    //now we create a struct that holds all these important things, so we can use it 
    (
        Renderer{
            window,
            surface,
            device,
            queue,
            config,
            size,
            running,
            shader,
            render_receiver: None,
            index_buffer,
            vertex_structs,
            cam_size,
            camera_buffer: uniform_camera_buffer,
            to_upgrade_vec: RefCell::new(Vec::new()),
        },
        event_loop
    )
}


fn load_buffer_structs(device: &Device) -> VertexBufferStructs{

    [
        VertexBufferStruct{
            vertex_buffer: create_vertex_buffer_for_config(device, VertexConfigration::SQUARE_SMALL_1),
            instance_state: create_initial_instance_buffer_struct(device, 10),
        },
        VertexBufferStruct{
            vertex_buffer: create_vertex_buffer_for_config(device, VertexConfigration::NEARLY_SQUARE_RECTANGLE_0),
            instance_state: create_initial_instance_buffer_struct(device, 1500),
        },
        VertexBufferStruct{
            vertex_buffer: create_vertex_buffer_for_config(device,VertexConfigration::LINE_HORIZONTAL),
            instance_state: create_initial_instance_buffer_struct(device, 1000),
        },
        VertexBufferStruct{
            vertex_buffer:  create_vertex_buffer_for_config(device,VertexConfigration::LINE_VERTICAL),
            instance_state: create_initial_instance_buffer_struct(device, 1000),
        },
        VertexBufferStruct{
            vertex_buffer: create_vertex_buffer_for_config(device,VertexConfigration::SMALL_ENTITY_WORKER),
            instance_state: create_initial_instance_buffer_struct(device, 50),
        },
        VertexBufferStruct{
            vertex_buffer: create_vertex_buffer_for_config(device,VertexConfigration::BIG_BASE_BASE_4X2),
            instance_state: create_initial_instance_buffer_struct(device, 10),
        },

    ]

}

fn create_initial_instance_buffer_struct(device: &Device, instance_size: usize) -> InstanceBufferState{
    
    let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Instance Buffer"),
        size: (instance_size * std::mem::size_of::<SpriteInstance>()) as u64,
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    InstanceBufferState{
        instance_buffer,
        num_instance_size: RefCell::new(instance_size as u32),
    }
}



fn create_vertex_buffer_for_config(device: &Device, config: VertexConfigration)-> wgpu::Buffer{
    let vertices = config.get_vertices();
    let vertex_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        }
    );
    vertex_buffer
}


