extern crate piston_window;

use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Sneake", [800, 600])
    .exit_on_esc(true).build().unwrap();

    let rect_color: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle(rect_color, rectangle::square(0.0, 0.0, 50.0), context.transform, graphics);
        });
    }
}
