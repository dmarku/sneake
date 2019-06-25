
use nannou::prelude::*;
use std::collections::VecDeque;
fn main() {
    nannou::app(model).update(update).view(view).run();
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

trait Axis {
    fn horizontal(&self) -> bool;
    fn vertical(&self) -> bool;
}

impl Axis for Direction {
    fn horizontal(&self) -> bool {
        match self {
            Direction::Up | Direction::Down => false,
            Direction::Left | Direction::Right => true,
        }
    }

    fn vertical(&self) -> bool {
        match self {
            Direction::Up | Direction::Down => true,
            Direction::Left | Direction::Right => false,
        }
    }
}

struct Snake {
    head: Vector2,
    max_length: i8,
    direction: Direction,
    // the segments of the snake, except for the head
    tail: VecDeque<Vector2>,
}

struct Model {
    scale: f32,
    snake: Snake,
}

/* initial model creation; this is similar to Arduino's `setup()` */
fn model(_app: &App) -> Model {
    _app.new_window()
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

    Model { scale: 24.0, snake }
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

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    if let Some(direction) = map_movement(key) {
        model.snake.direction = direction;
    }
}

fn view(_app: &App, model: &Model, frame: Frame) -> Frame {
    let draw = _app.draw();

    draw.background().color(DARK_BLUE);

    let pos = model.snake.head * model.scale;

    draw.quad()
        .xy(pos)
        .w_h(model.scale, model.scale)
        .color(WHITE);

    let eye_size = 0.2 * model.scale;
    let eye_direction = match model.snake.direction {
        Direction::Up => Vector2::unit_y(),
        Direction::Down => -Vector2::unit_y(),
        Direction::Left => -Vector2::unit_x(),
        Direction::Right => Vector2::unit_x(),
    };
    draw.quad()
        .xy(pos + eye_direction * 0.3 * model.scale)
        .w_h(eye_size, eye_size)
        .color(RED);

    for &segment in model.snake.tail.iter() {
        draw.quad()
            .xy(segment * model.scale)
            .w_h(model.scale, model.scale)
            .color(LIGHT_GRAY);
    }

    draw.to_frame(_app, &frame).unwrap();
    frame
}