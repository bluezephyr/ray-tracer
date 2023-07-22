mod color;
mod tuple;

struct Projectile {
    position: tuple::Tuple,
    velocity: tuple::Tuple,
}

struct Environment {
    gravity: tuple::Tuple,
    wind: tuple::Tuple,
}

fn tick(environment: &Environment, projectile: &mut Projectile) {
    projectile.position = projectile.position + projectile.velocity;
    projectile.velocity = projectile.velocity + environment.gravity + environment.wind;
}

fn main() {
    println!("Welcome to the simple Ray Tracer!");

    let mut projectile = Projectile {
        position: tuple::Tuple::point(0.0, 2.0, 0.0),
        velocity: tuple::Tuple::vector(1.0, 1.0, 0.0).normalize() * 2.0,
    };

    let environment = Environment {
        gravity: tuple::Tuple::vector(0.0, -0.1, 0.0),
        wind: tuple::Tuple::vector(-0.01, 0.0, 0.0),
    };

    while projectile.position.y > 0.0 {
        tick(&environment, &mut projectile);
    }

    println!("End x position: {}", projectile.position.x);
}
