# Development Log

## 2019-06-03

- goal: rebuild snake game in Rust and Piston
- initialized hello world example with `cargo init`, test build with `cargo run`
- first error: piston docs mention old version for dependencies
  - found current version with `cargo search piston_window` -> 0.94.0
  - updated build downloads & compiles _a lot_ of crates
