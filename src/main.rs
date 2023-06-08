use std::{error::Error};

use winit::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, event::{WindowEvent, KeyboardInput, ElementState, VirtualKeyCode, Event}};


pub fn main() {
    env_logger::init();     //wgpu logs per default to the env_logger. If we don't initialize it, we only get very basic and not very helpful errors
    let event_loop = EventLoop::new();          //event loop is the basic loop of a window. A window needs one, otherwise it does nothing
    let window = WindowBuilder::new().build(&event_loop).unwrap();     //builds a window with the event loop. We could open multiple windows from a single program, but for now we don't need to
     //control_flow is a variable that can tell the window some special commands, but usually is just used to close the program
    event_loop.run(move |event, _, control_flow| match event {   //<- this match means we can do different things depending on the type of Event
        Event::WindowEvent {           //so here we match it to a WindowEvent, meaning anything that relates to a window
            ref event,   //here we bind the actual type of the Event to the variable 'event', this means that we can use it in the match below, this might be something like Resize or Close etc.    
            window_id,       //here we get the id of the window. We need this to check wether the window the event is for is actually the correct one
        } if window_id == window.id() => match event {      //here we check if the window_id is the same as the one we created above. If it is, we use a match to process what kind of Event we got
            WindowEvent::CloseRequested       //This means that the 'X' got pressed or alt+f4
            | WindowEvent::KeyboardInput {    //This means that a key got pressed or released
                input:                        //these lines just check what is pressed and do something depending on it, for now only escape is implemented
                    KeyboardInput {           //this will most likely be put into its own function later
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),    //<-- escape is pressed
                        ..                     //this is a shorthand for 'everything else', so we don't have to write out all the other fields that we don't care about
                    },
                ..
            } =>                              //this is the end of the match (CloseRequested or Escape pressed)
                *control_flow = ControlFlow::Exit ,       //ControlFlow::Exit tells the program to shutdown
            _ => {}                           //every match needs to be exhaustive, so we need to process all cases. We just ignore all others with this line
        },
        _ => {}                        //same as above, we just ignore all other events
    });
}