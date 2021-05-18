pub mod proie{

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


const AGE_REPRODUCTION_PROIES: i32 = 4;
const AGE_REPRODUCTION_PREDATEURS: i32 = 7;
const AGE_FAMIN: i32 = 10;
const CAPACITE: i32 = 20;


#[derive(Debug, Clone)]
pub struct ProiePredateur {
    pub id_thread : usize,
    pub scheduler: Arc<Scheduler>,
    pub signal_1: Arc<Signal>, //signal emit par le scheduler

    pub is_alive: bool, // true -> le thread réactif en cours, else fini son exécution
    pub proie_pred: i32, // 1: proie, 2: predateur, 0: vide
    pub reprod: i32, // Age de reproduction
    pub faim: i32, // taux de famine pour les predateurs( pour proie nulle)
    pub coords: [i32;2], // Age de reproduction des proies
    pub neighbours: [(i32, i32); 8], // Voisins : 8 voisins pour chaque proie

    pub signal_2: Arc<Signal>, //signal emit par le window
    pub matrix_before: Arc<Mutex<Vec<Vec<(i32, i32, i32)>>>>, // avant l'update
    pub matrix_after: Arc<Mutex<Vec<Vec<(i32, i32, i32)>>>>, // apres l'emition de signal 2
}

impl Thread for ProiePredateur{

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
   //println!("\nhello from Cell\n");

    // caculer les voisins
    self.get_neighbours(20i32, 20i32);
    let mut i =0i32;
    {
        let x = self.coords[0] as usize;
        let y = self.coords[1] as usize;

        {let mut matrix = self.matrix_before.lock().unwrap();
           
        matrix[x][y].0 = self.proie_pred;
        matrix[x][y].1 = self.reprod;
        matrix[x][y].2 = self.faim;
        }
    }
    while(self.is_alive){

        //emi le signal par le scehduler
        self.thread_await(self.signal_1.clone(), ProiePredateur::is_here, ProiePredateur::is_not_here);

        //emi le signal par le window
        self.thread_await(self.signal_2.clone(), ProiePredateur::is_here, ProiePredateur::is_not_here);
        // update etat
        self.update_status();// update matrix_before through matrix_after
        // mise ajour état dans le matrice before
        // de proie_pred, reprod, faim
        self.check_status();



   }

      
      }
  }


impl ProiePredateur {
    pub fn new(sche: Arc<Scheduler>, signal1: Arc<Signal>, signal2: Arc<Signal>, m1: Arc<Mutex<Vec<Vec<(i32, i32, i32)>>>>, m2: Arc<Mutex<Vec<Vec<(i32, i32, i32)>>>>, x: i32, y: i32) -> ProiePredateur {
         let old_thread_count = GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);

        let mut rng = rand::thread_rng();
        let proie_pred: i32 = rng.gen_range(0, 3);
        let mut reprod = 0i32;
        let mut faim = 0i32;
        let mut is_alive = false;
        if(proie_pred == 1){
            reprod = rng.gen_range(0, 5);
            is_alive = true;
        }
        else if(proie_pred == 2){
             reprod = rng.gen_range(0, 7);
             is_alive = true;
        }
        
