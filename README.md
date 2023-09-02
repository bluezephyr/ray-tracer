# Ray Tracer

This repo contains a simple Ray Tracer implemented in Rust. The implementation
is based on the book [*The Ray Tracer Challenge*](http://raytracerchallenge.com/)
by Jamis Buck.

[<img src="doc/bookcover.jpg" width="30%" height="30%" />](http://raytracerchallenge.com/)]


## Current status

No ray tracing yet, but there are two commands available at the moment that use
the matrix operations and vector functions: *trajectory* and *clock*. Run the
commands using

`cargo run <command>`

The output for each command is a [PPM](https://en.wikipedia.org/wiki/Netpbm) image.
The images can be viewed (on Linux) using, for example, the command `feh`. Use:

`feh ball.ppm`

to show the 'ball.ppm' image.


### Clock

The clock command generates a simple clock case, with a dot for each hour. The
dots are generated using the `point` (0, 1, 0) on which three transformations
are applied:

* scaling (0, 100, 0)
* rotation around z (-2 * PI * hour / 12) radians
* translation (200, 200, 0)

The output is an image called 'clock.ppm'.

![Clock](doc/clock.png)

### Trajectory

A PPM image called `ball.ppm` is generated when `cargo run trajectory` is called. The
image depicts a trajectory of a ball that is sent away from position (0, 1)
with a speed defined by the normalized vector (1, 1.8, 0) in an environment
with gravity of -0.1 and a head wind of -0.01.

![Ball trajectory](doc/ball.png)

## Unit tests

All modules in the ray tracer have unit tests. To run them use

`cargo test`

To automatically run the test cases whenever a file is updated, use

`cargo watch -c -x -test`
