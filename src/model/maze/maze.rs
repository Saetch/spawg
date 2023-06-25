use std::{ time::Duration, cell::RefCell, rc::{Rc, Weak}, sync::{RwLock, Arc}};

use crate::game_objects::game_object::{DrawableObject, LogicObject};

const TIME_BETWEEN_STEPS_IN_MS : u32 = 1000;

#[derive(Debug)]
pub(crate) struct Maze{
    pub(crate) width: usize,
    pub(crate) height: usize,


    next_tile_ms: u32,
    maze: Vec<Vec<Rc<RefCell<MazeTile>>>>,
    current_path: Option<Vec<Weak<RefCell<MazeTile>>>>,
}


impl LogicObject for Maze{
    fn process_logic(&mut self, delta_time: Duration) -> bool{
        let millis = delta_time.as_millis() as u32;
        if self.next_tile_ms > millis{
            self.next_tile_ms -= millis;
            return true;
        }
        self.next_tile_ms = TIME_BETWEEN_STEPS_IN_MS - (millis - self.next_tile_ms);
        //if we have no path, we need to find one
        self.find_path_step();
        if self.current_path.is_none(){
            return true;
        }
        return false;
    }
}

impl Maze{

    fn new(width: usize, height: usize)-> (Self, Vec<Arc<RwLock<dyn DrawableObject>>>){
        //if any of these values is 0, panic!
        assert!(width > 0 && height > 0);
        let mut maze = Maze { 
            maze: vec![vec![Rc::new(RefCell::new(MazeTile{connected:(true,true,true,true),visited:false, underlying_objects: Vec::new()})); width]; height],
            width: width,
            height: height,
            current_path: Some(Vec::new()),
            next_tile_ms: 0,
         };

        //maze.set_outside_walls();
        
        //set left of 0,0 and right of width-1, height-1 to true, these are the start and end points
        RefCell::borrow_mut(&maze.maze[0][0]).connected.3 = true;

        RefCell::borrow_mut(&maze.maze[height-1][width-1]).connected.1 = true;

        (maze, Vec::new())
    }
   
   
    //set the outsides of the maze to false
    fn set_outside_walls(&mut self){
        for i in 0..self.width{
            RefCell::borrow_mut(&self.maze[0][i]).connected.0 = false;
            self.maze[self.height-1][i].borrow_mut().connected.2 = false;
        }
        for i in 0..self.height{
            self.maze[i][0].borrow_mut().connected.3 = false;
            self.maze[i][self.width-1].borrow_mut().connected.1 = false;
        }
    }



    fn find_path_step(&mut self){
        if let Some(path) = &mut self.current_path{
            if path.len() == 0{
                let rc = self.maze[0][0].clone();
                path.push(Rc::downgrade(&rc));
                
                //path.push(Arc::downgrade(&arc));
            }

        }
    }





}

#[derive(Debug, Clone)]
struct MazeTile{
    connected: (bool, bool, bool, bool),
    visited: bool,
    underlying_objects: Vec<Arc<RwLock<dyn DrawableObject>>>,
}