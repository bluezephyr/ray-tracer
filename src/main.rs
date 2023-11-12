mod camera;
mod canvas;
mod color;
mod lights;
mod matrices;
mod ppm;
mod rays;
mod shapes;
mod tuple;
mod world;

use crate::camera::Camera;
use crate::canvas::{Canvas, Coordinate};
use crate::color::Color;
use crate::lights::{lighting, PointLight};
use crate::rays::hit;
use crate::shapes::{Normal, Sphere};
use crate::world::World;
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

    let mut image = Ppm::new(&"trajectory.ppm".to_string());
    let mut canvas = Canvas::new(900, 550);
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
    let mut image = Ppm::new(&"clock.ppm".to_string());
    let mut canvas = Canvas::new(400, 400);
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
    let shape = shapes::Sphere::new();

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
    let mut image = Ppm::new(&"shadow.ppm".to_string());
    let mut canvas = Canvas::new(300, 300);
    generate_sphere_shadow(&mut canvas);
    image.add_canvas(canvas);
    image.write_file();
    println!("Image saved in file: shadow.ppm");
}

fn generate_phong_reflection(canvas: &mut Canvas) {
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_z = 12.0;
    let wall_size = 7.0; // Allow room for the projection and some extra space around
    let pixel_size = wall_size / canvas.width as f64; // Assume the canvas is a square
    let half = wall_size / 2.0;
    let mut shape = Sphere::new();
    shape.material.color = Color::color(0.0, 0.5, 1.0);
    let light = PointLight::new(
        Tuple::point(-10.0, 10.0, -10.0),
        Color::color(1.0, 1.0, 1.0),
    );

    // Uncomment to use transformations
    // shape.transformation = Matrix::new_identity().shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0).rotate_z(f64::consts::PI / 6.0);

    // Iterate over all rows in the canvas
    for y in 0..canvas.height - 1 {
        // World y coordinates: top = +half, bottom = -half
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas.width - 1 {
            let world_x = -half + pixel_size * x as f64;

            // Point on the wall that the ray targets
            let wall_point = Tuple::point(world_x, world_y, wall_z);
            let r = rays::Ray::new(ray_origin, wall_point);
            let xs = r.intersects(&shape);

            match hit(&xs) {
                Some(xs) => {
                    let point = r.position(xs.t);
                    let normal = xs.object.normal_at(&point);
                    let eyev = -r.direction.normalize();
                    let color = lighting(&xs.object.material, &light, &point, &eyev, &normal);
                    canvas.write_pixel(x, y, color);
                }
                None => (),
            }
        }
    }
}

fn phong_reflection() {
    println!("Ray tracing using the Phong reflection model. Please wait...");
    let mut image = Ppm::new(&"sphere.ppm".to_string());
    let mut canvas = Canvas::new(300, 300);
    generate_phong_reflection(&mut canvas);
    image.add_canvas(canvas);
    image.write_file();
    println!("Image saved in file: sphere.ppm");
}

