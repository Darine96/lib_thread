pub mod scheduler{

use std::fmt::Debug;
use std::sync::{Arc, Condvar, Mutex, MutexGuard, Barrier};
use std::sync::mpsc;
use std::thread;
use std::vec;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::fmt;
use std::sync::atomic::AtomicIsize;
use std::mem;

pub use crate::thread_trait::Thread_trait::Thread;

pub use crate::signal::signal::Signal;
/******************************************************
                 Numbers of scheduler
******************************************************/  
pub static GLOBAL_SCHEDULER_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;
pub static GLOBAL_THREAD_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;

/******************************************************
                 Scheduler
******************************************************/  
#[derive(Debug)]
pub struct  Scheduler  {
  id_scheduler: usize,
  pub NB_thread: Arc<Mutex<isize>>,
  //pub list_thread: Arc<Mutex<Vec<Arc<dyn Thread >>>>,//PAs besoin
  pub list_wait_signaux: Arc<Mutex<Vec<Arc<Signal>>>>,
  pub list_emit_signaux: Arc<Mutex<Vec<Arc<Signal>>>>,

/*********************************************************/

  pub condition_cooperate: Arc<(Mutex<bool>, Condvar)>,//waiting a next instant
  pub condition_wake_up: Arc<(Mutex<bool>, Condvar)>,
  pub condition_emit: Arc<(Mutex<bool>, Condvar)>,
  pub condition_cooperate_Wake: Arc<(Mutex<bool>, Condvar)>,
  pub condition_finish: Arc<(Mutex<bool>, Condvar)>,
  pub condition_instant: Arc<(Mutex<bool>, Condvar)>,
  pub condition_code_signal: Arc<(Mutex<bool>, Condvar)>,

  pub NB_cooperate: Arc<Mutex<i32>>,
  pub NB_wait: Arc<Mutex<i32>>,
  pub NB_finish: Arc<Mutex<i32>>,
  pub NB_instant: Arc<Mutex<i32>>,//a voir sans Mutex
  pub next_instant: Arc<Mutex<i32>>, 
  pub NB_wait_cooperate: Arc<Mutex<i32>>,

  pub check_NB_wait: Arc<Mutex<bool>>,
  pub check_NB_cooperate: Arc<Mutex<bool>>,
  pub return_code: Arc<Mutex<bool>>, 
}

impl Scheduler{

  // add code here
  pub fn scheduler_create()-> Scheduler{
    let old_scheduler_count = GLOBAL_SCHEDULER_COUNT.fetch_add(1, Ordering::SeqCst);

    Scheduler{
      id_scheduler: old_scheduler_count+1,
      NB_thread: Arc::new(Mutex::new(0isize)),

      //list_thread: Arc::new(Mutex::new(Vec::new())),
      list_wait_signaux: Arc::new(Mutex::new(Vec::new())),
      list_emit_signaux: Arc::new(Mutex::new(Vec::new())),
      
      condition_cooperate: Arc::new((Mutex::new(false), Condvar::new())),
      condition_wake_up: Arc::new((Mutex::new(false), Condvar::new())),
      condition_emit: Arc::new((Mutex::new(false), Condvar::new())),
      condition_cooperate_Wake: Arc::new((Mutex::new(false), Condvar::new())),
      condition_finish: Arc::new((Mutex::new(false), Condvar::new())),
      condition_instant: Arc::new((Mutex::new(false), Condvar::new())),
      condition_code_signal: Arc::new((Mutex::new(false), Condvar::new())),

      NB_cooperate: Arc::new(Mutex::new(0i32)),
      NB_wait: Arc::new(Mutex::new(0i32)),
      NB_finish: Arc::new(Mutex::new(0i32)),
      NB_instant: Arc::new(Mutex::new(0i32)),
      next_instant: Arc::new(Mutex::new(1i32)),
      NB_wait_cooperate: Arc::new(Mutex::new(0i32)),
      
      check_NB_wait: Arc::new(Mutex::new(false)),
      check_NB_cooperate: Arc::new(Mutex::new(false)),
      return_code: Arc::new(Mutex::new(false)),
    }
  }

  pub fn increment_nbthread(&self){
    *self.NB_thread.lock().unwrap()+=1;
  }

  pub fn clear_all_liste_wait(&self){
    self.list_wait_signaux.lock().unwrap().clear();
  }

  pub fn clear_all_liste_emit(&self){
    self.list_emit_signaux.lock().unwrap().clear();
  }

  pub fn add_wait_signal(&self, signal: Arc<Signal>){
    self.list_wait_signaux.lock().unwrap().push(signal);
  }

  pub fn add_emit_signal(&self, signal: Arc<Signal>){
    self.list_emit_signaux.lock().unwrap().push(signal);
    
  }

  pub fn return_length_list_wait(&self) -> i32{
    let mut i = 0i32;
    for signal in self.list_wait_signaux.lock().unwrap().iter(){
      if(get_condvar_bool(Arc::clone(&signal)) == true){
        
        i+=1;
    }
    
  }
  let length = self.list_wait_signaux.lock().unwrap().len() as i32;
  return (length - i)

}


  pub fn return_code(&self) -> bool{
      *self.return_code.lock().unwrap()
   }


   pub fn scheduler_emit(&self, signal: Arc<Signal>){
        notify_thread(get_condvar(signal));
  }

  pub fn initialise_all_signaux_emit(&self){
    for signal in self.list_emit_signaux.lock().unwrap().iter(){
      self.return_false_signal(Arc::clone(&signal));

    }
  }

  pub fn initialise_all_signaux_wait(&self){
    for signal in self.list_wait_signaux.lock().unwrap().iter(){
      self.return_false_signal(Arc::clone(&signal));
    }
  }


  pub fn return_false_signal(&self, signal: Arc<Signal>){
    return_condvar_false(get_condvar(signal)); 
  }

  pub fn verify_return_code(&self){
    if(*self.NB_wait.lock().unwrap()== 0 &&  *self.return_code.lock().unwrap()== true){
      self.initialise_all_signaux_wait();
      self.clear_all_liste_wait();
      *self.return_code.lock().unwrap()= false;
      self.notify_return_code();
    }
  }

  pub fn wait_return_code(&self){
          let ( num, cvar) = &*self.condition_emit;
        {
            let mut start = num.lock().unwrap();
            while !*start {
                start = cvar.wait(start).unwrap();
            }
        }
  }

  pub fn add_instant(&self){
    *self.NB_instant.lock().unwrap() +=1;
  }

  pub fn decrement_cooperate(&self){
    *self.NB_cooperate.lock().unwrap() -=1;
    if (*self.NB_cooperate.lock().unwrap() == 0i32){
      if(self.get_condvar_wake_up()){
          self.wait_cooperate_wake();
        }
      self.notify_all_wake_up();
    }
  }

  pub fn decrement_wait_cooperate(&self){
   *self.NB_wait_cooperate.lock().unwrap() -=1;
  }

  pub fn add_next_instant(&self)-> bool{
    if (*self.NB_cooperate.lock().unwrap() == 0i32){
       if(*self.NB_instant.lock().unwrap() == *self.next_instant.lock().unwrap()){
        *self.next_instant.lock().unwrap() +=1;
      }
      return true;
     }
     return false;
 } 

 pub fn notify_return_code(&self){
            let ( num, cvar) = &*self.condition_code_signal;
            let mut start = num.lock().unwrap();
            *start = true;
            cvar.notify_all();
        }

  pub fn wait_code_signal(&self){

          let ( num, cvar) = &*self.condition_code_signal;
        {
            let mut start = num.lock().unwrap();
            while !*start {
               
                start = cvar.wait(start).unwrap();  
            }
        }
  }

  pub fn notify_all_cooperate(&self){
            let ( num, cvar) = &*self.condition_cooperate;
            let mut start = num.lock().unwrap();
            *start = true;
            cvar.notify_all();
        }

  pub fn wait_cooperate(&self){

          let ( num, cvar) = &*self.condition_cooperate;
        {
            let mut start = num.lock().unwrap();
            while !*start {
               
                start = cvar.wait(start).unwrap();  
            }
        }
  }

  pub fn notify_all_wake_up(&self){

            let ( num, cvar) = &*self.condition_wake_up;
            let mut start = num.lock().unwrap();
            *start = true;
            cvar.notify_all();

  }

  pub fn wait_wake_up(&self){
          let ( num, cvar) = &*self.condition_wake_up;
        {
            let mut start = num.lock().unwrap();
            while !*start {
               
                start = cvar.wait(start).unwrap();              
            }
              self.return_cooperatewake_false();
              self.return_finish_false();
              self.return_emit_false();
              self.return_instant_false();
        }
  }

  pub fn get_condvar_wake_up(&self) -> bool{
    let ( num, cvar) = &*self.condition_wake_up;
    let mut start = num.lock().unwrap();
    return *start
   }

   pub fn get_condvar_cooperateWake(&self) -> bool{
    let ( num, cvar) = &*self.condition_cooperate_Wake;
    let mut start = num.lock().unwrap();
    return *start
   }

   pub fn get_condvar_cooperate(&self) -> bool{
    let ( num, cvar) = &*self.condition_cooperate;
    let mut start = num.lock().unwrap();
    return *start
   }

   
  pub fn notify_instant(&self){

            let ( num, cvar) = &*self.condition_instant;
            let mut start = num.lock().unwrap();
            *start = true;
            cvar.notify_all();
     
  }

   pub fn wait_instant(&self){

          let ( num, cvar) = &*self.condition_instant;
        {   self.add_nbwait_cooperate();
            let mut start = num.lock().unwrap();
            while !*start {
               
                start = cvar.wait(start).unwrap();
            }
            self.decrement_wait_cooperate();
        }
  }

  pub fn get_condvar_instant(&self) -> bool{
    let ( num, cvar) = &*self.condition_instant;
    let mut start = num.lock().unwrap();
    return *start
   }


   pub fn notify_all_cooperate_wake(&self){
            let ( num, cvar) = &*self.condition_cooperate_Wake;
            let mut start = num.lock().unwrap();
            *start = true;
            cvar.notify_all();
  }

  pub fn wait_cooperate_wake(&self){

          let ( num, cvar) = &*self.condition_cooperate_Wake;
        {
            let mut start = num.lock().unwrap();
            while !*start {
               
                start = cvar.wait(start).unwrap();
               
            }
        }
  }

  pub fn notify_all_emit(&self){
            let ( num, cvar) = &*self.condition_emit;
            let mut start = num.lock().unwrap();
            *start = true;
            cvar.notify_all();
  }

  pub fn return_emit_false(&self){
    let ( num, cvar) = &*self.condition_emit;
    *num.lock().unwrap() = false;
  }

   pub fn notify_all_finish(&self){
            let ( num, cvar) = &*self.condition_finish;
            let mut start = num.lock().unwrap();
            *start = true;
            cvar.notify_all();

  }

  pub fn wait_finish(&self){

          let ( num, cvar) = &*self.condition_finish;
        {
            let mut start = num.lock().unwrap();
            while !*start {
               
                start = cvar.wait(start).unwrap();   
            }
        }
  }

  pub fn return_cooperatewake_false(&self){
    let ( num, cvar) = &*self.condition_cooperate_Wake;
    *num.lock().unwrap() = false;
  }

  pub fn return_finish_false(&self){
    let ( num, cvar) = &*self.condition_finish;
    *num.lock().unwrap() = false;
  }

  pub fn return_wakeup_false(&self){
    let ( num, cvar) = &*self.condition_wake_up;
    *num.lock().unwrap() = false;
  }

  pub fn return_cooperate_false(&self){
    let ( num, cvar) = &*self.condition_cooperate;
    *num.lock().unwrap() = false;
  }

  pub fn return_code_signal(&self){
    let ( num, cvar) = &*self.condition_code_signal;
    *num.lock().unwrap() = false;
  }

  pub fn return_instant_false(&self){
    if(*self.NB_wait_cooperate.lock().unwrap() == 0){
    let ( num, cvar) = &*self.condition_instant;
    *num.lock().unwrap() = false;
  }
  }

  pub fn add_nbcooperate(&self){

    *self.NB_cooperate.lock().unwrap() +=1;
  }

  pub fn add_nbfinish(&self){

    *self.NB_finish.lock().unwrap() +=1;
  }

  pub fn add_nbwait_cooperate(&self){

    *self.NB_wait_cooperate.lock().unwrap() +=1;
  }

  pub fn add_nbwait(&self){
    //println!("hereee add NB wait");
    *self.NB_wait.lock().unwrap() +=1;
    //println!("add_wait{:?}\n", *self.NB_wait.lock().unwrap());
  }

  pub fn decrement_nbwait(&self){

    *self.NB_wait.lock().unwrap() -=1;
  }

  // pub fn notify_thread_wait(&self){

  //   for signal in self.list_wait_signaux.lock().unwrap().iter(){
  //     if(get_condvar_bool(Arc::clone(&signal)) == false){
  //       *(self.liste_planete.lock().unwrap()) = signal.list_planete.lock().unwrap().clone();
  //       signal.list_planete.lock().unwrap().clear();
  //       self.scheduler_emis(Arc::clone(&signal));
  //   }
      
  //   }
  // }


  pub fn notify_thread_wait(&self){

    for signal in self.list_wait_signaux.lock().unwrap().iter(){
      if(get_condvar_bool(Arc::clone(&signal)) == false){
        
        self.scheduler_emit(Arc::clone(&signal));
        
    }
      
    }
  }


  pub fn thread_not_finish_execution(&self) -> bool{
   // println!("Scheduler: compare_1");
    if(*self.NB_thread.lock().unwrap() > (self.return_length_list_wait() + *self.NB_cooperate.lock().unwrap() + *self.NB_finish.lock().unwrap()) as isize){ //somme as isize converter i32 to isize
      return true
    }
    return false

  }

  pub fn thread_finish_execution(&self)-> bool{
    //println!("\nScheduler: compare_2\n");
    if(*self.NB_thread.lock().unwrap() == (self.return_length_list_wait() + *self.NB_cooperate.lock().unwrap() + *self.NB_finish.lock().unwrap()) as isize){
      if(*self.NB_cooperate.lock().unwrap() == 0 && *self.NB_wait.lock().unwrap() == 0){
        return true;
      }

      else if(*self.NB_cooperate.lock().unwrap() == 0 && *self.NB_wait.lock().unwrap() != 0){
        *self.check_NB_wait.lock().unwrap()=true;//Partie wait pure
        return false;
      }

      else if(*self.NB_cooperate.lock().unwrap() != 0 && *self.NB_wait.lock().unwrap() == 0){
        *self.check_NB_wait.lock().unwrap()=false;//Partie cooperate pure
        return false;
      }

    }
    return false;
  }


/************************************************************/
  pub fn scheduler_start(&self){
    //wait a notification from thread
    let mut done = false;
    while(done == false){
      let mut not_finish_execution = false;
      let mut finish_execution =false;
      let mut verify = false;

       if (*self.NB_cooperate.lock().unwrap() == 0i32){
        self.return_cooperate_false();
       }
      self.wait_wake_up();
      verify = self.add_next_instant();
      self.verify_return_code();
      not_finish_execution = self.thread_not_finish_execution();
      if(!not_finish_execution){
        finish_execution = self.thread_finish_execution();
      }
    if(verify == true){
      self.return_cooperate_false();
      self.notify_instant();
    }
    if(not_finish_execution){
      //println!("not_finish_execution\n");
      self.return_wakeup_false();
      self.notify_all_cooperate_wake();
      self.notify_all_emit();
      self.notify_all_finish();
    }
    else if(finish_execution == false){
      if(self.get_condvar_cooperate() == false){
        if(*self.check_NB_wait.lock().unwrap()== false){// partie cooperate pure
          self.add_instant();
          self.return_wakeup_false();
          *self.return_code.lock().unwrap()=true;
          self.return_code_signal();
          self.notify_thread_wait();
          self.notify_all_cooperate();
          self.notify_all_finish();
          self.notify_all_emit();
          self.initialise_all_signaux_emit();
          self.clear_all_liste_emit();
       }
     
        else if(*self.check_NB_wait.lock().unwrap()== true){// partie wait pure
          *self.return_code.lock().unwrap()=true;
          self.return_code_signal();
          self.notify_thread_wait();
          *self.check_NB_wait.lock().unwrap()== false;
           self.add_instant();
          self.return_wakeup_false();
          self.notify_all_emit();
          self.initialise_all_signaux_emit();
          self.clear_all_liste_emit();
      }

    }
    else {
      self.return_wakeup_false();
        self.notify_all_cooperate_wake();
        self.notify_all_finish();
    }
    }
    else if(finish_execution == true){
      done = true;
    }
  }

  }
}


/******************************************************
                 My_scheduler
******************************************************/  

#[derive(Debug)]
pub struct My_scheduler {
  pub scheduler: Arc<Scheduler>,

}

impl My_scheduler {
  // add code here
  pub fn scheduler_create(sc: Arc<Scheduler>)-> My_scheduler{

    My_scheduler{
      scheduler: sc,
    }
  }

 pub fn scheduler_start(self)-> thread::JoinHandle<()>{
  //let num: u64 = 100_000_000;
        let builder = thread::Builder::new()
    .name("Scheduler".into());
        builder.spawn(move || {
      
            self.scheduler.scheduler_start();
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

  pub fn return_condvar_false(condvar: Arc<(Mutex<bool>, Condvar)>) {

    
      let &(ref num, ref cvar) = &*condvar;
            *num.lock().unwrap() = false;
            
  }


}