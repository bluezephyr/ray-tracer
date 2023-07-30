mod canvas;
mod color;
mod ppm;
mod tuple;

use crate::color::Color;
use tuple::Tuple;
use ppm::Ppm;
use canvas::Canvas;

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

fn main() {
    println!("Welcome to the simple Ray Tracer!");

    let mut projectile = Projectile {
        position: Tuple::point(0.0, 2.0, 0.0),
        velocity: Tuple::vector(1.0, 1.0, 0.0).normalize() * 2.0,
    };

    let environment = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };

    while projectile.position.y > 0.0 {
        tick(&environment, &mut projectile);
    }

    let mut canvas = Canvas::create(5, 3);
    let mut p = Ppm::new("test.ppm".to_string());
    canvas.write_pixel(0, 0, Color::color(1.5, 0.0, 0.0));
    canvas.write_pixel(2, 1, Color::color(0.0, 0.5, 0.0));
    canvas.write_pixel(2, 2, Color::color(0.5, 0.0, 0.0));
    canvas.write_pixel(4, 2, Color::color(-0.5, 0.0, 1.5));
    p.save_canvas(canvas);
    println!("'{}'", p.lines[3]);

    println!("End x position: {}", projectile.position.x);
}