        ProiePredateur {
            id_thread: old_thread_count+1,
            scheduler: sche,
            signal_1: signal1,

            is_alive: is_alive,
            proie_pred: proie_pred, // 1: proie, 2: predateur, 0: vide
            reprod: reprod, // Age de reproduction
            faim: faim, // taux de famine pour les predateurs( pour proie nulle)
            coords: [x, y],
            neighbours: [(0, 0); 8],

            signal_2: signal2,
            matrix_before: m1,
            matrix_after: m2,
        }

    }
    pub fn new_1(sche: Arc<Scheduler>, signal1: Arc<Signal>, signal2: Arc<Signal>, m1: Arc<Mutex<Vec<Vec<(i32, i32, i32)>>>>, m2: Arc<Mutex<Vec<Vec<(i32, i32, i32)>>>>, x: i32, y: i32, proie_pred: i32, reprod: i32, faim: i32) -> ProiePredateur {
         let old_thread_count = GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
        ProiePredateur {
            id_thread: old_thread_count+1,
            scheduler: sche,
            signal_1: signal1,

            is_alive: true,
            proie_pred: proie_pred, // 1: proie, 2: predateur, 0: vide
            reprod: reprod, // Age de reproduction
            faim: faim, // taux de famine pour les predateurs( pour proie nulle)
            coords: [x, y],
            neighbours: [(0, 0); 8],

            signal_2: signal2,
            matrix_before: m1,
            matrix_after: m2,
        }

    }

    pub fn create_new(sche: Arc<Scheduler>, signal1: Arc<Signal>, signal2: Arc<Signal>, m1: Arc<Mutex<Vec<Vec<(i32, i32, i32)>>>>, m2: Arc<Mutex<Vec<Vec<(i32, i32, i32)>>>>, x: i32, y: i32, proie_pred: i32, reprod: i32, faim: i32){
        let mut new = ProiePredateur::new_1(sche, signal1, signal2, m1, m2, x, y, proie_pred, reprod, faim);
        new.thread_run();

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
       
         self.neighbours = [(north) , (south), (east), (west), (north_east), (north_west), (south_east), (south_west)];
 
    }

    fn update_status(&mut self){
        
        let mut rng = rand::thread_rng();
       if(self.proie_pred == 1){ // 1 == c'est une proie

         let cell_vide = self.cell_neighbours_vide();
         if (cell_vide.len() != 0){
         //La proie choisit une case au hasard vers laquelle se déplacer: x, y
            let len: i32 = cell_vide.len() as i32;
            let choice: usize =  rng.gen_range(0, len) as usize;
            let(x, y) = cell_vide[choice];

             // if reprod >= reprod_proie: Naissance d'une nouvelle proie 
            if(self.reprod >= AGE_REPRODUCTION_PROIES){
                // 'proie_vie' fait apparaître une proie aux coord fournies: x, y, repro == 1 (pour lui meme aussi)
                self.proie_vie(x, y, 1, 0);
                //population ++
                //Actualise la population des proies
                //dans matrix after == 1, repro, faim == 0
                self.update_status_proie(self.coords[0], self.coords[1] ,1, 0);
                // self.reprod = 1;
                // self.faim = 0;
            }
            else {
                // si non, Déplace la proie vers x, y et incrémente sa variable interne:
                // tuer le proie à ses position et decrementer la population et matrix after 0,0,0
                self.proie_dead();
                self.coords[0] = x;
                self.coords[1] = y;
                self.reprod +=1;
                self.faim = 0;
                self.update_status_proie(self.coords[0], self.coords[1] ,self.reprod, self.faim);
                ProiePredateur::create_new(self.scheduler.clone(), self.signal_1.clone(), self.signal_2.clone(), self.matrix_before.clone(), self.matrix_after.clone(), x, y, 1, self.reprod, self.faim);

                
            }
         }

        }

        else if(self.proie_pred == 2) { // Puis on parcout les cases contenant les prédateurs:
            //     // recupere faim:
                if(self.faim >= AGE_FAMIN){ // if faim >= faim_pred ==> tue_cellule
                    self.predateur_dead();
                }

            
             else {
            //     //Récupère les cases vides adjacentes:
                let cell_proie = self.cell_neighbours_proies();
                let cell_vide = self.cell_neighbours_vide();
               
                if((cell_proie.len()!= 0 && rng.gen_range(0, 21) <= CAPACITE) || cell_vide.len() == 0){
                    
        //      // et Si il y a une proie à coté, et que le prédateur réussit à l'attraper, il prendra sa place:

                    let len: i32 = cell_proie.len()as i32;
                    let choice: usize =  rng.gen_range(0, len) as usize;
                    let(x, y) = cell_proie[choice];
                    self.predateur_dead();
                    self.coords[0] = x;
                    self.coords[1] = y;
                    self.faim =0;
                    //self.update_status_predateur(self.coords[0], self.coords[1] ,self.reprod, self.faim);
                    self.update_status_predateur(x, y ,self.reprod, 0);
                    ProiePredateur::create_new(self.scheduler.clone(), self.signal_1.clone(), self.signal_2.clone(), self.matrix_before.clone(), self.matrix_after.clone(), x, y, 2, self.reprod, self.faim);
                }

                else {// si non, il se déplace vers une case vide, en plus un predateur peut donner naisance à un nouveau née
                    if(cell_vide.len() != 0){
                    let len: i32 = cell_vide.len() as i32;
                    let choice: usize =  rng.gen_range(0, len) as usize;
                    let(x, y) = cell_vide[choice];

                    if( self.reprod >= AGE_REPRODUCTION_PREDATEURS){
                        self.predateur_vie(x, y, 1, self.faim +1);
                        self.reprod = 1;
                        self.faim +=1;
                        self.update_status_predateur(self.coords[0], self.coords[1] ,self.reprod, self.faim);
                    }
                    else {// si non, deplacer le pred  reprod ++ et faim ++ et tuer la cellule.
                        self.predateur_dead();
                        self.coords[0] = x;
                        self.coords[1] = y;
                        self.reprod +=1;
                        self.faim +=1; 
                        self.update_status_predateur(self.coords[0], self.coords[1] ,self.reprod, self.faim); 
                        ProiePredateur::create_new(self.scheduler.clone(), self.signal_1.clone(), self.signal_2.clone(), self.matrix_before.clone(), self.matrix_after.clone(), x, y, 2, self.reprod, self.faim);                      
                    }
        }
                 }
            }
        }
        else if(self.proie_pred == 0){
            self.is_alive = false;
        }

       // println!("x y {:?} {:?} {:?} afer update",self.coords[0], self.coords[1], self.proie_pred );
            
    }

    fn cell_neighbours_vide(&mut self) -> Vec<(i32, i32)>{

        //calculer les voisins vides
        let matrix = self.matrix_after.lock().unwrap().clone();
        let mut cell_vide: Vec<(i32, i32)> = vec![(0i32, 0i32); 1];
        cell_vide.remove(0);
        let mut k = 0i32;
        for cell in self.neighbours.iter(){
            let (i, j) = cell.clone();
            let x = i as usize;
            let y = j as usize;


            let (proie_pred, reprod, faim) = matrix[x][y];
            // println!("matrix xxxxxxxxxxxxxx {:?}", matrix[x][y]);
            if (proie_pred == 0){ // if la case est vide
                cell_vide.push((x as i32, y as i32));
            }
            // k+=1;
    }
   
    return cell_vide
}
 
