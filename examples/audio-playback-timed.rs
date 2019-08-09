// play an audio file multiple times
// this is mainly a test to investigate issues with playback interruptions
// when too many files are played at once, the playback starts to stutter and
// have gaps of a couple of milliseconds

use rodio::Source;
use std::fs::File;
use std::io::BufReader;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let device = rodio::default_output_device().unwrap();

    loop {
        let file = File::open("assets/sound/sine_440_20s.wav").unwrap();
        //let file = File::open("assets/sound/sine_440_20s.ogg").unwrap();
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
        rodio::play_raw(&device, source.repeat_infinite().convert_samples());
        sleep(Duration::from_millis(50));
    }
}
