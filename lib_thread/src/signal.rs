pub mod signal{

use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::sync::mpsc;
use std::thread;
use std::vec;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::fmt;

/******************************************************
                 Numbers of thread and Scheduler
******************************************************/  

pub static GLOBAL_SIGNAL_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;
pub use crate::thread_trait::Thread_trait::Thread;

/******************************************************
                 Signal
******************************************************/  
#[derive(Debug)]
//#[derive(Clone)]
pub struct Signal {
  pub id: usize,
  pub condition_variable: Arc<(Mutex<bool>, Condvar)>,

 
  
}

impl Signal {
  // add code here

  pub fn signal_create() -> Signal{
    let old_signal_count = GLOBAL_SIGNAL_COUNT.fetch_add(1, Ordering::SeqCst);

    Signal{
      id: old_signal_count+1,
      condition_variable: Arc::new((Mutex::new(false), Condvar::new())),

    
    }
  }

}

/******************************************************
                 Fonctions:
******************************************************/

pub fn get_condvar(signal: Arc<Mutex<Signal>>)-> Arc<(Mutex<bool>, Condvar)> {

  let condvar = signal.lock().unwrap().condition_variable.clone();
  condvar
}

fn get_condvar_bool(signal: Arc<Mutex<Signal>>) -> bool{
    let ( num, cvar) = &*signal.lock().unwrap().condition_variable;
    let mut start = num.lock().unwrap();
    return *start
   }

pub fn notify_thread(condvar: Arc<(Mutex<bool>, Condvar)>) {

    
      let &(ref num, ref cvar) = &*condvar;
            *num.lock().unwrap() = true;
            cvar.notify_all();
             
  }

  pub fn return_condvar_false(condvar: Arc<(Mutex<bool>, Condvar)>) {

    
      let &(ref num, ref cvar) = &*condvar;
            *num.lock().unwrap() = false;
            
  }

  pub fn remove_liste(liste: Arc<Mutex<Vec<Arc<Mutex<Signal>>>>>, indice: usize){
    liste.lock().unwrap().remove(indice);
  }


}