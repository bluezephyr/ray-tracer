# Ray Tracer

A simple Ray Tracer implemented in Rust

## Current status

A PPM image called `test.ppm` is generated when `cargo run` is called. The
image can be viewed (on Linux) using, for example, the following command:

`feh test.ppm -R 2 -g 200x200 --zoom 2000`

The image contains just a few pixels on a canvas. No ray tracing yet...

## Unit tests

All modules in the ray tracer have unit tests. To run them use

`cargo test`

To automatically run the test cases whenever a file is updated, use

`cargo watch -c -x -test`
