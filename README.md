# Ray Tracer

A simple Ray Tracer implemented in Rust, but no ray tracing yet..

## Current status

There are two commands available at the moment: 'trajectory' and 'clock'. Run
the commands using

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