fn cell_neighbours_proies(&mut self) -> Vec<(i32, i32)>{

        //calculer les voisins vides
        let mut matrix = self.matrix_after.lock().unwrap().clone();
        let mut cell_proie: Vec<(i32, i32)> = vec![(0i32, 0i32); 1];
        cell_proie.remove(0);

        for cell in self.neighbours.iter(){
            let (i, j) = cell.clone();
            let x = i as i32;
            let y = j as i32;


            let (proie_pred, reprod, faim) = matrix[x as usize][y as usize];
            if (proie_pred == 1){ // if la case contient une proie
                cell_proie.push((x, y));
            }
    }

    return cell_proie
}

    fn proie_dead(&mut self){
        // Pour la structure
        self.proie_pred = 0;//vide
        self.reprod = 0;
        self.faim = 0;
        self.is_alive = false;

        // pour piston (la gille)
        let x = self.coords[0] as usize;
        let y = self.coords[1] as usize;
        let mut  matrix = self.matrix_before.lock().unwrap();
        matrix[x][y].0 = 0;
        matrix[x][y].1 = 0;
        matrix[x][y].2 = 0; 
    }

    fn predateur_dead(&mut self){
        // Pour la structure
        self.proie_pred = 0;//vide
        self.reprod = 0;
        self.faim = 0;
        self.is_alive = false;

        // pour piston (la gille)
        let x = self.coords[0] as usize;
        let y = self.coords[1] as usize;
        let mut  matrix = self.matrix_before.lock().unwrap();
        matrix[x][y].0 = 0;
        matrix[x][y].1 = 0;
        matrix[x][y].2 = 0; 
    }

    fn proie_vie(&mut self, x: i32, y: i32, reprod: i32, faim: i32){
        //Pour la structure
        self.proie_pred = 1;//vide
        self.reprod = reprod;
        self.faim = faim;

        // pour piston (la gille)
        let mut matrix = self.matrix_before.lock().unwrap();
        matrix[x as usize][y as usize].0 = 1;
        matrix[x as usize][y as usize].1 = reprod;
        matrix[x as usize][y as usize].2 = faim;
        ProiePredateur::create_new(self.scheduler.clone(), self.signal_1.clone(), self.signal_2.clone(), self.matrix_before.clone(), self.matrix_after.clone(), x, y, 1, reprod, faim);

    }


    fn predateur_vie(&mut self, x: i32, y: i32, reprod: i32, faim: i32){
        // Pour la structure
        self.proie_pred = 2;//vide
        self.reprod = reprod;
        self.faim = faim;

        // pour piston (la gille)
       let mut matrix = self.matrix_before.lock().unwrap();
        matrix[x as usize][y as usize].0 = 2;
        matrix[x as usize][y as usize].1 = reprod;
        matrix[x as usize][y as usize].2 = faim; 
        ProiePredateur::create_new(self.scheduler.clone(), self.signal_1.clone(), self.signal_2.clone(), self.matrix_before.clone(), self.matrix_after.clone(), x, y, 2, reprod, faim);

    }

    fn update_status_proie(&mut self, x: i32, y: i32, reprod: i32, faim: i32){
        self.proie_pred = 1;
        self.reprod = reprod;
        self.faim = faim;
        
        // pour piston (la gille)
       let mut  matrix = self.matrix_before.lock().unwrap();
        matrix[x as usize][y as usize].0 = 1;
        matrix[x as usize][y as usize].1 = reprod;
        matrix[x as usize][y as usize].2 = faim; 
    }
    
    fn update_status_predateur(&mut self, x: i32, y: i32, reprod: i32, faim: i32){
        self.proie_pred = 2;
        self.reprod = reprod;
        self.faim = faim;

        // pour piston (la gille)
       let mut  matrix = self.matrix_before.lock().unwrap();
        matrix[x as usize][y as usize].0 = 2;
        matrix[x as usize][y as usize].1 = reprod;
        matrix[x as usize][y as usize].2 = faim; 
    }

    fn check_status(&mut self){
        let mut  matrix = self.matrix_after.lock().unwrap();
        let x = self.coords[0]  as usize;
        let y = self.coords[1] as usize;

        if(self.proie_pred != matrix[x][y].0){
            self.is_alive = false;
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