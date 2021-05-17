pub mod soleil{

use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::sync::mpsc;
use std::thread;
use std::vec;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::fmt;
use rand::thread_rng;
use rand::Rng;
use std::mem;


pub use crate::vector::vector::Vec2;
pub use crate::vector::vector::Planet_1;

extern crate lib_thread;
use lib_thread::scheduler;
use lib_thread::thread_trait;
use lib_thread::signal;



use lib_thread::thread_trait::Thread_trait::Thread;
use lib_thread::scheduler::scheduler::Scheduler;
use lib_thread::signal::signal::Signal;


use lib_thread::thread_trait::Thread_trait::GLOBAL_THREAD_COUNT;


pub static GLOBAL_NUMBER_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;


const GRAVITY: f64 = 6.67;
const dt: f64 = 0.1;

#[derive(Debug, Clone)]
pub struct Soleil {
    pub id_thread : usize,
    pub scheduler: Arc<Scheduler>,
    pub signal_1: Arc<Signal>,

    pub color: i32,
    pub position: Vec2,
    pub mass: f64,
    pub vitesse: Vec2,
    pub accel: Vec2,

    pub signal_2: Arc<Signal>,
    pub liste_planete_before: Arc<Mutex<Vec<Planet_1>>>,


}

impl Thread for Soleil{

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
    // create a sun
    let mut i= 0;
    loop{
        //emis_signal to add a planet into list
        let soleil = self.create_planete();
        
        {self.liste_planete_before.lock().unwrap().push(soleil);}
        //wait signal
        self.thread_await(self.signal_1.clone(), Soleil::is_here, Soleil::is_not_here);// emit par le scheduler

         self.thread_await(self.signal_2.clone(), Soleil::is_here, Soleil::is_not_here);// emit par le window
       //  update positions according to other positions
       self.update_position();
        
   }

      
      }
  }


impl Soleil {
    pub fn new_random(sche: Arc<Scheduler>, signal1: Arc<Signal>, signal2: Arc<Signal>, l_before: Arc<Mutex<Vec<Planet_1>>>) -> Soleil {
      //sche.increment_nbthread();
        Soleil {
            id_thread: 0,
            scheduler: sche,
            signal_1: signal1,

            color: 7,
            position: Vec2::new(0.0, 0.0),
            mass: 30000.0,
            vitesse: Vec2::new(0.0, 0.0),
            accel: Vec2::new(0.0, 0.0),

            signal_2: signal2,
            liste_planete_before: l_before,



        }

    }

    pub fn create_planete(& self) -> Planet_1{
        
        Planet_1 {
            id_thread: self.id_thread.clone(),
            color: self.color,
            position: self.position.clone(),
            mass: self.mass.clone(),
            vitesse: self.vitesse.clone(),
            accel: self.accel.clone(),

        }

    }

    pub fn update_position(&mut self){
        //println!("planet {:?}", all_planete.lock().unwrap() );
        self.accel.x = 0.0;
        self.accel.y = 0.0;
        if(self.id_thread == 0){
            self.position.x = 0.0;
            self.position.y = 0.0;
        }
    }

    fn is_here(){
    //println!("hello from THREAD 1 the signal is here");
  }

  fn is_not_here(){
    //println!("hello from THREAD 1 the signal is not here");
  }

}


}