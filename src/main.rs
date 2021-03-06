extern crate env_logger;
extern crate glutin;
extern crate gfx;
extern crate gfx_window_glutin;
extern crate claymore_game as game;

pub fn main() {
    use gfx::traits::*;

    env_logger::init().unwrap();
    println!("Initializing the window...");

    let window = glutin::WindowBuilder::new()
        .with_title("Claymore".to_string())
        .with_vsync()
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 2)))
        .with_srgb(Some(true))
        .build().unwrap();
    let (mut stream, mut device, mut factory) = gfx_window_glutin::init(window);
    let _ = stream.out.set_gamma(gfx::Gamma::Convert);

    println!("Loading the game...");
    let mut app = game::App::new(&mut factory);

    println!("Rendering...");
    let (mut mouse_x, mut mouse_y) = (0, 0);
    'main: loop {
        // quit when Esc is pressed.
        for event in stream.out.window.poll_events() {
            use glutin::{ElementState, Event, MouseButton, VirtualKeyCode};
            match event {
                Event::Closed => break 'main,
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) => break 'main,
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::W)) =>
                    app.rotate_camera(-90.0),
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Q)) =>
                    app.rotate_camera(90.0),
                Event::MouseMoved((x, y)) => { mouse_x = x; mouse_y = y; },
                Event::MouseInput(ElementState::Pressed, MouseButton::Left) => {
                    let (sx, sy) = stream.out.get_size();
                    app.mouse_click(mouse_x as f32 / sx as f32, mouse_y as f32 / sy as f32);
                },
                _ => (),
            }
        }

        app.render(&mut stream);
        stream.present(&mut device);
    }
    println!("Done.");
}
