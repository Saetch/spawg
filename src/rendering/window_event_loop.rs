use std::{sync::{atomic::AtomicBool, Arc}, thread::JoinHandle};

use winit::{event::{Event, WindowEvent}, event_loop::{ControlFlow}};

use crate::controller::input::{ControllerInput, MouseInputType};

use super::{init::init, wgpurenderer::{Renderer, DummyPosition}, sprites::load_sprites::load_sprites};

impl Renderer {

    //this is the main loop of the program, it will be called from main.rs
    //this whole file is only for putting the event loop and window handling in one easy to use place
    #[inline(always)]
    pub(crate) async fn run(running: Arc<AtomicBool>, mut join_handles: Vec<JoinHandle<()>>, controller_sender: flume::Sender<ControllerInput>, cam_pos: DummyPosition) {


        //this is the most important struct for the current state. Almost all infos are grouped here
        let (mut renderer, event_loop) = init(running, cam_pos).await;  //we cannot put the event_loop into the Renderer struct, as the .run() function requires a move, which takes ownership of the values in it. And it is not possible for a data field to take ownership of the struct it is in
        
        renderer.instance();
        #[allow(unused)]
        let (mut render_pipeline, mut bind_group) = load_sprites(0, &renderer);
        


        
        event_loop.run(move |event, _, control_flow| match event {
            Event::RedrawRequested(window_id) if window_id == renderer.window.id() => {
                //we could trigger this Event by calling window.request_redraw(), for example in MainEventsCleared, but rendering right there is faster due to reduced function overhead
            }
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == renderer.window.id() => {match event {
                //These Window-Events are prebaked, we only need to know which ones to respond to and how
                WindowEvent::Resized(physical_size) => {
                    if physical_size.width > 0 && physical_size.height > 0 {
                        renderer.size = *physical_size;
                        renderer.config.width = renderer.size.width;
                        renderer.config.height = renderer.size.height;
                        renderer.surface.configure(&renderer.device, &renderer.config);


                        //TODO: If the window gets resized, the controller should be notified, otherwise it might calculate mouse positions, etc. wrong
                    }
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    // new_inner_size is &&mut so we have to dereference it twice
                    if new_inner_size.width > 0 && new_inner_size.height > 0 {
                        renderer.size = **new_inner_size;
                        renderer.config.width = renderer.size.width;
                        renderer.config.height = renderer.size.height;
                        renderer.surface.configure(&renderer.device, &renderer.config);
                    }
                }
                WindowEvent::CloseRequested
                 => {
                    renderer.running.store(false, std::sync::atomic::Ordering::SeqCst);
                    controller_sender.send(ControllerInput::Exit).expect("Could not send exit info to controller thread!");
                    //now we wait for the other threads to finish, before we finally close the program completely we cannot just use for handles in join_handles, because they would still exist, but be captured by the move closure, which would be a problem
                    join_handles.drain(..).for_each(|join_handle| {
                        join_handle.join().unwrap();
                    });

                    println!("Gracefully exiting ...");
                    *control_flow = ControlFlow::Exit;

                }
                //send necessary inputs to the controller thread for further evaluation
               WindowEvent::KeyboardInput { device_id: _ , input, is_synthetic: _ }
                 => {
                    let ret = controller_sender.send(ControllerInput::KeyboardInput { key: input.virtual_keycode, state : input.state });
                    if ret.is_err(){
                        if renderer.running.load(std::sync::atomic::Ordering::SeqCst) {
                            println!("Could not send keyboard input details to controller thread!");
                        }
                    }
                }
                WindowEvent::MouseInput { device_id: _, state , button: btn, .. }
                 => {
                    let ret = controller_sender.send(ControllerInput::MouseInput { action: MouseInputType::Click { button: *btn, state: *state } });
                    if ret.is_err(){
                        if renderer.running.load(std::sync::atomic::Ordering::SeqCst) {
                            println!("Could not send mouse input details to controller thread!");
                        }
                    }
                }
                WindowEvent::CursorLeft { device_id: _ }
                 => {
                    let ret = controller_sender.send( ControllerInput::MouseInput { action: MouseInputType::LeftWindow });
                    if ret.is_err(){
                        if renderer.running.load(std::sync::atomic::Ordering::SeqCst) {
                            println!("Could not send cursor left info to controller thread!");
                        }
                    }
                }
                WindowEvent::CursorEntered { device_id: _ }
                 => {
                    let ret = controller_sender.send( ControllerInput::MouseInput { action: MouseInputType::EnteredWindow });
                    if ret.is_err(){
                        if renderer.running.load(std::sync::atomic::Ordering::SeqCst) {
                            println!("Could not send cursor entered info to controller thread!");
                        }
                    }
                }
                WindowEvent::CursorMoved { device_id: _, position, .. }
                 => {
                    let ret = controller_sender.send( ControllerInput::MouseInput { action: MouseInputType::Move(position.x as f32, position.y as f32) });
                    if ret.is_err(){
                        if renderer.running.load(std::sync::atomic::Ordering::SeqCst) {
                            println!("Could not send cursor moved info to controller thread!");
                        }
                    }
                }
                WindowEvent::MouseWheel { device_id: _, delta, phase , ..}
                 => {
                    let ret = controller_sender.send( ControllerInput::MouseInput { action: MouseInputType::Scroll { delta: *delta, phase: *phase } });
                    if ret.is_err(){
                        if renderer.running.load(std::sync::atomic::Ordering::SeqCst) {
                            println!("Could not send mouse wheel info to controller thread!");
                        }
                    }
                }
                _ => {}
            }
    
        }
        Event::MainEventsCleared => {
            let res = renderer.render(&render_pipeline, &bind_group);
            if let Err(e) = res {
                eprintln!("Error during rendering: {:?}", e);
            }
        }
            _ => {}
        });
    }
    
}