use winit::event::{VirtualKeyCode, ElementState, MouseButton, MouseScrollDelta, TouchPhase};
//modifiers: ctrl, shift, alt, represented with binary OR -> 00000111
#[derive(Clone, Copy)]
pub enum ControllerInput{
    MouseInput{  action: MouseInputType},
    KeyboardInput{ key: Option<VirtualKeyCode>, state : ElementState },  
    WindowResized{ dimensions : (u32,u32) },
}

#[derive(Clone, Copy)]
pub enum MouseInputType{
    Move(f32, f32),
    Click{ button: MouseButton, state: ElementState},
    Scroll{ delta: MouseScrollDelta, phase: TouchPhase },                                          //use Later if desired
    LeftWindow,
    EnteredWindow
    
}