use std::fmt::Debug;
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::sync::mpsc;
//use std::thread;
use std::vec;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::fmt;
use std::{thread, time};

use std::process;



extern crate lib_thread;
use lib_thread::scheduler;
use lib_thread::thread_trait;
use lib_thread::signal;



use lib_thread::thread_trait::Thread_trait::Thread;
use lib_thread::scheduler::scheduler::Scheduler;
use lib_thread::signal::signal::Signal;


use lib_thread::thread_trait::Thread_trait::GLOBAL_THREAD_COUNT;
use lib_thread::scheduler::scheduler::GLOBAL_SCHEDULER_COUNT;
use lib_thread::scheduler::scheduler::My_scheduler;
use lib_thread::signal::signal::GLOBAL_SIGNAL_COUNT;
use lib_thread::thread_trait::Thread_trait::get_condvar;

#[derive(Debug, Clone)]
pub struct Reactif_Thread1 {
    pub id_thread : usize,
    pub scheduler: Arc<Scheduler>,

   
}

impl Thread for Reactif_Thread1{

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
    let mut i =0;
    loop{
    	i+=1;
    	println!("the instance now thread 1>>>>> {:?}", i);
     	self.thread_cooperate();
     	   }

      
      }
  }


impl Reactif_Thread1 {
    pub fn new(sche: Arc<Scheduler>) -> Reactif_Thread1 {
         let old_thread_count = GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
         Reactif_Thread1{
         	id_thread: old_thread_count+1,
            scheduler: sche,
         }
        
    }

    
  fn is_here(){
    //println!("hello from THREAD 1 the signal is here");
  }

  fn is_not_here(){
    //println!("hello from THREAD 1 the signal is not here");
  }
}
								/**************************************************************/

#[derive(Debug, Clone)]
pub struct Reactif_Thread2 {
    pub id_thread : usize,
    pub scheduler: Arc<Scheduler>, 

   
}

impl Thread for Reactif_Thread2{

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
    self.thread_cooperate_n(10);
    println!("Stop");
    process::exit(0x0100);

  }
}


impl Reactif_Thread2 {
    pub fn new(sche: Arc<Scheduler>) -> Reactif_Thread2 {
         let old_thread_count = GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
         Reactif_Thread2{
         	id_thread: old_thread_count+1,
            scheduler: sche,
         }
        
    }

    
  fn is_here(){
    //println!("hello from THREAD 1 the signal is here");
  }

  fn is_not_here(){
    //println!("hello from THREAD 1 the signal is not here");
  }
}
					/***************************************************************/

#[derive(Debug, Clone)]
pub struct Reactif_Thread3 {
    pub id_thread : usize,
    pub scheduler: Arc<Scheduler>, 

   
}

impl Thread for Reactif_Thread3{

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
    let mut i =0;
    loop{
    	i+=1;
    	println!("the instance now is  thread 3 >>>>> {}", i);
    	
     	self.thread_cooperate();
     	   }

      
      }
  }


impl Reactif_Thread3 {
    pub fn new(sche: Arc<Scheduler>) -> Reactif_Thread3 {
         let old_thread_count = GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
         Reactif_Thread3{
         	id_thread: old_thread_count+1,
            scheduler: sche,
         }
        
    }

    
  fn is_here(){
    //println!("hello from THREAD 1 the signal is here");
  }

  fn is_not_here(){
    //println!("hello from THREAD 1 the signal is not here");
  }
}

					/****************************************************************/

fn main(){
	/*** scheduler **/
	let sche = Scheduler::scheduler_create();
    let arc_sche = Arc::new(sche);
    let my_sche = My_scheduler::scheduler_create(Arc::clone(&arc_sche));
    //let s_1 = my_sche.scheduler_start();
    // s_1.join().unwrap();

    /*** Reactif_Thread ***/
    let th1 = Reactif_Thread1::new(Arc::clone(&arc_sche));
    let th2 = Reactif_Thread2::new(Arc::clone(&arc_sche));
    let th3 = Reactif_Thread3::new(Arc::clone(&arc_sche));

    th1.thread_run();
    th2.thread_run();
    th3.thread_run();
    let s_1 = my_sche.scheduler_start();
    s_1.join().unwrap();
}