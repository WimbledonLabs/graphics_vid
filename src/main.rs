extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use graphics_vid::*;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

fn main() {
    let mut buffer: Vec<(f32,f32,f32,f32)> = vec![(0.0, 0.0, 0.0, 1.0); WIDTH * HEIGHT];
    let mut ibuffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let window_options = WindowOptions {
        ..WindowOptions::default()
    };
    let mut window = Window::new("Graphics Vid", WIDTH, HEIGHT, window_options)
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16666)));

    let mut t = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        clear((0.0,0.0,0.0,1.0), &mut buffer);

        let mouse_posf = window.get_mouse_pos(minifb::MouseMode::Pass).unwrap_or((0.0, 0.0));
        let mouse_posi = (
            mouse_posf.0.round() as i32,
            mouse_posf.1.round() as i32,
        );

        fn distort((x,y): (i32, i32)) -> (i32, i32) {
            // As y gets closer to 0, x gets closer to the midpoint (WIDTH/2)
            let distortion = y as f32 / HEIGHT as f32;
            let xf = (x - (WIDTH/2) as i32) as f32 * distortion + (WIDTH/2) as f32;
            let yf = (y - (HEIGHT/2) as i32) as f32 * distortion + (HEIGHT/2) as f32;

            (xf.round() as i32, yf.round() as i32)
        }

        let frame_start = std::time::Instant::now();
        for y in 1..=25 {
            for x in 1..=36 {
                let real_y = y * 50 + (t%50);
                let value = match real_y {
                    (0..=950) => (real_y as f32 / 1080.0).powi(2),
                    _ => {
                        let t = (1000 - real_y) as f32 / 50.0;
                        clamp(interpf(t, 0.0, 1.0), 0.0, 1.0)
                    },
                };

                let color = (
                    value,
                    0.0,
                    value,
                    1.0,
                );

                if value != 0.0 {
                    wu_line(color, distort(((x+0)*50, (y+0)*50 + (t%50))), distort(((x+1)*50, (y+0)*50 + (t%50))), WIDTH, &mut buffer);
                    wu_line(color, distort(((x+1)*50, (y+0)*50 + (t%50))), distort(((x+1)*50, (y+1)*50 + (t%50))), WIDTH, &mut buffer);
                    wu_line(color, distort(((x+1)*50, (y+1)*50 + (t%50))), distort(((x+0)*50, (y+1)*50 + (t%50))), WIDTH, &mut buffer);
                    wu_line(color, distort(((x+0)*50, (y+1)*50 + (t%50))), distort(((x+0)*50, (y+0)*50 + (t%50))), WIDTH, &mut buffer);
                    wu_line(color, distort(((x+0)*50, (y+1)*50 + (t%50))), distort(((x+1)*50, (y+0)*50 + (t%50))), WIDTH, &mut buffer);
                }
            }
        }

        wu_line((1.0, 1.0, 1.0, 1.0), (WIDTH as i32/2, HEIGHT as i32/2), mouse_posi, WIDTH, &mut buffer);

        gamma_correct_buffer(&buffer, &mut ibuffer);

        let frame_time = frame_start.elapsed();
        println!("Frame time: {:?}", frame_time);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&ibuffer, WIDTH, HEIGHT).unwrap();

        t += 1;
    }
}
