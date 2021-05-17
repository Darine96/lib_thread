extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;


use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use opengl_graphics::{ GlGraphics, OpenGL };
use glutin_window::GlutinWindow as Window;
use rand::thread_rng;
use rand::Rng;



use std::fmt::Debug;
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::sync::mpsc;
//use std::thread;
use std::vec;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::fmt;
use std::{thread, time};



mod planet;
mod vector;
mod soleil;


pub use crate::planet::planet::Planete;
pub use crate::soleil::soleil::Soleil;
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
use lib_thread::scheduler::scheduler::GLOBAL_SCHEDULER_COUNT;
use lib_thread::scheduler::scheduler::My_scheduler;
use lib_thread::signal::signal::GLOBAL_SIGNAL_COUNT;
use lib_thread::thread_trait::Thread_trait::get_condvar;

/******************************************************
                 Piston:
******************************************************/

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    color: [f32; 4],
    background: [f32; 4],
    rotation: f64,   // Rotation for the square.
    position: (f64, f64),
    vitesse: (f64, f64),
    n_planet: Vec<Planet_1>,
    color_sender: mpsc::Sender<(Vec<Planet_1>)>,
    color_receiver: mpsc::Receiver<(Vec<Planet_1>)>
}


impl App {
    
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        //println!("hello render");
        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;

        let background = self.background;
        //println!("APP {:?}\n", self.n_planet);
        let liste = self.n_planet.clone();
        //println!("hello render");
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
           clear(background, gl);
           for p in liste.iter(){
            //println!("hello render");
            let g_x = args.window_size[0] / 2.0;
            let g_y = args.window_size[1] / 2.0;
            //println!("g_x {:?} {:?}\n", g_x,g_y);
            let (mut x, mut y) = (p.position.x+ g_x, p.position.y + g_y);
            let color = p.color;
            let square = ellipse::circle(0.0, 0.0, 10.0);
            //println!("x y {:?} {:?}", x, y);
            let transform = c
                .transform
                .trans(x , y );
         ellipse(colors_marge(color), square, transform, gl);
       }

        });
    }

     /**** create a sun *****/
    // fn sun(&mut self, args: &RenderArgs) {
    //     use graphics::*;

    //     let background = self.background;
    //     let (x, y) = self.position;
    //     self.gl.draw(args.viewport(), |c, gl| {
    //         // Clear the screen.
    //        //clear(background, gl);

    //     let square = ellipse::circle(0.0, 0.0, 10.0);
    //     let (e, f) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
    
    //     let jaune = [1.0, 1.0, 0.0, 1.0];//jaune
    //     let transform = c
    //             .transform
    //             .trans(e, f);
    //      ellipse(jaune, square, transform, gl);
 
    //     });
    // }

    fn update(&mut self, args: &UpdateArgs) {

        // Rotate 2 radians per second.
        if let Ok((n_planet)) = self.color_receiver.try_recv(){
            self.n_planet = n_planet;
        }
    }

}

#[derive(Debug)]
struct window_app {
  id_thread : usize,
  scheduler: Arc<Scheduler>,
  signal: Arc<Signal>,
  color_sender: Mutex<mpsc::Sender<(Vec<Planet_1>)>>,

  signal_s1: Arc<Signal>,
  liste_planete_before: Arc<Mutex<Vec<Planet_1>>>,
  liste_planete_after: Arc<Mutex<Vec<Planet_1>>>,
  
}

impl Thread for window_app{

  fn return_scheduler(&self)-> Arc<Scheduler>{
      Arc::clone(&self.scheduler)
  }

  fn return_ID(&self) -> usize{
      return self.id_thread
  }

  fn return_nb_thread(&self) -> isize{
    *self.return_scheduler().NB_thread.lock().unwrap()
  }
    
   fn thread_execute(&mut self){

    //generer S1
   let mut i= 0;
        loop{
          //thread::sleep(time::Duration::from_secs(1));
          //wait signal
           self.thread_await(self.signal.clone(), window_app::is_here, window_app::is_not_here);
           
           //tous les planètes emittent leurs positions,
          {
           self.liste_planete_after.lock().unwrap().clear();
          }

           {
            *(self.liste_planete_after.lock().unwrap()) = self.liste_planete_before.lock().unwrap().clone();
         }


           {
            self.liste_planete_before.lock().unwrap().clear();
           }

           self.thread_emit(self.signal_s1.clone());//pour reveiller les planètes
          //send les positions couleur et vitesse au window
          let liste: Vec<Planet_1> = self.liste_planete_after.lock().unwrap().clone();
          //for planet in liste.iter(){
            let send_result = self.color_sender.lock().unwrap().send((liste.clone()));
            if let Err(send_err) = send_result {
            println!("Error on sending colors back to main thread: {}", send_err);
        }
          //}
      }
        
   }

  }

  impl window_app {
    // add code here
    fn new(scheduler: Arc<Scheduler>, signal: Arc<Signal>, signal_s1: Arc<Signal>, l_before: Arc<Mutex<Vec<Planet_1>>>, l_after: Arc<Mutex<Vec<Planet_1>>> , color_sender: mpsc::Sender<(Vec<Planet_1>)>)-> window_app{
      //scheduler.increment_nbthread();

      window_app{
        id_thread : 0usize,
        scheduler: scheduler,
        signal: signal,
        color_sender: Mutex::new(color_sender),

        signal_s1: signal_s1,
        liste_planete_before: l_before,
        liste_planete_after: l_after,

      }
    }

    fn is_here(){
    //println!("hello from THREAD 1 the signal is here");
  }

  fn is_not_here(){
    //println!("hello from THREAD 1 the signal is not here");
  }

  }

