# Development Log

## 2019-09-28

- Added: collision checks. If the snake tries to move into a grid cell that is
  impassable, it won't commit the movement and no turn will be taken.
- Added: level boundaries for collision. These are currently hard-coded and
  very small. On a side note, I think the little space would be interesting for
  very tight, limited puzzle designs.
- Added: collision blocks. Certain grid cells can be made impassable by listing
  them in the `blocks` property of the `Game` struct.
- Added: Snake self-collision. The snake can't collide with its own tail. Unlike
  classic snake, this will just be regarded as an invalid action and won't advance
  the game instead of triggering a game over.
- Added: **laser towers** as a first antagonist.
  - each laser tower occupies one grid cell and is aimed in one of the four
    cardinal directions
  - each tower has a charge up time in turns. When fully charged, for one turn,
    the tower fires a laser in a straight line in its aiming direction until it
    hits an impassable block. After that, the charge cycle begins anew.
- Added: data model and rendering for goals

## 2019-08-15

- read up on data-driven game design
  - amethyst engine
  - Katherine West's talk transcript (Chucklefish/Starbound dev)
    https://kyren.github.io/2018/09/14/rustconf-talk.html
- tried amethyst engine's pong example
  - in debug mode, audio is shit there too (music "catches up" on input/respawn)
  - release mode looks fine
- release mode works fine on **EVERYTHING** :D

## 2019-08-08

- upgraded nannou to v0.10
- further tests with audio playback reveal playback gaps when too many sounds
  are played at the same time
  - problem occurs with aubrey + nannou_audio as well as `rodio`
  - both use `cpal` for device access, so the problem might be there
  - prime suspects are either filesystem performance or threading because
    the issues occur earlier with many simultaneous sounds and when the
    game process runs in the background
- custom audio rendering requires input and output formats to match exactly
  - mono file was played at twice the speed, with samples distributed to
    both stereo output channels
  - `rodio` crate allows audio file playback with correct sampling for both
    WAV and Ogg Vorbis test files

## 2019-07-02

- got audio playback of WAV files working, with help from Nannou devs
- found out that the output stream has to be moved to the model to save it from
  destruction when leaving `model()`s scope

## 2019-06-26

- ran into first issues with ownership
- typesafe pattern `match` is _really_ nice
- the snake is now drawn, can be moved with arrow keys and its tail trails
  behind its head
- try out [`cargo-watch`](https://crates.io/crates/cargo-watch).

  - Install with `cargo install cargo-watch`
  - watches for source file changes and re-runs `cargo check` by default
  - Automatically compiles and restarts game with `cargo check -x 'run --release'`
  - ha, rebuilds on blog changes, add `--ignore '*.md'`
  - final commandline:

    ```
    cargo check -x 'run --release' --ignore '*.md'
    ```

  - decided to switch to Nannou

## 2019-06-12

- check out Nannou <https://nannou.cc>
- app loop is inspired by functional reactive programming, looks neat
- intial build takes a while since it requires _a lot_ of libraries
- draw a rectangle; not at the position I expected...

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

- move on pressing arrow keys
