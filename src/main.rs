use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).view(view).run();
}

struct Model {}

/* initial model creation; this is similar to Arduino's `setup()` */
fn model(_app: &App) -> Model {
    _app.new_window()
        .with_dimensions(800, 600)
        .with_title("Sneake")
        .build()
        .unwrap();
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(_app: &App, _model: &Model, frame: Frame) -> Frame {
    let draw = _app.draw();

    draw.background().color(DARK_BLUE);

    draw.quad().x_y(10.0, 10.0).color(WHITE);

    draw.to_frame(_app, &frame).unwrap();

    frame
}