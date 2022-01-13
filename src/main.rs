extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use ball::Ball;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{ResizeEvent, ResizeArgs, Window as PistonWindow};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

mod ball;
mod colors;
mod settings;

const WINDOW_HEIGHT: u32 = 200;
const WINDOW_WIDTH:  u32 = 800;
const NOB: usize = 100;         // Number Of Balls (NOB)

pub struct App {
    gl: GlGraphics,
    balls: Vec<ball::Ball>,                 
    window: Window,                         
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(colors::WHITE, gl);

            // Render all of balls
            for ball in &self.balls {

                let transform = c
                    .transform
                    .trans(ball.position[0], ball.position[1])
                    .trans(-(ball.radius / 2.0), -(ball.radius / 2.0));
            
                ellipse(ball.color, [ball.radius; 4], transform, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {

        // Update all the balls
        for ball in &mut self.balls {
            ball.update(
                args.dt,
                self.window.size().height,
                self.window.size().width
            );
        }

    }

    fn update_window(&mut self, args: &ResizeArgs) {
        // Do stuff when the window updates
    }

    fn new(opengl: OpenGL, number_of_balls: usize) -> Self {

        // Create window
        let mut window: Window = WindowSettings::new(
            "spinning-square",
            [WINDOW_WIDTH, WINDOW_HEIGHT],
        )
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

        // Create balls
        let mut balls = Vec::with_capacity(number_of_balls);
        for _ in 0..number_of_balls {
            balls.push(ball::Ball::new(WINDOW_WIDTH, WINDOW_HEIGHT));
        }

        // Create the app
        let mut app = App {
            gl: GlGraphics::new(opengl),
            balls,
            window,
        };

        app
    }

    fn run(&mut self) {
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut self.window) {
            if let Some(args) = e.render_args() {
                // Render call event
                self.render(&args)
            }
    
            if let Some(args) = e.update_args() {
                // Update call event
                self.update(&args);
            }
    
            if let Some(args) = e.resize_args() {
                // Window Resize event
                self.update_window(&args);
            }
        }
    }
}

fn main() {

    let opengl = OpenGL::V3_2;

    let mut app = App::new(opengl, NOB, );

    // Start the app
    app.run();

}