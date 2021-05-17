pub mod planet{

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



const GRAVITY: f64 = 6.67;
const DT: f64 = 0.1;

#[derive(Debug, Clone)]
pub struct Planete {
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
    pub liste_planete_after: Arc<Mutex<Vec<Planet_1>>>,

}

impl Thread for Planete{

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
    println!("\nhello from planet\n");

    // create a planet random here or in main
    let mut i= 0;
    loop{
        //emis_signal to add a planet into list
        let planet = self.create_planete();
        println!("\n planet {:?}\n", self.liste_planete_before.lock().unwrap());
       
       // self.thread_emis_pos(self.signal_1.clone(), planet);
        {
            self.liste_planete_before.lock().unwrap().push(planet);
        }
        //wait signal
        self.thread_await(self.signal_1.clone(), Planete::is_here, Planete::is_not_here);//emit par le scheduler
        
        self.thread_await(self.signal_2.clone(), Planete::is_here, Planete::is_not_here);// emit par le window
    
        //println!("liste Scheduler {:?}\n", self.Scheduler.liste_planete.lock().unwrap().len());
       //  update positions according to other positions
        self.update_position();
        
   }

      
      }
  }


impl Planete {
    pub fn new_random(sche: Arc<Scheduler>, signal1: Arc<Signal>, signal2: Arc<Signal>, l_before: Arc<Mutex<Vec<Planet_1>>>, l_after: Arc<Mutex<Vec<Planet_1>>>) -> Planete {
         let mut rng = rand::thread_rng();
         let z: i32 = rng.gen_range(0,10);
         let old_thread_count = GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);

        //sche.increment_nbthread();
        
        Planete {
            id_thread: old_thread_count+1,
            scheduler: sche,
            signal_1: signal1,

            color: z,
            position: Vec2::new((rng.gen_range(0.0, 200.0) -100.0), (rng.gen_range(0.0, 200.0) -100.0)),
            //position: Vec2::new((rng.gen_range(0.0, 20.0)), (rng.gen_range(0.0, 20.0))),
            mass: 1.0,
            //mass: rng.gen_range(500.0, 1000.0),
            vitesse: Vec2::new(rng.gen_range(0.0, 100.0) - 50.0, rng.gen_range(0.0, 100.0) - 50.0),
           //vitesse: Vec2::new(10.0, -50.0),
            accel: Vec2::new(0.0, 0.0),


            signal_2: signal2,
            liste_planete_before: l_before,
            liste_planete_after: l_after,

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


    pub fn force_newton(&mut self, planete: Planet_1) -> f64{
        let d_2 = self.position.distance_2(planete.position);
        println!("d_2 {:?}", d_2);
        if d_2!= 0.0 {
           // println!("planete.mass {:?}\n",planete.mass );
            let force = (GRAVITY )*((self.mass * planete.mass) /d_2);
            //println!("force  {:?} \n", force);
            return force
        }
        return 0.0
    }

    pub fn compute_accel(&mut self, planete: Planet_1){
        let d_1 = self.position.distance_1(planete.position.clone());
        println!("d_1 {:?}", d_1);
        let force = self.force_newton(planete.clone());
        if(d_1 != 0.0){
        self.accel.x = (self.accel.x + (force*(self.position.distance_x(planete.position.clone())))/(d_1));
        self.accel.y = (self.accel.y + (force*(self.position.distance_y(planete.position.clone())))/(d_1));
         println!("acc.x {:?}\n", self.accel.x);
         println!("acc.y {:?}\n", self.accel.y);
        }
        //println!("force {:?}\n ", self.accel.x);
    }

    pub fn compute_accel_all(&mut self, all_planete: Vec<Planet_1>){
        let mut i = 0;
        for planet in all_planete.iter(){
           // println!("iiiiiiiiiiiiiiiiiiiii {:?}", i);
            if(self.id_thread != planet.id_thread){
            self.compute_accel(planet.clone());
        }
        i+=1;
        }
    }

    pub fn update_vitesse(&mut self, all_planete: Vec<Planet_1>){
        self.compute_accel_all(all_planete);
        println!("self.vitesse.x {:?}\n", self.vitesse.x);
        self.vitesse.x += (DT * self.accel.x);
        self.vitesse.y += (DT * self.accel.y);
    }

    pub fn update_position(&mut self){
        //println!("planet {:?}", all_planete.lock().unwrap() );
        let liste = self.liste_planete_after.lock().unwrap().clone();
        self.accel.x = 0.0;
        self.accel.y = 0.0;
        
        if(self.id_thread == 0){
            self.position.x = 0.0;
            self.position.y = 0.0;
        }
        else {
            self.update_vitesse(liste);
            println!("self.position {:?}\n", self.position.x);
            self.position.x += (self.vitesse.x * DT);
            self.position.y += (self.vitesse.y * DT);
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