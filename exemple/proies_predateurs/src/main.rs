use deform_grid::DeformGrid;
use piston_window::*;
use drag_controller::{ DragController, Drag };


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


mod proie;
pub use crate::proie::proie::ProiePredateur;

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
    matrix_etat: Vec<Vec<(i32, i32, i32)>>,
    color_sender: mpsc::Sender<(Vec<Vec<(i32, i32, i32)>>)>,
    color_receiver: mpsc::Receiver<(Vec<Vec<(i32, i32, i32)>>)>,
}


impl App {
    
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        //println!("hello render");
        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;

        let background = self.background;
        let predateur = [1.0, 0.0, 0.0, 1.0];//red
        let vide = [1.0, 1.0, 0.0, 1.0];//jaune
        let proie = [0.0, 1.0, 0.0, 1.0];//green
        let matrix = self.matrix_etat.clone();
        //println!("hello render");
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
           clear(background, gl);
           

           let mut grid = DeformGrid::new(
        [0.0, 0.0, 640.0, 640.0],
        20, 20
    );
       let mut drag = DragController::new();
       let mut draw_grid = true;
       if draw_grid {
                // Draw grid.
                grid.draw_vertical_lines(
                    &Line::new([0.0, 0.0, 0.0, 1.0], 1.5),
                    &c.draw_state,
                    c.transform,
                    gl
                );
                grid.draw_horizontal_lines(
                    &Line::new([0.0, 0.0, 0.0, 1.0], 1.5),
                    &c.draw_state,
                    c.transform,
                    gl
                );
            }

            for i in 0 .. 20{
            	for j in 0 .. 20{
                //println!(" graphics {:?}",matrix[i][j].0 );
            		let mut colour = vide;
                if(matrix[i][j].0 == 0) {
            			colour = vide
            		}
            		else if(matrix[i][j].0 == 1){
            			colour = proie
            		}

                else if(matrix[i][j].0 == 2){
                  colour = predateur
                };
            let square = rectangle::square(0.0, 0.0, 30.0);
  
            let transform = c
                .transform
                .trans(i as f64 * 32.0 , j as f64 * 32.0);
         rectangle(colour, square, transform, gl);
         
         
         
            	}
            }
            
       

        });
    }

    fn update(&mut self, args: &UpdateArgs) {

        // Rotate 2 radians per second.
        if let Ok((matrix)) = self.color_receiver.try_recv(){
        	//println!("hi");
            self.matrix_etat = matrix;
           // println!(" render in main {:?}", self.matrix_etat);
        }
    }

}


#[derive(Debug)]
struct window_app {
  id_thread : usize,
  scheduler: Arc<Scheduler>,
  signal: Arc<Signal>,
  color_sender: Mutex<mpsc::Sender<(Vec<Vec<(i32, i32, i32)>>)>>,
  signal_1: Arc<Signal>,
  matrix_before_update: Arc<Mutex<Vec<Vec<(i32, i32, i32)>>>>,
  matrix_after_update: Arc<Mutex<Vec<Vec<(i32, i32, i32)>>>>,
  
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
          thread::sleep(time::Duration::from_secs(1));
          //wait signal
           self.thread_await(self.signal.clone(), window_app::is_here, window_app::is_not_here);

           self.matrix_after_update.lock().unwrap().clear();
           *(self.matrix_after_update.lock().unwrap()) = self.matrix_before_update.lock().unwrap().clone();
            //println!("window matrix_before_update{:?}\n", self.matrix_before_update.lock().unwrap());
          // println!("window matrix_after_update{:?}\n", self.matrix_after_update.lock().unwrap());
           self.thread_emit(self.signal_1.clone());//pour reveiller les cellules
          //send les états au window
           let matrix = self.matrix_after_update.lock().unwrap().clone();
         	//println!("window in main {:?}\n", matrix);
         	//for cell in matrix.iter(){
         		let send_result = self.color_sender.lock().unwrap().send((matrix.clone()));
            	if let Err(send_err) = send_result {
            	println!("Error on sending colors back to main thread: {}", send_err);
        		}
         	//}
          
          
      }
        
   }

  }

  impl window_app {
    // add code here
    fn new(scheduler: Arc<Scheduler>, signal: Arc<Signal>, signal_1: Arc<Signal>, m1: Arc<Mutex<Vec<Vec<(i32, i32, i32)>>>>, m2: Arc<Mutex<Vec<Vec<(i32, i32, i32)>>>> ,color_sender: mpsc::Sender<(Vec<Vec<(i32, i32, i32)>>)>)-> window_app{
     // scheduler.increment_nbthread();

      window_app{
        id_thread : 0usize,
        scheduler: scheduler,
        signal: signal,
        color_sender: Mutex::new(color_sender),
        signal_1: signal_1,
  		matrix_before_update: m1,
  		matrix_after_update: m2,

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
 color = [1.0, 1.0, 0.0, 1.0];//jaune

  return color

}


/******************************************************
                 Main:
******************************************************/
fn main(){

  println!("Hello");
   /********************** Piston *****************/
  // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "spinning-square",
            [1200, 1200]
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
        matrix_etat: vec![vec![(0i32, 0i32, 0i32); 20]; 20],
    	color_sender: color_sender.clone(),
    	color_receiver: color_receiver,


    };

   let mut events = Events::new(EventSettings::new());

   

            /**** Scheduler sche ****/
    let mut matrix_1: Arc<Mutex<Vec<Vec<(i32, i32, i32)>>>> = Arc::new(Mutex::new(vec![vec![(0i32, 0i32, 0i32); 20]; 20]));    
    let mut matrix_2: Arc<Mutex<Vec<Vec<(i32, i32, i32)>>>> = Arc::new(Mutex::new(vec![vec![(0i32, 0i32, 0i32); 20]; 20]));  
    let sche = Scheduler::scheduler_create();
    let arc_sche = Arc::new(sche);
    let my_sche = My_scheduler::scheduler_create(Arc::clone(&arc_sche));
    

    let signal= Arc::new(Signal::signal_create());
    let signal_1= Arc::new(Signal::signal_create());
    
    for i in 0..20{
    	for j in 0..20{
    		let cell = ProiePredateur::new(Arc::clone(&arc_sche), Arc::clone(&signal), Arc::clone(&signal_1), matrix_1.clone(), matrix_2.clone(),  i as i32, j as i32);
    		let index = cell.is_alive;
        if(index){
    		let cell_2 = cell.thread_run();
      }
    		 //cell.thread_emis_etat(cell.signal_1.clone(), i as usize, j as usize, cell.alive);
    	}
    }
    //println!("sss {:?}", signal.matrix_etat.lock().unwrap());
    
   

    let window_app = window_app::new(Arc::clone(&arc_sche), Arc::clone(&signal), Arc::clone(&signal_1), matrix_1.clone(), matrix_2.clone(), color_sender.clone());
    let w = window_app.thread_run();

     let s_1 = my_sche.scheduler_start();
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
 	println!("nombre thread {:?}", arc_sche.NB_thread.lock().unwrap());
 	

  /********************************************************************/

    

}
