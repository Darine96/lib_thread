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
    pub signal1: Arc<Signal>,
    pub length:   Arc<Mutex<i32>>,

   
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
    for i in 0..5{
        *self.length.lock().unwrap() +=1;
        println!("producer length {:?}", self.length.lock().unwrap());
        self.thread_emit(self.signal1.clone());
    }
      
      }
  }


impl Reactif_Thread1 {
    pub fn new(sche: Arc<Scheduler>, s1:  Arc<Signal>, len: Arc<Mutex<i32>>) -> Reactif_Thread1 {
         let old_thread_count = GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
         Reactif_Thread1{
         	id_thread: old_thread_count+1,
            scheduler: sche,
            signal1: s1,
            length: len,
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
    pub signal1: Arc<Signal>,
    pub length:   Arc<Mutex<i32>>,

   
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
    let mut i =0;
    for i in 0..5{
        if(*self.length.lock().unwrap() == 0){
          println!("consumer wainting ...\n");
          self.thread_await(self.signal1.clone(), Reactif_Thread2::is_here, Reactif_Thread2::is_not_here);
        }
        
        *self.length.lock().unwrap() -=1;
        println!("consumer {:?}", *self.length.lock().unwrap());
    }
      
      }
  }


impl Reactif_Thread2 {
    pub fn new(sche: Arc<Scheduler>, s1:  Arc<Signal>, len: Arc<Mutex<i32>>) -> Reactif_Thread2 {
         let old_thread_count = GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
         Reactif_Thread2{
          id_thread: old_thread_count+1,
            scheduler: sche,
            signal1: s1,
            length: len,
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
    let signal= Arc::new(Signal::signal_create());
    let signal_1= Arc::new(Signal::signal_create());
    let s_1 = my_sche.scheduler_start();
    let length = Arc::new(Mutex::new(0i32));
    // s_1.join().unwrap();

    /*** Reactif_Thread ***/
    let th1 = Reactif_Thread1::new(Arc::clone(&arc_sche), Arc::clone(&signal), Arc::clone(&length));
    let th2 = Reactif_Thread2::new(Arc::clone(&arc_sche), Arc::clone(&signal), Arc::clone(&length));

    th1.thread_run();
    th2.thread_run();
    
    s_1.join().unwrap();
}