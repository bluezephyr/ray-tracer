mod canvas;
mod color;
mod matrices;
mod ppm;
mod rays;
mod tuple;
mod shapes;

use crate::canvas::{Canvas, Coordinate};
use crate::color::Color;
use matrices::{to_tuple, Matrix};
use ppm::Ppm;
use std::{env, f64, process};
use tuple::Tuple;

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

struct Config {
    command: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 2 {
            return Err("Expect <command>");
        }

        let command = args[1].clone();

        Ok(Config { command })
    }
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

fn create_trajectory() {
    println!("Creating a trajectory image: ball.ppm");
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

fn print_clock_hours(canvas: &mut Canvas) {
    let white = Color::color(1.0, 1.0, 1.0);
    canvas.write_pixel(200, 200, white);

    for hour in 0..12 {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let transform = Matrix::new_identity()
            .scale(0.0, 100.0, 0.0)
            .rotate_z(-f64::consts::PI * 2.0 * hour as f64 / 12.0)
            .translate(200.0, 200.0, 0.0);
        let dot = to_tuple(&(&transform * &p));
        let c = get_canvas_coordinate(dot.x, dot.y, canvas.height);
        canvas.write_pixel(c.x, c.y, white);
    }
}

fn create_clock() {
    println!("Creating a clock image: clock.ppm");
    let mut image = Ppm::new("clock.ppm".to_string());
    let mut canvas = Canvas::create(400, 400);
    print_clock_hours(&mut canvas);
    image.add_canvas(canvas);
    image.write_file();
}

fn main() {
    println!("Welcome to the simple Ray Tracer!");
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        println!("Commands: 'trajectory' or 'clock'");
        process::exit(1);
    });

    match config.command.as_str() {
        "trajectory" => create_trajectory(),
        "clock" => create_clock(),
        _ => println!("Unknown command"),
    }
}