fn pre_configure_world(light_x: f64) -> World {
    let mut world = World::new();
    let light = PointLight::new(
        Tuple::point(light_x, 10.0, -10.0),
        Color::color(1.0, 1.0, 1.0),
    );
    world.lights.push(light);

    // The floor and the walls are just transformed spheres
    let mut floor = Sphere::new();
    floor.transformation = Matrix::new_identity().scale(10., 0.01, 10.);
    floor.material.color = Color::color(1., 0.9, 0.9);
    floor.material.specular = 0.;
    world.objects.push(floor);

    // Left wall
    let mut left_wall = Sphere::new();
    left_wall.transformation = Matrix::new_identity()
        .scale(10., 0.01, 10.)
        .rotate_x(f64::consts::PI / 2.)
        .rotate_y(-f64::consts::PI / 4.)
        .translate(0., 0., 5.);
    left_wall.material.color = Color::color(1., 0.9, 0.9);
    left_wall.material.specular = 0.;
    world.objects.push(left_wall);

    // Right wall
    let mut right_wall = Sphere::new();
    right_wall.transformation = Matrix::new_identity()
        .scale(10., 0.01, 10.)
        .rotate_x(f64::consts::PI / 2.)
        .rotate_y(f64::consts::PI / 4.)
        .translate(0., 0., 5.);
    right_wall.material.color = Color::color(1., 0.9, 0.9);
    right_wall.material.specular = 0.;
    world.objects.push(right_wall);

    // Large sphere in the middle: Blue and translated slightly upward
    let mut middle = Sphere::new();
    middle.transformation = Matrix::new_identity().translate(-0.5, 1.0, 0.5);
    middle.material.color = Color::color(0.0, 0.5, 1.0);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    world.objects.push(middle);

    // Smaller sphere on the right: Green
    let mut right = Sphere::new();
    right.transformation = Matrix::new_identity()
        .scale(0.5, 0.5, 0.5)
        .translate(1.5, 0.5, -0.5);
    right.material.color = Color::color(0.1, 1.0, 0.5);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;
    world.objects.push(right);

    // Smallest sphere on the left: Yellow
    let mut left = Sphere::new();
    left.transformation = Matrix::new_identity()
        .scale(0.33, 0.33, 0.33)
        .translate(-1.5, 0.33, -0.75);
    left.material.color = Color::color(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;
    world.objects.push(left);

    world
}

fn planets_world(angle: f64) -> World {
    let mut world = World::new();
    let light = PointLight::new(Tuple::point(-10., 10.0, -10.0), Color::color(1.0, 1.0, 1.0));
    world.lights.push(light);

    // Large sphere in the middle: Blue
    let mut middle = Sphere::new();
    middle.transformation = Matrix::new_identity();
    middle.material.color = Color::color(0.0, 0.5, 1.0);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    world.objects.push(middle);

    // Rotating planet
    let mut left = Sphere::new();
    let x_pos = angle.cos() * 4.;
    let z_pos = angle.sin() * 4.;
    left.transformation = Matrix::new_identity()
        .scale(0.33, 0.33, 0.33)
        .translate(x_pos, 0., z_pos);
    left.material.color = Color::color(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;
    world.objects.push(left);

    world
}
fn ray_trace_world() {
    println!("Ray tracing a pre-configured world using the Phong reflection model. Please wait...");
    let mut camera = Camera::new(600, 300, f64::consts::PI / 3.);
    camera.set_view_transformation(
        &Tuple::point(0., 1.5, -5.),
        &Tuple::point(0., 1., 0.),
        &Tuple::vector(0., 1., 0.),
    );
    let world = pre_configure_world(-10.);

    let mut image = Ppm::new(&"world.ppm".to_string());
    image.add_canvas(camera.render(&world));
    image.write_file();
    println!("Image saved in file: world.ppm");
}

fn ray_trace_planets() {
    println!("Creating an animation of planets. Please wait...");
    const FRAMES: i32 = 100;
    const NAME: &str = "planet";
    let mut camera = Camera::new(300, 150, f64::consts::PI / 3.);

    for frame in 0..FRAMES {
        let angle = f64::consts::PI * 2. / FRAMES as f64 * frame as f64;
        camera.set_view_transformation(
            &Tuple::point(0., 1.5, -8.),
            &Tuple::point(0., 0., 0.),
            &Tuple::vector(0., 1., 0.),
        );
        let world = planets_world(angle);

        let name = if frame < 10 {
            format!("{}-0{}.ppm", NAME, frame)
        } else {
            format!("{}-{}.ppm", NAME, frame)
        };

        let mut image = Ppm::new(&name);
        println!(
            "Generating image {}/{}: {} {}",
            frame,
            FRAMES - 1,
            name,
            angle
        );
        image.add_canvas(camera.render(&world));
        image.write_file();
    }
    println!("Done");
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
        println!("sphere     - First ray tracing using Phong reflection model");
        println!("world      - Create a ray traced image of a pre-configured world");
        println!("planets    - Create a ray traced animation of two planets");
        process::exit(1);
    });

    match config.command.as_str() {
        "trajectory" => create_trajectory(),
        "clock" => create_clock(),
        "shadow" => trace_shadow(),
        "sphere" => phong_reflection(),
        "world" => ray_trace_world(),
        "planets" => ray_trace_planets(),
        _ => println!("Unknown command '{}'", config.command.as_str()),
    }
}
