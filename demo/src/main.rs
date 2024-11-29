#![deny(clippy::all)]
#![forbid(unsafe_code)]

use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use rand::thread_rng;
use tetris::random::RandomGenerator;
use tetris::rotate::SuperRotationSystem;
use tetris::Game;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard:: KeyCode;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const PIXEL_SIZE: u32 = 10;
const WIDTH: u32 = 24;
const HEIGHT: u32 = 24;

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new((WIDTH * PIXEL_SIZE) as f64, (HEIGHT * PIXEL_SIZE) as f64);
        WindowBuilder::new()
            .with_title("Tetris")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH * PIXEL_SIZE,HEIGHT * PIXEL_SIZE, surface_texture)?
    };

    let rot = SuperRotationSystem{};
    let rng = RandomGenerator::new(thread_rng());
    let mut game = Game::new(rng, rot);
    game.update();
    game.update();
    game.update();

    let res = event_loop.run(|event, elwt| {
        // Draw the current frame
        if let Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } = event
        {
            let frame = pixels.frame_mut();

            for x in 0..10 {
                for y in 0..20 {
                    draw_pixel(frame, x,  y, (0, 0, 0));
                }
            }

            let p = game.ghost_piece();
            let mut c = p.color();
            c.0 /= 2;
            c.1 /= 2;
            c.2 /= 2;
            draw_mask(frame, p.y(), p.mask(), c);

            let p = game.current_piece();
            draw_mask(frame, p.y(), p.mask(), p.color());
            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
                elwt.exit();
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                elwt.exit();
                return;
            }

            if input.key_pressed(KeyCode::KeyZ) {
                game.rotate_left();
            }

            if input.key_pressed(KeyCode::ArrowUp) {
                game.rotate_right();
            }

            if input.key_pressed(KeyCode::ArrowDown) {
                // Drop
                // game.drop();
            }

            if input.key_pressed(KeyCode::ArrowLeft) {
                game.move_left();
            }

            if input.key_pressed(KeyCode::ArrowRight) {
                game.move_right();
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_error("pixels.resize_surface", err);
                    elwt.exit();
                    return;
                }
            }

            // Update internal state and request a redraw
            // game.update();
            window.request_redraw();
        }
    });
    res.map_err(|e| Error::UserDefined(Box::new(e)))
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}

fn draw_pixel(frame: &mut [u8], x: u32, y: u32, color: (u8, u8, u8)) {
    let y = HEIGHT - y - 1;
    let mut colors = [0; (PIXEL_SIZE as usize) * 4];
    for pixel in colors.chunks_exact_mut(4) {
        let rgba = [color.0, color.1, color.2, 255];
        pixel.copy_from_slice(&rgba);
    }

    for i in 0..PIXEL_SIZE {
        let start = (((WIDTH * (i + (y * PIXEL_SIZE))) + x) * 4 * PIXEL_SIZE) as usize;
        frame[start..start + ((PIXEL_SIZE as usize) * 4)].copy_from_slice(&colors);
    }
}

fn draw_mask(frame: &mut [u8], y: u32, mask: [u16; 4], color: (u8, u8, u8)) {
    for (i, m) in mask.iter().enumerate() {
        let y = y + i as u32;
        if y < 20 {
            for x in 0..10 {
                if ((1 << x) & *m) != 0 {
                    draw_pixel(frame, x, y, color);
                }
            }
        }
    }
}