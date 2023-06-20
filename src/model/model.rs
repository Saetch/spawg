use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, os::windows::thread, time::Duration};

pub(crate) struct Model{
    pub(crate) running: Arc<AtomicBool>,  //<-- this is used to indicate whether the program should exit or not
}



impl Model{
    pub(crate) fn new(running:Arc< AtomicBool>) -> Self{
        Self{
            running: running,
        }
    }


    


    pub(crate) fn run(&mut self){


        while self.running.load(Ordering::Relaxed){
            //do stuff
            self.compute_logic_objects();
            //sleep 1 second
            std::thread::sleep(Duration::from_secs(1));
        }   



        println!("Model thread exited");
    }




    #[inline(always)]
    fn compute_logic_objects(&mut self){
        println!("Model thread computing logic objects");
    }
}
