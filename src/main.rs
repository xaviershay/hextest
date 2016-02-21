extern crate piston;
extern crate piston_window;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate hex2d;
extern crate rand;

use hex2d::*;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use opengl_graphics::{ GlGraphics, OpenGL };

use std::collections::HashSet;
use std::collections::LinkedList;

use graphics::line::*;

use sdl2_window::Sdl2Window;
use piston_window::PistonWindow;

use rand::Rng;

pub type GameWindow = PistonWindow<(), Sdl2Window>;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,   // Rotation for the square.
    coordinates: Vec<Coordinate>,
}

const POLYGON: &'static [[f64; 2]] = &[
    [0.0, -8.0],
    [20.0, 0.0],
    [0.0, 8.0]
];

const HEX: &'static [[f64; 2]] = &[
    [1.0000, 0.0000],
    [0.5000, 0.8660],
    [-0.5000, 0.8660],
    [-1.0000, 0.0000],
    [-0.5000, -0.8660],
    [0.5000, -0.8660],
    [1.0000, -0.0000],
];

impl App {
    fn new(gl: GlGraphics) -> Self {
      let root = Coordinate::new(0, 0);
      let mut tried = HashSet::new();
      let mut candidates : LinkedList<Coordinate> = LinkedList::new();
      candidates.push_back(root);
      let mut coords: Vec<Coordinate> = vec!(root);
      let mut rng = rand::thread_rng();
      let mut x = 0;

      loop {
        x += 1;
        if candidates.is_empty() {
            break;
        }
        let candidate = candidates.pop_front().unwrap();
        if tried.contains(&candidate) {
            continue;
        }
        tried.insert(candidate);
        if candidate == root || rng.gen::<f32>() < 0.7 {
          coords.push(candidate);
          candidates.extend(candidate.neighbors().iter().cloned());
        }
        if x > 100 {
          break;
        }
      }

      App {
        gl: gl,
        rotation: 0.0,
        coordinates: coords,
      }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [0.9, 0.9, 0.9, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const GREY:  [f32; 4] = [0.5, 0.5, 0.5, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);

        let t = 1.0;
        let arc = 3.14; // radians
        let z = 0.2;

        let mut dt = self.rotation;
        if dt >= t {
          dt = t;
        }

        let ref coords = self.coordinates;
        let r = -arc + (dt / t) * arc;
        let z = z + (dt / t) * (1.0 - z);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            for p in coords {
                let coords = p.to_pixel(Spacing::FlatTop(1.0));

                let transform = c.transform
                  .trans(x, y)
                  .rot_rad(r)
                  .scale(z, z)
                  .scale(30.0, 30.0)
                  .trans(coords.0 as f64, coords.1 as f64)
                  .scale(0.9, 0.9)
                  ;

                // Draw a box rotating around the middle of the screen.
                //rectangle(RED, square, transform, gl);
                polygon(GREY, HEX, transform, gl);
                let mut hex_coords : Vec<_> = From::from(HEX);
                let head = hex_coords[0].clone();
                hex_coords.push(head);
                for (p1, p2) in hex_coords.iter().zip(hex_coords.iter().skip(1)) {
                  Line::new(WHITE, 0.02).shape(Shape::Bevel).draw([p1[0], p1[1], p2[0], p2[1]], &DrawState::new(), transform, gl);
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += args.dt;
        //self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;
    let mut window: GameWindow =
        WindowSettings::new("Hextest", (600, 600))
        .samples(4)
        .exit_on_esc(true)
        .build()
        .unwrap();

/*
    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "spinning-square",
            [200, 200]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
*/

    // Create a new game and run it.
    let mut app = App::new(GlGraphics::new(opengl));

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
