use std::collections::{HashSet, VecDeque};

use audrey::open;
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

struct Game {
    snake: Snake,
    blocks: HashSet<(i32, i32)>,
    towers: Vec<Tower>,
}

struct Snake {
    head: Vector2,
    max_length: usize,
    direction: Direction,
    // the segments of the snake, except for the head
    tail: VecDeque<Vector2>,
}

struct Tower {
    direction: Direction,
    interval: i8,
    position: Vector2<i32>,
    countdown: i8,
}

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
fn is_free(game: &Game, x: i32, y: i32) -> bool {
    let inside_limits = x > 2 && x < 10 && y > 2 && y < 10;
    let blocked_by_block = game.blocks.contains(&(x, y));
    let blocked_by_tower = game
        .towers
        .iter()
        .any(|t| x == t.position.x && y == t.position.y);

    inside_limits && !blocked_by_block && !blocked_by_tower
}

/* initial model creation; this is similar to Arduino's `setup()` */
fn model(app: &App) -> Model {
    app.new_window()
        .with_dimensions(800, 600)
        .with_title("Sneake")
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let snake = Snake {
        head: Vector2::new(5.0, 5.0),
        direction: Direction::Right,
        max_length: 3,
        tail: VecDeque::with_capacity(20),
    };

    let mut blocks = HashSet::new();
    blocks.insert((3, 3));
    blocks.insert((7, 7));

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

    let towers = vec![Tower {
        direction: Direction::Up,
        position: Vector2::new(4, 7),
        interval: 6,
        countdown: 5,
    }];

    Model {
        scale: 24.0,
        game: Game {
            snake,
            blocks,
            towers,
        },
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

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn map_movement(key: Key) -> Option<Direction> {
    match key {
        Key::Up => Some(Direction::Up),
        Key::Down => Some(Direction::Down),
        Key::Left => Some(Direction::Left),
        Key::Right => Some(Direction::Right),
        _ => None,
    }
}

fn direction_vector(direction: &Direction) -> Vector2 {
    match direction {
        Direction::Up => Vector2::unit_y(),
        Direction::Down => -Vector2::unit_y(),
        Direction::Left => -Vector2::unit_x(),
        Direction::Right => Vector2::unit_x(),
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    if let Some(direction) = map_movement(key) {
        let snake = &model.game.snake;
        let head = snake.head + direction_vector(&direction);

        if is_free(&model.game, head.x as i32, head.y as i32) {
            let snake = &mut model.game.snake;
            snake.tail.push_front(snake.head);
            while snake.tail.len() > snake.max_length - 1 {
                snake.tail.pop_back();
            }

            snake.direction = direction;
            snake.head = head;
        }
    }
}

fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();
    let snake = &model.game.snake;

    draw.background().color(DARKBLUE);

    for &segment in snake.tail.iter() {
        draw.quad()
            .xy(segment * model.scale)
            .w_h(model.scale, model.scale)
            .color(GRAY);
    }

    let pos = snake.head * model.scale;

    draw.quad()
        .xy(pos)
        .w_h(model.scale, model.scale)
        .color(WHITE);

    let eye_size = 0.2 * model.scale;
    let eye_direction = direction_vector(&snake.direction);

    draw.quad()
        .xy(pos + eye_direction * 0.3 * model.scale)
        .w_h(eye_size, eye_size)
        .color(RED);

    for x in -20..20 {
        for y in -20..20 {
            if !is_free(&model.game, x, y) {
                draw.quad()
                    .x_y(x as f32 * model.scale, y as f32 * model.scale)
                    .w_h(model.scale, model.scale)
                    .color(BLACK);
            }
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
