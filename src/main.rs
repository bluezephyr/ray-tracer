mod canvas;
mod color;
mod ppm;
mod tuple;

use crate::color::Color;
use canvas::{Canvas, Coordinate};
use ppm::Ppm;
use tuple::Tuple;

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn tick(environment: &Environment, projectile: &mut Projectile) {
    projectile.position = projectile.position + projectile.velocity;
    projectile.velocity = projectile.velocity + environment.gravity + environment.wind;
}

fn get_canvas_coordinate(x: f64, y: f64, height: usize) -> Coordinate {
    return Coordinate {
        x: x as usize,
        y: height - y as usize,
    };
}

fn print_trajectory(canvas: &mut Canvas, projectile: &mut Projectile, environment: Environment) {
    let white = Color::color(1.0, 1.0, 1.0);
    while projectile.position.y > 0.0 {
        tick(&environment, projectile);
        let c = get_canvas_coordinate(projectile.position.x, projectile.position.y, canvas.height);
        canvas.write_pixel(c.x, c.y, white);
    }
}

fn main() {
    println!("Welcome to the simple Ray Tracer!");

    let mut ball = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25,
    };

    let garden = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };

    let mut image = Ppm::new("ball.ppm".to_string());
    let mut canvas = Canvas::create(900, 550);
    print_trajectory(&mut canvas, &mut ball, garden);
    image.add_canvas(canvas);
    image.write_file();
}
