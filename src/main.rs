use std::sync::atomic::AtomicBool;

use rendering::wgpurenderer;




mod rendering;

#[tokio::main]
pub async fn main() {

    

    env_logger::init();     //wgpu logs per default to the env_logger. If we don't initialize it, we only get very basic and not very helpful errors
    let running = AtomicBool::new(true);  //<-- this is used to indicate whether the program should exit or not

    wgpurenderer::Renderer::run(running).await;
}