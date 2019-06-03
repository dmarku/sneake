# Development Log

## 2019-06-03

- goal: rebuild snake game in Rust and Piston
- initialized hello world example with `cargo init`, test build with `cargo run`
- first error: piston docs mention old version for dependencies

  - found current version with `cargo search piston_window` -> 0.94.0
  - updated build downloads & compiles _a lot_ of crates

- follow (a piston "getting started" tutorial")[https://github.com/PistonDevelopers/Piston-Tutorials/tree/master/getting-started]

  - the example code is _very_ different to piston's repository readme example
  - learned about piston's modular architecture
  - learned that glutin is a portable OpenGL context creation library in pure Rust, similar to glut
  - learned that piston's graphics interface can be implemented with different backends

- handle mouse button presses
