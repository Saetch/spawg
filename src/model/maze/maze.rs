use std::{ time::Duration, cell::{RefCell, Ref, RefMut}, rc::{Rc, Weak}, sync::{RwLock, Arc}, ops::Deref};

use rand::Rng;

use crate::{game_objects::game_object::{DrawableObject, LogicObject}, model::results::{LogicResult, GameObjects}, controller::controller::Direction};

const TIME_BETWEEN_STEPS_IN_MS : u32 = 1000;

#[derive(Debug)]
pub(crate) struct Maze{
    pub(crate) width: usize,
    pub(crate) height: usize,

    start_tile: Weak<RefCell<MazeTile>>,
    end_tile: Weak<RefCell<MazeTile>>,
    next_tile_ms: u32,
    maze: Vec<Vec<Rc<RefCell<MazeTile>>>>,
    current_path: Option<Vec<Weak<RefCell<MazeTile>>>>,
}


impl LogicObject for Maze{
    fn process_logic(&mut self, delta_time: Duration) -> LogicResult{
        let millis = delta_time.as_millis() as u32;
        if self.next_tile_ms > millis{
            self.next_tile_ms -= millis;
            return LogicResult::None;
        }
        self.next_tile_ms = TIME_BETWEEN_STEPS_IN_MS - (millis - self.next_tile_ms);
        //if we have no path, we need to find one
        self.find_path_step();

        if self.current_path.is_some(){
            return LogicResult::None;
        }
        return LogicResult::SelfDestruct;
    }
}

impl Maze{

    fn new(width: usize, height: usize)-> (Self, GameObjects){
        //if any of these values is 0, panic!
        assert!(width > 0 && height > 0);
        let mut maze = vec![vec![Rc::new(RefCell::new(MazeTile{position: (0, 0),connected:(true,true,true,true),visited:false, underlying_objects: Vec::new()})); width]; height];
        //set the correct positions for the MazeTiles
        for i in 0..height{
            for j in 0..width{
                maze[i][j].borrow_mut().position = (i, j);
            }
        }
        let start_tile= Rc::downgrade(&maze[0][0].clone());
        let end_tile= Rc::downgrade(&maze[height-1][width-1].clone());
        let mut maze = Maze { 
            maze: maze,
            width: width,
            height: height,
            current_path: Some(Vec::new()),
            next_tile_ms: 0,
            start_tile: start_tile,
            end_tile: end_tile,            
         };

        maze.set_outside_walls();
        
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
                let weak = Rc::downgrade(&rc);
                path.push(weak.clone());
                self.visit_tile(&weak);
            }
            else{
                let last_tile = path.last().unwrap().clone();
                self.visit_tile(&last_tile);
                self.extend_path();
            }

        }
    }


    fn extend_path(&mut self){
        let path = self.current_path.as_ref().unwrap();
        let mut point ;
        let mut i = path.len()-1;
        let mut next_tile: Weak<RefCell<MazeTile>> = Weak::new();
        loop{
            
            point = path.get(i).unwrap();
            let neighbors = self.check_for_tile_neighbors(&point);
            let mut possible_directions = Vec::new();
            for j in 0..4{
                if let Some(actual_tile) = &neighbors[j]{
                    let tile = actual_tile.upgrade().unwrap();
                    if !tile.borrow().visited{
                        possible_directions.push(actual_tile);
                    }
                }
            }
            if possible_directions.len() == 0{
                if i == 0{
                    self.current_path = None;

                    return;
                }
                i = i -1;
                continue;
            }

            next_tile = possible_directions[rand::thread_rng().gen_range(0..possible_directions.len())].clone();
            break;

        }

        let to_pop = path.len() - i - 1;
        for _ in 0..to_pop{
            self.current_path.as_mut().unwrap().pop();
        }

        self.current_path.as_mut().unwrap().push(next_tile);
        
    }

    //the new tile is the end point of our current path, which means we can set everything to false, except for where we just came from and the end/start point
    fn visit_tile(&mut self, tile_weak: &Weak<RefCell<MazeTile>>){


        let upgrade = tile_weak.upgrade().unwrap();
        let mut tile = upgrade.borrow_mut();
        tile.visited = true;
        let possible_directions : [Option<Weak<RefCell<MazeTile>>>; 4] = self.check_for_tile_neighbors(tile_weak);
        self.set_connection(&mut tile, &possible_directions);

        if tile_weak.ptr_eq(&self.start_tile){
            tile.connected.3 = true;   
        }
        if tile_weak.ptr_eq(&self.end_tile){
            tile.connected.1 = true;   
        }
    }

    fn set_connection(&self, tile: &mut RefMut<MazeTile>, directions: &[Option<Weak<RefCell<MazeTile>>>; 4]){

        if let Some(neighbor) = &directions[0]{
            let connected = neighbor.upgrade().unwrap().borrow().connected.2;
            tile.connected.0 = connected;
        }
        if let Some(neighbor) = &directions[1]{
            let connected = neighbor.upgrade().unwrap().borrow().connected.3;
            tile.connected.0 = connected;
        }
        if let Some(neighbor) = &directions[2]{
            let connected = neighbor.upgrade().unwrap().borrow().connected.0;
            tile.connected.0 = connected;
        }
        if let Some(neighbor) = &directions[3]{
            let connected = neighbor.upgrade().unwrap().borrow().connected.1;
            tile.connected.0 = connected;
        }
    }

    fn check_for_tile_neighbors(&self, tile_weak: &Weak<RefCell<MazeTile>>) -> [Option<Weak<RefCell<MazeTile>>>; 4]{
        let upgrade = tile_weak.upgrade().unwrap();
        let tile = upgrade.borrow();

        self.check_directions(tile)
    }

    fn check_directions(&self, tile: Ref<MazeTile>) ->  [Option<Weak<RefCell<MazeTile>>>; 4]{
        let up = self.check_direction(&tile, 0);
        let right = self.check_direction(&tile, 1);
        let down = self.check_direction(&tile, 2);
        let left = self.check_direction(&tile, 3);
        [up, right, down, left]
    }

    fn check_direction(&self, tile: &Ref<MazeTile>, direction: usize) -> Option<Weak<RefCell<MazeTile>>>{
        if direction > 3 {
            panic!("Direction must be between 0 and 3");
        }
        let (x, y) = tile.position;
        let (x, y) = match direction{
            0 => (x, y+1),
            1 => (x+1, y),
            2 => (x, y-1),
            3 => (x-1, y),
            _ => panic!("Direction must be between 0 and 3"),
        };
        let row_check = self.maze.get(x);
        if let Some(row) = row_check{
            let column_check = row.get(y);
            if let Some(tile) = column_check{
                let weak = Rc::downgrade(&tile);
                return Some(weak);
            }
        }
        None

    }


}



#[derive(Debug, Clone)]
struct MazeTile{
    position: (usize, usize),
    connected: (bool, bool, bool, bool),
    visited: bool,
    underlying_objects: Vec<Arc<RwLock<dyn DrawableObject>>>,
}