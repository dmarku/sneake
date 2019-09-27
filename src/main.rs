use std::collections::{HashSet, VecDeque};

use audrey::open;
use nannou::math::{Deg, Rad};
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

enum TowerState {
    Charging(i8),
    Firing,
}

struct Tower {
    direction: Direction,
    interval: i8,
    position: Vector2<i32>,
    state: TowerState,
}

fn tower_next(tower: &Tower) -> Tower {
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
fn is_passable(game: &Game, x: i32, y: i32) -> bool {
    let blocked_by_tower = game
        .towers
        .iter()
        .any(|t| x == t.position.x && y == t.position.y);

    is_free(game, x, y) && !blocked_by_tower
}

fn is_free(game: &Game, x: i32, y: i32) -> bool {
    let inside_limits = x > 2 && x < 10 && y > 2 && y < 10;
    let blocked_by_block = game.blocks.contains(&(x, y));

    inside_limits && !blocked_by_block
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
    blocks.insert((4, 4));
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

    let towers = vec![
        Tower {
            direction: Direction::Down,
            position: Vector2::new(4, 7),
            interval: 6,
            state: TowerState::Charging(2),
        },
        Tower {
            direction: Direction::Left,
            position: Vector2::new(8, 8),
            interval: 4,
            state: TowerState::Charging(3),
        },
    ];

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

fn direction_vector_int(direction: &Direction) -> Vector2<i32> {
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

        if is_passable(&model.game, head.x as i32, head.y as i32) {
            let snake = &mut model.game.snake;
            snake.tail.push_front(snake.head);
            while snake.tail.len() > snake.max_length - 1 {
                snake.tail.pop_back();
            }

            snake.direction = direction;
            snake.head = head;

            model.game.towers = model.game.towers.iter().map(tower_next).collect();
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
        .color(TEAL);

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

    for tower in model.game.towers.iter() {
        let tower_position = Vector2::new(tower.position.x as f32, tower.position.y as f32);

        draw.quad()
            .xy(tower_position * model.scale)
            .w_h(model.scale, model.scale)
            .color(DARKGREY);

        let direction_indicator_position =
            tower_position * model.scale + direction_vector(&tower.direction) * model.scale * 0.5;

        for i in 1..tower.interval {
            let angle = Deg(360.0 / tower.interval as f32 * i as f32);
            let (sin, cos) = angle.sin_cos();

            let color = match tower.state {
                TowerState::Firing => ORANGE,
                TowerState::Charging(turns) if i <= turns => WHITE,
                _ => BLACK,
            };

            draw.quad()
                .xy((tower_position + Vector2::new(0.3 * sin, 0.3 * cos)) * model.scale)
                .w_h(model.scale * 0.1, model.scale * 0.1)
                .rotate(Rad::from(angle).0)
                .color(color);
        }

        match tower.state {
            TowerState::Firing => {
                // limit laser range because I'm afraid of shooting in an unblocked line at some point,
                // triggering an infinite loop
                let max_tower_range = 50;
                let mut d = 1;
                let mut pos = tower.position + direction_vector_int(&tower.direction) * d;
                let size = match tower.direction {
                    Direction::Up | Direction::Down => Vector2::new(0.1, 1.0),
                    Direction::Left | Direction::Right => Vector2::new(1.0, 0.1),
                };

                while d < max_tower_range && is_passable(&model.game, pos.x, pos.y) {
                    draw.quad()
                        .x_y(pos.x as f32 * model.scale, pos.y as f32 * model.scale)
                        .wh(size * model.scale)
                        .color(ORANGE);
                    d = d + 1;
                    pos = tower.position + direction_vector_int(&tower.direction) * d;
                }
            }
            _ => (),
        }

        draw.quad()
            .xy(direction_indicator_position)
            .w_h(0.2 * model.scale, 0.2 * model.scale)
            .color(ORANGE);
    }

    draw.to_frame(app, &frame).unwrap();
}
