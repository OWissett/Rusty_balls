

pub struct AppSettings {
    pub window_height: u32,
    pub window_width: u32,
    pub number_of_balls: usize,
}

impl Clone for AppSettings {
    fn clone(&self) -> Self {
        Self { 
            window_height: self.window_height.clone(),
            window_width: self.window_width.clone(), 
            number_of_balls: self.number_of_balls.clone() 
        }
    }
}