fn colors_marge(x: i32) -> [f32; 4]{
  let color: [f32; 4];
  match x {
    1 => color = [0.0, 1.0, 0.0, 1.0],//green
    2 => color = [0.0, 0.0, 0.0, 1.0],//black
    3 => color = [1.0, 0.0, 0.0, 1.0],//red
    4 => color = [0.0, 0.0, 1.0, 1.0],//blue
    5 => color = [1.0, 0.5, 0.0, 1.0],//orange
    6 => color = [1.0, 0.0, 0.5, 1.0],//magenta
    7 => color = [1.0, 1.0, 0.0, 1.0],//jaune
    _ => color = [0.0, 0.0, 0.0, 1.0],//black
  }
  return color

}


/******************************************************
                 Main:
******************************************************/
fn main(){

  println!("Hello");

  // let mut rng = rand::thread_rng();
  // let mut h = (rng.gen_range(0,200) -100);
  // println!("Random i32: {}", h);
  // let mut s = h +600;
  //   println!("Random i32: {}", s);
  // let d = (s - 600) as f64;
  //   println!("Random f64: {}", d as f64);
   

//   /********************** Piston *****************/
   // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "spinning-square",
            [1600,1600]
        )
        .graphics_api(OpenGL::V3_2)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // // Create a new game and run it.
     let (color_sender, color_receiver) = mpsc::channel();
    let mut app = App {
        gl: GlGraphics::new(opengl),
        color: [0.0, 0.0, 0.0, 1.0],
        background: [1.0, 1.0, 1.0, 1.0],
        rotation: 0.0,
        position: (200.0, 200.0),
        vitesse: (0.0, 0.0),
        n_planet: Vec::new(),
        color_sender: color_sender.clone(),
        color_receiver: color_receiver
    };

   let mut events = Events::new(EventSettings::new());

            /**** Scheduler sche ****/
    let liste_planete_before: Arc<Mutex<Vec<Planet_1>>> = Arc::new(Mutex::new(Vec::new()));
    let liste_planete_after: Arc<Mutex<Vec<Planet_1>>> = Arc::new(Mutex::new(Vec::new()));
    let sche = Scheduler::scheduler_create();
    let arc_sche = Arc::new(sche);
    let my_sche = My_scheduler::scheduler_create(Arc::clone(&arc_sche));
    let s_1 = my_sche.scheduler_start();
    
 /********************** Signal *****************/

  let signal= Arc::new(Signal::signal_create());
  let signal_s1= Arc::new(Signal::signal_create());
  
          /**** THRAED  ****/
 let soleil = Soleil::new_random(Arc::clone(&arc_sche), Arc::clone(&signal),  Arc::clone(&signal_s1), liste_planete_before.clone());
 let s_1 = soleil.thread_run();
for i in 1 .. 50{
      let planete = Planete::new_random(Arc::clone(&arc_sche), Arc::clone(&signal), Arc::clone(&signal_s1), liste_planete_before.clone(), liste_planete_after.clone());
      let t_2 = planete.thread_run();

}


  let window_app = window_app::new(Arc::clone(&arc_sche), Arc::clone(&signal), Arc::clone(&signal_s1), liste_planete_before.clone(), liste_planete_after.clone(), color_sender.clone());
  let w = window_app.thread_run();

   
    while let Some(e) = events.next(&mut window){
        if let Some(r) = e.render_args() {
            app.render(&r);
            //app.sun(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

    }

    s_1.join().unwrap();

  println!("=_=_=_=_=_ main =_=_=_=_=_=");
  println!("NB of NB_thread {:?}", arc_sche.NB_thread.lock().unwrap());
  println!("NB of finish {:?}", arc_sche.NB_finish.lock().unwrap());
  println!("list_wait_signaux {:?}", arc_sche.list_wait_signaux.lock().unwrap().len());
  println!("NB_instant {:?}", arc_sche.NB_instant.lock().unwrap());
  println!("Next next_instant {:?}", arc_sche.next_instant.lock().unwrap());



  /********************************************************************/

    

}
