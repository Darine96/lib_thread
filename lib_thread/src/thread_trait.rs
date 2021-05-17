pub mod Thread_trait{

use std::fmt::Debug;
use std::sync::{Arc, Condvar, Mutex, MutexGuard, Barrier};
use std::sync::mpsc;
use std::thread;
use std::vec;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::fmt;
use std::sync::atomic::AtomicIsize;


pub use crate::signal::signal::Signal;
pub use crate::scheduler::scheduler::Scheduler;

/******************************************************
                 Numbers of thread and Scheduler
******************************************************/  

pub static GLOBAL_THREAD_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;

static N_THREAD: AtomicIsize = AtomicIsize::new(2);

/******************************************************
                 Trait : Thread
******************************************************/ 
pub trait Thread:  Send + Debug + Sync {

  fn return_nb_thread(&self) -> isize;
  fn return_scheduler(&self)-> Arc<Scheduler>;
  fn return_ID(&self)-> usize;

          /***************************************/

  fn thread_emit(&self, signal: Arc<Signal>){

      if(self.return_scheduler().return_code() == true){
        if(*self.return_scheduler().NB_wait.lock().unwrap() == 0){
      if(self.return_scheduler().get_condvar_wake_up()){
          self.return_scheduler().wait_return_code();
         }
    self.return_scheduler().notify_all_wake_up();
    }
      self.return_scheduler().wait_code_signal();
    }

      if(get_condvar_bool(Arc::clone(&signal)) == false){
        self.return_scheduler().add_emit_signal(Arc::clone(&signal));
        if(self.return_scheduler().get_condvar_wake_up()){
          self.return_scheduler().wait_return_code();
        }
       // self.return_scheduler().notify_all_wake_up();
    }
      notify_thread(get_condvar(signal));
        

  }

      /********************************************/
  fn thread_await_signal(&self, signal: Arc<Signal>){

    if(self.return_scheduler().return_code() == true){
    if(*self.return_scheduler().NB_wait.lock().unwrap() == 0){
      if(self.return_scheduler().get_condvar_wake_up()){
          self.return_scheduler().wait_return_code();
         }
    self.return_scheduler().notify_all_wake_up();
    }
      self.return_scheduler().wait_code_signal();
    }


    let mut check = false;
     let condvar = signal.condition_variable.clone();
    {let ( num, cvar) = &*condvar;
  
    if(*num.lock().unwrap() == false){
      check = true;
      self.return_scheduler().add_nbwait();
      self.return_scheduler().add_wait_signal(signal);
    }
  }

    if(self.return_nb_thread() == (self.return_scheduler().return_length_list_wait() + *self.return_scheduler().NB_cooperate.lock().unwrap() + *self.return_scheduler().NB_finish.lock().unwrap()) as isize){
      if(self.return_scheduler().get_condvar_wake_up()){
          //println!("dans wait {:?}",  self.return_ID());
          self.return_scheduler().wait_return_code();
        }
        self.return_scheduler().notify_all_wake_up();
    }
      //println!("wait signal {:?}",  self.return_ID());
      let ( num, cvar) = &*condvar;
            {
            let mut start = num.lock().unwrap();

      
             while !*start {
                let current = cvar.wait(start).unwrap();
                start = current; 
            }
            if(check){
              self.return_scheduler().decrement_nbwait();
            }
         }
          
  }

  fn thread_await(& self, signal: Arc<Signal>, is_here: fn(), is_not_here: fn()){
      //wait un signal
      self.thread_await_signal(signal);

      let code = self.return_scheduler().return_code();
      let see = match code {
        false => is_here(),
        true => is_not_here(),
        _ => panic!(),
      };

    }
    
      /****************************************************/

    fn thread_cooperate(&mut self){
      if(*self.return_scheduler().NB_instant.lock().unwrap() == *self.return_scheduler().next_instant.lock().unwrap()){
            self.return_scheduler().wait_instant(); 
          }
      
      if(self.return_scheduler().return_code() == true ){
        if(*self.return_scheduler().NB_wait.lock().unwrap() == 0){
      if(self.return_scheduler().get_condvar_wake_up()){
          self.return_scheduler().wait_cooperate_wake();
         }
      self.return_scheduler().notify_all_wake_up();
     }
      self.return_scheduler().wait_code_signal(); 

          }
        
      if(self.return_scheduler().get_condvar_wake_up()){
          self.return_scheduler().wait_cooperate_wake();
        }
      self.return_scheduler().add_nbcooperate();
      self.return_scheduler().notify_all_wake_up();
      self.return_scheduler().wait_cooperate(); 
      self.return_scheduler().decrement_cooperate();
      //println!("=================finishh cooperate================> for the Thread {:?}\n", self.return_ID());
   }

   fn thread_cooperate_n(&mut self, instant: i32){
    for i in (0) ..(instant){
      self.thread_cooperate();
    }

   }
   
      /********************************************************/

    fn thread_execute(&mut self);
    fn thread_run(mut self) -> thread::JoinHandle<()> 
    where Self:'static + std::marker::Sync +  std::marker::Sized
    {

      let builder = thread::Builder::new()
    .name("Scheduler".into());
    builder.spawn(move || {
    self.return_scheduler().increment_nbthread();
    self.thread_execute();
    if(self.return_scheduler().get_condvar_wake_up()){
          self.return_scheduler().wait_finish();
        }
    self.return_scheduler().add_nbfinish();
    self.return_scheduler().notify_all_wake_up();
    //println!("\n=====================finishhh exucte=====================> {:?}\n", self.return_ID());

    }).unwrap()
    
      
  }
}
/******************************************************
                 Fonctions:
******************************************************/

pub fn get_condvar(signal: Arc<Signal>)-> Arc<(Mutex<bool>, Condvar)> {

  let condvar = signal.condition_variable.clone();
  condvar
}

fn get_condvar_bool(signal: Arc<Signal>) -> bool{
    let ( num, cvar) = &*signal.condition_variable;
    let mut start = num.lock().unwrap();
    return *start
   }

pub fn notify_thread(condvar: Arc<(Mutex<bool>, Condvar)>) {

    
      let &(ref num, ref cvar) = &*condvar;
            *num.lock().unwrap() = true;
            cvar.notify_all();
             
  }



}