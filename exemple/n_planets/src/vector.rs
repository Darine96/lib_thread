pub mod vector{
pub extern crate num;

use std::ops;
use rand::thread_rng;
use rand::Rng;


#[derive(Debug, Clone, Copy)]
pub struct Planet_1 {
    pub id_thread : usize,
    pub color: i32,
    pub position: Vec2,
    pub mass: f64,
    pub vitesse: Vec2,
    pub accel: Vec2,

}


#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x: x, y: y }
    }

    // pub fn new_position() -> Vec2 {
    //     Vec2 { x: rng.gen_range(5.0, 300.0), y: rng.gen_range(5.0, 300.0) }
    // }


    // pub fn new_vitesse() -> Vec2 {
    //     Vec2 { x: rng.gen_range(50.0, 100.0), y: rng.gen_range(50.0, 100.0) }
    // }

    pub fn distance_2(&self, vec: Vec2) -> f64 {
        println!(" self.x {:?}", self.x);
        println!("vec.x {:?}", vec.x);
        (self.x - vec.x).powi(2) + (self.y - vec.y).powi(2)
    }

    pub fn distance_1(&self, vec: Vec2) -> f64 {
        ((self.x - vec.x).powi(2) + (self.y - vec.y).powi(2)).sqrt()
    }

    pub fn distance_x(&self, vec: Vec2) -> f64 {
        vec.x - self.x
    }

    pub fn distance_y(&self, vec: Vec2) -> f64 {
        vec.y - self.y 
    }
}


}