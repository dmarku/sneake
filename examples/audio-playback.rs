// for now this is a carbon-copy of rodio's example file
use rodio::Source;
use std::fs::File;
use std::io::BufReader;

use nannou::prelude::*;

struct Model {
    device: rodio::Device,
}

fn model(app: &App) -> Model {
    app.new_window().key_pressed(key_pressed).build().unwrap();
    let device = rodio::default_output_device().unwrap();
    //let file = File::open("assets/music/ObservingTheStar.ogg").unwrap();
    let file = File::open("assets/sound/sine_440_20s.wav").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

    rodio::play_raw(&device, source.repeat_infinite().convert_samples());

    Model { device }
}

fn view(_app: &App, _model: &Model, frame: &Frame) {
    frame.clear(DARKGREEN);
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => {
            let file = File::open("assets/sound/sine_440_20s.wav").unwrap();
            let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
            rodio::play_raw(&model.device, source.convert_samples());
        }
        _ => {}
    }
}

fn main() {
    nannou::app(model).view(view).run();
}
