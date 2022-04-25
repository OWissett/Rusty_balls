
use super::colors;
use super::*;

use rand::Rng;

pub struct Ball {
    pub position: [f64; 2],
    velocity: [f64; 2],
    pub color: [f32; 4] ,
    pub radius: f64,
}

impl Ball {
    pub fn new(window_width: u32, window_height: u32) -> Self {

        let mut rng = rand::thread_rng();
        Ball { 
           position: [
               rng.gen_range(0.0..window_width as f64), 
               rng.gen_range(0.0..window_height as f64)
               ], 
           velocity: [100.0, 100.0], 
           color: colors::BLACK,
           radius: 10.0,
       }

    }


    pub fn update(&mut self, dt: f64, window_height: f64, window_width: f64) {

        self.move_ball(dt, window_height, window_width);

        
    }

    fn move_ball(&mut self, dt: f64, window_height: f64, window_width: f64) {
        if (self.position[0] <= self.radius)  ||
           (self.position[0] >= (window_width - self.radius)) {
            self.velocity[0] = -self.velocity[0]; 
            //println!("Square hit the side! square position {:?}", self.position);
        }

        // y boundary
        if (self.position[1] <= self.radius)  || 
           (self.position[1] >= window_height - self.radius) {
            self.velocity[1] = -self.velocity[1]; 
            //println!("Square hit the side! square position {:?}", self.position);
        }

        // Update position based on velocity
        self.position[0] += self.velocity[0] * dt; // Update x by velocity * time.
        self.position[1] += self.velocity[1] * dt; // update y
    }
}

impl Clone for Ball {
    fn clone(&self) -> Self {
        Self { 
            position: self.position.clone(), 
            velocity: self.velocity.clone(),
            color: self.color.clone(), 
            radius: self.radius.clone(),
        }
    }
}
