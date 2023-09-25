mod canvas;
mod color;
mod matrices;
mod ppm;
mod rays;
mod shapes;
mod tuple;
mod lights;

use crate::canvas::{Canvas, Coordinate};
use crate::color::Color;
use crate::rays::hit;
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
    println!("Creating a trajectory image: trajectory.ppm");
    let mut projectile = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25,
    };

    let garden = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };

    let mut image = Ppm::new("trajectory.ppm".to_string());
    let mut canvas = Canvas::create(900, 550);
    print_trajectory(&mut canvas, &mut projectile, garden);
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

fn generate_sphere_shadow(canvas: &mut Canvas) {
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_z = 12.0;
    let wall_size = 7.0; // Allow room for the projection and some extra space around
    let pixel_size = wall_size / canvas.width as f64; // Assume the canvas is a square
    let half = wall_size / 2.0;
    let shadow = Color::color(0.4, 0.4, 0.7);
    let shape = shapes::Sphere::new_unit_sphere();

    // Uncomment to use transformations
    // shape.transformation = Matrix::new_identity().shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0).rotate_z(f64::consts::PI / 6.0);

    // Iterate over all rows in the canvas
    for y in 0..canvas.height - 1 {
        // World y coordinates: top = +half, bottom = -half
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas.width - 1 {
            let world_x = half - pixel_size * x as f64;

            // Point on the wall that the ray targets
            let wall_point = Tuple::point(world_x, world_y, wall_z);
            let r = rays::Ray::new(ray_origin, wall_point);
            let xs = r.intersects(&shape);

            // If there is a hit, the sphere casts a 'shadow' on the wall
            match hit(&xs) {
                Some(_) => canvas.write_pixel(x, y, shadow),
                None => (),
            }
        }
    }
}

fn trace_shadow() {
    println!("Primitive ray tracing of a sphere's 'shadow' on a wall. Please wait...");
    let mut image = Ppm::new("shadow.ppm".to_string());
    let mut canvas = Canvas::create(300, 300);
    generate_sphere_shadow(&mut canvas);
    image.add_canvas(canvas);
    image.write_file();
    println!("Image saved in file: shadow.ppm");
}

fn main() {
    println!("Welcome to the simple Ray Tracer!");
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        println!("Commands:");
        println!("trajectory - Create an image of a projectile's trajectory");
        println!("clock      - Create an simple clock case with a dot for each hour");
        println!("shadow     - Primitive ray tracing of a sphere's 'shadow' on a wall");
        process::exit(1);
    });

    match config.command.as_str() {
        "trajectory" => create_trajectory(),
        "clock" => create_clock(),
        "shadow" => trace_shadow(),
        _ => println!("Unknown command '{}'", config.command.as_str()),
    }
}
