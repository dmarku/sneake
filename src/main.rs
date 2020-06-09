use std::collections::HashSet;
use std::ops::Neg;

use audrey::open;
use nannou::math::Rad;
use nannou::prelude::*;
use nannou_audio as audio;

fn main() {
    nannou::app(model).update(update).view(view).run();
}

struct Model {
    scale: f32,
    game: Game,
    stream: audio::Stream<Audio>,
}

// enable equality check for progress
enum Progress {
    Running,
    Failure,
    Victory,
}

enum Obstacle {
    Tower,
    Block,
    Snake,
}

struct Region {
    top: i32,
    bottom: i32,
    left: i32,
    right: i32,
}

struct Game {
    progress: Progress,
    snake: Snake,
    blocks: HashSet<Vector2<i32>>,
    towers: Vec<Tower>,
    goals: Vec<Vector2<i32>>,
    boundaries: Region,
}

struct Snake {
    head: Vector2<i32>,
    max_length: usize,
    direction: Direction,
    // the segments of the snake, except for the head
    tail: Vec<Vector2<i32>>,
}

enum TowerState {
    Charging(i8),
    Firing,
}

struct Tower {
    direction: Direction,
    interval: i8,
    position: Vector2<i32>,
    state: TowerState,
    range: u8,
}

fn update_tower(tower: &Tower) -> Tower {
    Tower {
        state: match tower.state {
            TowerState::Firing => TowerState::Charging(tower.interval - 1),
            TowerState::Charging(1) => TowerState::Firing,
            TowerState::Charging(i) => TowerState::Charging(i - 1),
        },
        ..*tower
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Audio {
    sound: Option<audrey::read::BufFileReader>,
}

// implement some static level limits
fn is_blocked(game: &Game, position: &Vector2<i32>) -> Option<Obstacle> {
    if game.towers.iter().any(|tower| tower.position == *position) {
        return Some(Obstacle::Tower);
    };

    // TODO: fix i32/f32 conversion
    if game.snake.tail.iter().any(|segment| *segment == *position) {
        return Some(Obstacle::Snake);
    };

    if game.snake.head == *position {
        return Some(Obstacle::Snake);
    }

    match is_free(game, position) {
        true => None,
        false => Some(Obstacle::Block),
    }
}

fn is_free(game: &Game, position: &Vector2<i32>) -> bool {
    let Vector2 { x, y } = *position;
    let ref boundaries = game.boundaries;
    let inside_limits =
        x > boundaries.left && x < boundaries.right && y > boundaries.top && y < boundaries.bottom;
    let blocked_by_block = game.blocks.contains(position);

    inside_limits && !blocked_by_block
}

fn create_demo_level() -> Game {
    let snake = Snake {
        head: Vector2 { x: 5, y: 5 },
        direction: Direction::Right,
        max_length: 5,
        tail: Vec::new(),
    };

    let mut blocks = HashSet::new();
    blocks.insert(Vector2 { x: 4, y: 4 });
    blocks.insert(Vector2 { x: 7, y: 7 });

    let towers = vec![
        Tower {
            direction: Direction::Down,
            position: Vector2 { x: 4, y: 7 },
            interval: 6,
            state: TowerState::Charging(2),
            range: 50,
        },
        Tower {
            direction: Direction::Left,
            position: Vector2 { x: 8, y: 8 },
            interval: 4,
            state: TowerState::Charging(3),
            range: 50,
        },
    ];

    Game {
        progress: Progress::Running,
        boundaries: Region {
            top: 2,
            bottom: 10,
            left: 2,
            right: 10,
        },
        snake,
        blocks,
        towers,
        goals: vec![Vector2 { x: 3, y: 9 }],
    }
}

/* initial model creation; this is similar to Arduino's `setup()` */
fn model(app: &App) -> Model {
    app.new_window()
        .title("Sneake")
        .size(800, 600)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let assets = app.assets_path().expect("couldn't find assets path");
    let music_file = assets.join("music").join("ObservingTheStar.wav");
    let background_music = open(music_file).expect("couldn't load background music track");

    let audio_host = audio::Host::new();
    let stream = audio_host
        .new_output_stream(Audio {
            sound: Some(background_music),
        })
        .render(render_audio)
        .build()
        .unwrap();

    Model {
        scale: 24.0,
        game: create_demo_level(),
        stream,
    }
}

fn render_audio(audio: &mut Audio, buffer: &mut audio::Buffer) {
    let len_frames = buffer.len_frames();
    let mut frame_count = 0;
    if let Some(ref mut sound) = audio.sound {
        // 2-channel floating point single precision audio?
        let file_frames = sound.frames::<[f32; 2]>().filter_map(Result::ok);

        for (frame, file_frame) in buffer.frames_mut().zip(file_frames) {
            for (sample, file_sample) in frame.iter_mut().zip(&file_frame) {
                // add sound level sample by sample
                *sample += *file_sample;
            }
            frame_count += 1;
        }

        if frame_count < len_frames {
            audio.sound = None;
        }
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for tower in &model.game.towers {
        if let TowerState::Firing = tower.state {
            let increment: Vector2<i32> = tower.direction.into();
            for d in 1..tower.range {
                let pos = tower.position + increment * (d as i32);
                match is_blocked(&model.game, &pos) {
                    Some(Obstacle::Snake) => model.game.progress = Progress::Failure,
                    Some(_) => break,
                    None => (),
                }
            }
        }
    }
}

fn map_movement(key: Key) -> Option<Direction> {
    match key {
        Key::Up => Some(Direction::Up),
        Key::Down => Some(Direction::Down),
        Key::Left => Some(Direction::Left),
        Key::Right => Some(Direction::Right),
        _ => None,
    }
}

impl<S: Neg<Output = S> + One + Zero> From<Direction> for Vector2<S> {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => Vector2::<S>::unit_y(),
            Direction::Down => -Vector2::<S>::unit_y(),
            Direction::Left => -Vector2::<S>::unit_x(),
            Direction::Right => Vector2::<S>::unit_x(),
        }
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    // reset game
    // useful if game is won, failed or stuck
    if key == Key::R {
        model.game = create_demo_level();
        return;
    }

    if matches!(model.game.progress, Progress::Running) {
        if let Some(direction) = map_movement(key) {
            let ref snake = model.game.snake;
            let new_head = snake.head + direction.into();

            if is_blocked(&model.game, &new_head).is_none() {
                /*
                snake.tail.push_front(snake.head);
                while snake.tail.len() > snake.max_length - 1 {
                    snake.tail.pop_back();
                }
                */

                let new_snake = Snake {
                    head: new_head,
                    tail: Some(snake.head)
                        .into_iter()
                        .chain(snake.tail.to_owned())
                        .take(snake.max_length - 1)
                        .collect(),
                    ..*snake
                };

                model.game.snake = new_snake;

                if model.game.goals.iter().any(|goal| *goal == new_head) {
                    model.game.progress = Progress::Victory;
                };

                model.game.towers = model.game.towers.iter().map(update_tower).collect();
            }

            model.game.snake.direction = direction;
        }
    }
}

fn draw_segment(draw: &nannou::draw::Draw, model: &Model, segment: &Vector2<i32>) {
    draw.quad()
        .xy(pt2::<f32>(segment.x as f32, segment.y as f32) * model.scale)
        .w_h(model.scale, model.scale)
        .color(DARKSLATEBLUE);
}

// helper function to convert to floating-point vectors for rendering
fn vec2f(v: &Vector2<i32>) -> Vector2<f32> {
    Vector2::new(v.x as f32, v.y as f32)
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let snake = &model.game.snake;

    draw.background().color(WHITE);

    for &goal in &model.game.goals {
        draw.ellipse()
            .xy(vec2f(&goal) * model.scale)
            .w_h(0.5 * model.scale, 0.5 * model.scale)
            .no_fill()
            .stroke_color(MEDIUMSLATEBLUE);
    }

    for segment in &snake.tail {
        draw_segment(&draw, model, segment);
    }

    let pos: Vector2 = vec2f(&snake.head) * model.scale;

    draw.quad()
        .xy(pos)
        .w_h(model.scale, model.scale)
        .color(SLATEBLUE);

    let eye_size = 0.2 * model.scale;
    let dir: Vector2 = snake.direction.into();
    let eye_position: Vector2 = pos + dir * 0.3 * model.scale;

    draw.ellipse()
        .xy(eye_position)
        .w_h(eye_size, eye_size)
        .color(WHITE);

    // those are kind of arbitrary values for boundary limits
    for x in -20..20 {
        for y in -20..20 {
            if !is_free(&model.game, &Vector2 { x, y }) {
                draw.quad()
                    .x_y(x as f32 * model.scale, y as f32 * model.scale)
                    .w_h(model.scale, model.scale)
                    .color(LIGHTGRAY);
            }
        }
    }

    for tower in &model.game.towers {
        let position = vec2f(&tower.position);
        let direction: Vector2 = tower.direction.into();

        // draw base
        draw.quad()
            .xy(position * model.scale)
            .w_h(model.scale, model.scale)
            .color(DARKGREY);

        let direction_indicator_position = position * model.scale + direction * model.scale * 0.5;

        // draw charge indicator
        for i in 1..tower.interval {
            let angle = Rad::full_turn() * (i as f32 / (tower.interval - 1) as f32);

            let color = match tower.state {
                TowerState::Firing => ORANGE,
                TowerState::Charging(turns) if i < turns => BLACK,
                _ => WHITE,
            };

            draw.quad()
                .xy((position + Vector2::from_angle(angle.0) * 0.3) * model.scale)
                .w_h(model.scale * 0.1, model.scale * 0.1)
                .color(color);
        }

        // draw laser if tower is firing
        match tower.state {
            TowerState::Firing => {
                let increment: Vector2<i32> = tower.direction.into();
                let tile_size = 1.0;
                let beam_width = 0.1;

                let size = match tower.direction {
                    Direction::Up | Direction::Down => Vector2 {
                        x: beam_width,
                        y: tile_size,
                    },
                    Direction::Left | Direction::Right => Vector2 {
                        x: tile_size,
                        y: beam_width,
                    },
                };

                // limit laser range because I'm afraid of shooting in an unblocked line at some point,
                // triggering an infinite loop
                for d in 1..tower.range {
                    let pos = tower.position + increment * (d as i32);
                    if is_blocked(&model.game, &pos).is_some() {
                        break;
                    }

                    draw.quad()
                        .xy(vec2f(&pos) * model.scale)
                        .wh(size * model.scale)
                        .color(ORANGE);
                }
            }
            _ => (),
        }

        draw.quad()
            .xy(direction_indicator_position)
            .w_h(0.2 * model.scale, 0.2 * model.scale)
            .color(ORANGE);
    }

    let w = app.window_rect().w();

    let failure_bg = Srgba::new(0.5, 0.0, 0.0, 0.5);
    let victory_bg = Srgba::new(0.0, 0.5, 0.0, 0.5);

    match model.game.progress {
        Progress::Failure => {
            draw.quad().w_h(w, 300.0).x_y(0.0, 0.0).color(failure_bg);
            draw.text("You Lost!").font_size(64).color(WHITE);
        }
        Progress::Victory => {
            draw.quad().w_h(w, 300.0).x_y(0.0, 0.0).color(victory_bg);
            draw.text("You Won!").font_size(64).color(WHITE);
        }
        Progress::Running => (),
    };

    draw.to_frame(app, &frame).unwrap();
}
