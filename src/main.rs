extern crate piston_window;

use piston_window::*;

fn main() {

    let mut x = 50.0;
    let mut y = 50.0;

    let mut window: PistonWindow = WindowSettings::new("Sneake", (800, 600))
        .exit_on_esc(true)
        .graphics_api(OpenGL::V3_2)
        .build()
        .unwrap();

    let rect_color: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

    while let Some(event) = window.next() {
        if let Some(_) = event.render_args() {
            window.draw_2d(&event, |context, graphics, _device| {
                clear([1.0; 4], graphics);
                rectangle(
                    rect_color,
                    rectangle::square(x, y, 50.0),
                    context.transform,
                    graphics,
                );
            });
        }
        if let Some(button) = event.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                x = x + 10.0;
            }
            if button == Button::Mouse(MouseButton::Right) {
                x = x - 10.0;
            }
        }
    }
}
