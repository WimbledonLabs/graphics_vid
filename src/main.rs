extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use graphics_vid::*;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let window_options = WindowOptions {
        ..WindowOptions::default()
    };
    let mut window = Window::new("Graphics Vid", WIDTH, HEIGHT, window_options)
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16666)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        clear(0, &mut buffer);

        let mouse_posf = window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap_or((0.0, 0.0));
        let mouse_posi = (
            i32::min(mouse_posf.0.round() as i32, WIDTH as i32 - 1),
            i32::min(mouse_posf.1.round() as i32, HEIGHT as i32 - 1)
        );

        let frame_start = std::time::Instant::now();
        for y in vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000].iter() {
            for x in vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200, 1300, 1400, 1500, 1600, 1700, 1800].iter() {
                wu_line(0xffff_ffff, (*x, *y), mouse_posi, WIDTH, HEIGHT, &mut buffer);
            }
        }

        let frame_time = frame_start.elapsed();
        println!("Frame time: {:?}", frame_time);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
