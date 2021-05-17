pub mod cell{

use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::sync::mpsc;
use std::thread;
use std::vec;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::fmt;
use rand::thread_rng;
use rand::Rng;
use std::mem;

extern crate lib_thread;
use lib_thread::scheduler;
use lib_thread::thread_trait;
use lib_thread::signal;



use lib_thread::thread_trait::Thread_trait::Thread;
use lib_thread::scheduler::scheduler::Scheduler;
use lib_thread::signal::signal::Signal;


use lib_thread::thread_trait::Thread_trait::GLOBAL_THREAD_COUNT;

#[derive(Debug, Clone)]
pub struct Cell {
    pub id_thread : usize,
    pub scheduler: Arc<Scheduler>,
    pub signal_1: Arc<Signal>,

    pub alive: bool,
    pub coords: [f64;2],
    pub neighbours: [(i32, i32); 8],

    pub signal_2: Arc<Signal>,
    pub matrix_before: Arc<Mutex<Vec<Vec<bool>>>>,
    pub matrix_after: Arc<Mutex<Vec<Vec<bool>>>>, 
}

impl Thread for Cell{

  fn return_scheduler(&self)-> Arc<Scheduler>{
      Arc::clone(&self.scheduler)
  }

  fn return_ID(&self) -> usize{
      self.id_thread
  }

  fn return_nb_thread(&self) -> isize{
    *self.return_scheduler().NB_thread.lock().unwrap()
  }
    
   fn thread_execute(&mut self){

    //generer S1
    println!("\nhello from Cell\n");

    // create a planet random here or in main
    let mut i= 0;
    let x = self.coords[0] as usize;
    let y = self.coords[1] as usize;
    //println!("sef.alive {:?}\n",self.alive );
    // caculer les voisins
    self.get_neighbours(20i32, 20i32);
    let mut i =0i32;
    loop{


        // mise ajour état dans le matrice before
        {let mut matrix = self.matrix_before.lock().unwrap();
            if(self.alive){
                //println!("alive true {:?} {:?} {:?} \n", self.alive, x, y);
                matrix[x][y] = true;   
            }
        matrix[x][y] = self.alive;
        }
       // println!("Cell matrix_before {:?}", self.alive);
        //wait le signal par le scheduler
        self.thread_await(self.signal_1.clone(), Cell::is_here, Cell::is_not_here);

        //wait le signal par le window
        self.thread_await(self.signal_2.clone(), Cell::is_here, Cell::is_not_here);



        // update etat
        self.update_etat();
        i+=1;
   }

      
      }
  }


impl Cell {
    pub fn new(sche: Arc<Scheduler>, signal1: Arc<Signal>, signal2: Arc<Signal>, m1: Arc<Mutex<Vec<Vec<bool>>>>, m2: Arc<Mutex<Vec<Vec<bool>>>>, x: f64, y: f64) -> Cell {
         let old_thread_count = GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);

        //sche.increment_nbthread();
        let mut alive: bool = if( (x == 9.0 && y == 6.0) || (x == 10.0 && y == 7.0) || (x == 11.0 && y == 7.0) || (x == 11.0 && y == 8.0) 
                                        || (x == 9.0 && y == 8.0)  || (x == 6.0 && y == 6.0) || (x == 5.0 && y == 7.0)
                                        || (x == 3.0 && y == 7.0)  || (x == 4.0 && y == 9.0) || (x == 3.0 && y == 11.0)
                                        || (x == 6.0 && y == 9.0)  || (x == 7.0 && y == 9.0)
                                        || (x == 5.0 && y == 10.0) || (x == 6.0 && y == 11.0)
                                        || (x == 5.0 && y == 12.0) || (x == 7.0 && y == 12.0)
                                        || (x == 6.0 && y == 13.0) || (x == 7.0 && y == 14.0)
                                        || (x == 9.0 && y == 13.0) || (x == 9.0 && y == 11.0)){
            true
        }
        else {
            false
        };
        
        Cell {
            id_thread: old_thread_count+1,
            scheduler: sche,
            signal_1: signal1,

            alive: alive,
            coords: [x, y],
            neighbours: [(0, 0); 8],

            signal_2: signal2,
            matrix_before: m1,
            matrix_after: m2,
        }

    }


    // pub fn count_neighbours(&mut self) -> i32{
    //     let count = 0i32;
    //     for cell in self.neighbours.iter(){
    //         if cell.alive {
    //             count +=1;
    //         }
    //     }
    //     count
    // }

    pub fn if_alive(&mut self, count: i32){
        if( self.alive == false && count == 3){
            self.alive = true;
        }
        else if( self.alive == true && (count < 2 || count > 3)){
            self.alive = false;
        }
    }

    pub fn get_neighbours(&mut self,  width: i32, height: i32){
    
        
        let mut north = (0, 0);
        let mut south = (0, 0);
        let mut east = (0, 0); 
        let mut west = (0, 0);
        let mut north_east = (0, 0);
        let mut north_west = (0, 0);
        let mut south_east = (0, 0);
        let mut south_west = (0, 0);

        let x = self.coords[0] as i32;
        let y = self.coords[1] as i32;

        if ( x > 0 && x < (width -1) && y > 0 && y < (height - 1)){
            north = (x - 1, y) ;
            south = (x + 1, y);
            east = (x , y +1);
            west = (x , y - 1);
            north_east = (x - 1, y +1);
            north_west = (x - 1, y -1);
            south_east = (x + 1, y +1);
            south_west = (x + 1, y - 1);
        }

        else if(x == 0 && y == 0) {
            north = (width -1, y);
            south = (x + 1, y);
            east = (x , y +1);
            west = (x , height - 1);
            south_east = (x + 1, y +1);
            north_east = (width -1, y+1);
            north_west = (width -1, height - 1);
            south_west = (x + 1, height - 1);
        }

        else if(x == 0 && y < (height - 1)) {
            north = (width -1 , y);
            south = (x + 1, y);
            east = (x , y +1);
            west = (x , y - 1);
            south_east = (x + 1, y +1);
            north_west = (width -1, y -1);
            north_east = (width - 1, y +1);
            south_west = (x + 1, y - 1);
        }

        else if (x == (width - 1) && y == 0){
            north = (x - 1, y) ;
            south = (0, y);
            east = (x , y +1);
            west = (x , height - 1);
            north_east = (x - 1, y +1);
            north_west = (x - 1, height -1);
            south_east = (0, y +1);
            south_west = (0, height - 1);

        }

        else if (x == (width - 1) && y == (height - 1)){
            north = (x - 1, y);
            west = (x , y - 1);
            north_west = (x - 1, y -1);
            south = (0, y);
            east = (x , 0);
            north_east = (x - 1, 0);
            south_east = (0,0);
            south_west = (0, y - 1);

        }

        else if(y == 0) {
            north = (x - 1, y);
            south = (x + 1, y);
            east = (x , y +1);
            north_east = (x - 1, y +1);
            south_east = (x + 1, y +1);
            west = (x , height - 1);
            north_west = (x - 1, height -1);
            south_west = (x + 1, height - 1);
        }

        else if( x == 0 && y == (height - 1)){
            north = (width - 1, y);
            south = (x + 1, y);
            east = (x , 0);
            west = (x , y - 1);
            north_east = (width - 1, 0);
            north_west = (width - 1, y -1);
            south_east = (x + 1, 0);
            south_west = (x + 1, y - 1);
        }
        else if (y == (height - 1)){
            north = (x - 1, y) ;
            south = (x + 1, y);
            east = (x , 0);
            west = (x , y - 1);
            north_east = (x - 1, 0);
            north_west = (x - 1, y -1);
            south_east = (x + 1, 0);
            south_west = (x + 1, y - 1);
        }

        else if (x == (width - 1)){
            north = (x - 1, y) ;
            south = (0, y);
            east = (x , y +1);
            west = (x , y - 1);
            north_east = (x - 1, y +1);
            north_west = (x - 1, y -1);
            south_east = (0, y +1);
            south_west = (0, y - 1);
        }
       // println!(" [north, south, east,west, north_east, north_west, south_east, south_west] {:?}", [north, south, east,west, north_east, north_west, south_east, south_west] );
        //self.neighbours = [(north) , (south), (east), (west), (north_east), (north_west), (south_east), (south_west)] as [(usize, usize), (usize, usize) (usize, usize), (usize, usize), (usize, usize), (usize, usize), (usize, usize), (usize, usize)];
         self.neighbours = [(north) , (south), (east), (west), (north_east), (north_west), (south_east), (south_west)];
         //println!("voisins {:?} _> {:?}\n", self.neighbours, self.coords);   
    }

    fn update_etat(&mut self){
        // selon les coordonnées dans neighbours, il fait parcourir la liste pour compter le nombre 
        //des voisins alives (vivantes)
        let mut count = 0i32;
        let matrix = self.matrix_after.lock().unwrap().clone();
        //println!("matrixxxxxxxxxxxxxxx  {:?}", matrix);
        //let mut k = 0i32;
        for cell in self.neighbours.iter(){
            let (i, j) = cell.clone();
           // let (x, y) = (i, j) as (usize, usize);
             let x = i as usize;
             let y = j as usize;
             // if(i == 20 || j == 20){
             //    println!("here {:?}", self.coords);
             // }
            //println!("xxxxx, yyyyyy {:?} {:?} \n", x, y);
            if (matrix[x][y] == true){
                count +=1;
            }

            //k+=1;
        }

        //println!("count {:?}", count);
        self.if_alive(count);

    }

    fn is_here(){
    //println!("hello from THREAD 1 the signal is here");
  }

  fn is_not_here(){
    //println!("hello from THREAD 1 the signal is not here");
  }
}


}