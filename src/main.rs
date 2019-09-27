use std::collections::VecDeque;

use audrey::open;
use nannou::prelude::*;
use nannou_audio as audio;

fn main() {
    nannou::app(model).update(update).view(view).run();
}

struct Model {
    scale: f32,
    snake: Snake,
    stream: audio::Stream<Audio>,
}

// implement some static level limits
fn is_free<S: PartialOrd<i32>>(x: S, y: S) -> bool {
    return x > 2 && x < 10 && y > 2 && y < 10;
}

struct Snake {
    head: Vector2,
    max_length: usize,
    direction: Direction,
    // the segments of the snake, except for the head
    tail: VecDeque<Vector2>,
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

    let assets = app.assets_path().expect("couldn't find assets path");
    let music_file = assets.join("music").join("ObservingTheStar.wav");
    let background_music = open(music_file).expect("couldn't load background music track");

    let audio_host = audio::Host::new();
    let stream = audio_host
        .new_output_stream(
            Audio {
                sound: Some(background_music),
            },
        )
        .render(render_audio)
        .build()
        .unwrap();

    Model {
        scale: 24.0,
        snake,
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
        let Snake { head: old_head, .. } = model.snake;
        let head = old_head + direction_vector(&direction);

        if is_free(head.x as i32, head.y as i32) {

        model.snake.tail.push_front(old_head);
        while model.snake.tail.len() > model.snake.max_length - 1 {
            model.snake.tail.pop_back();
        }

        model.snake.direction = direction;
        model.snake.head = head;
    }
}
}

fn view(_app: &App, model: &Model, frame: &Frame) {
    let draw = _app.draw();

    draw.background().color(DARKBLUE);

    for &segment in model.snake.tail.iter() {
        draw.quad()
            .xy(segment * model.scale)
            .w_h(model.scale, model.scale)
            .color(GRAY);
    }

    let pos = model.snake.head * model.scale;

    draw.quad()
        .xy(pos)
        .w_h(model.scale, model.scale)
        .color(WHITE);

    let eye_size = 0.2 * model.scale;
    let eye_direction = direction_vector(&model.snake.direction);

    draw.quad()
        .xy(pos + eye_direction * 0.3 * model.scale)
        .w_h(eye_size, eye_size)
        .color(RED);

    draw.to_frame(_app, &frame).unwrap();
}