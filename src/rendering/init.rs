use std::sync::atomic::AtomicBool;

use wgpu::{Queue, Surface, Device, SurfaceConfiguration, RenderPipeline, util::DeviceExt};
use winit::{window::{Window, WindowBuilder}, event_loop::{EventLoop, self}, dpi::PhysicalSize};

use super::{wgpurenderer::Renderer, load_sprites::load_sprites, vertex::Vertex};

// Creating some of the wgpu types requires async code
pub async fn init(running: AtomicBool) -> (Renderer, event_loop::EventLoop<()>) {
    let event_loop = EventLoop::new();          //event loop is the basic loop of a window. A window needs one, otherwise it does nothing
    let window = WindowBuilder::new().build(&event_loop).unwrap();     //builds a window with the event loop. We could open multiple windows from a single program, but for now we don't need to

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
        .find(|f| f.is_srgb())            
        .unwrap_or(surface_caps.formats[0]);

    //here we create a surface configuration, this is basically just a configuration we need so we can tell the GPU
    //what to do with the surface. We need to tell it what format to use, what size to use, what present mode to use, etc.
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,                        //the format we just got from the surface_caps, most likely Bgra8UnormSrgb
        width: size.width,                             //this is the size of the window
        height: size.height,
        present_mode: surface_caps.present_modes[0],   //this basically is wgpu::PresentMode::Fifo (FIFO = First In First Out), since this is always supported and always the first
        alpha_mode: surface_caps.alpha_modes[0],       //this basically is wgpu::AlphaMode::Opaque, since this is always supported and always the first
        view_formats: vec![],
    };
    surface.configure(&device, &config);


    let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));




    //let (render_pipeline, bind_group) = load_sprites(0, &device, &queue, &shader, &config);



    //Here we create a render pipeline layout, which means we tell the GPU what kind of data we will be sending to it additionally to the vertices to render, like a texture
    let render_pipeline_layout =
    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });



    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main", 
            buffers: &[Vertex::desc()], //this tells the pipeline that it can expect a buffer of vertices, how these vertices look in memory is in desc()
        },
        fragment: Some(wgpu::FragmentState { //this is technically optional, so we need to wrap it in Some(), this stores color data
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState { //what color outputs should be set up. We only need one for the surface now
                format: config.format,
                blend: Some(wgpu::BlendState::REPLACE),  
                write_mask: wgpu::ColorWrites::ALL, // RGBA
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList, //interpret three vertices as a triangle, so we need to send 3 vertices to the GPU, the next 3 are the next triangle, etc.
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw, //counter-clockwise. This tells the GPU that the vertices are in counter-clockwise order. This is important because the texture might face the other way and thus will be invisible
            cull_mode: Some(wgpu::Face::Back), //cull mode means that we can tell the GPU to not render certain faces, like the back face, which we can't see anyway. Otherwise these would still be rendered, even tho they can't be seen
            // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
            polygon_mode: wgpu::PolygonMode::Fill,
            // Requires Features::DEPTH_CLIP_CONTROL
            unclipped_depth: false,
            // Requires Features::CONSERVATIVE_RASTERIZATION
            conservative: false,
        },
        depth_stencil: None, 
        multisample: wgpu::MultisampleState {
            count: 1, //Multisampling. A count of 1 means no multisampling, so we don't need to worry about this now
            mask: !0, //all bitmaps enabled. I don't know what they do exactly
            alpha_to_coverage_enabled: false, // 4.
        },
        multiview: None, //multiview indicates how many array layers the render attachments can have. We won't be rendering to array textures so we can set this to None
    });




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
        render_pipeline,
    },
    event_loop,
    )
}