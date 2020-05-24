use rayon::prelude::*;

pub fn wu_line(
    (r,g,b,a): (f32,f32,f32,f32),
    (x0, y0): (i32, i32),
    (x1, y1): (i32, i32),
    width: usize,
    buffer: &mut Vec<(f32,f32,f32,f32)>,
) {
    let dx = x1 - x0;
    let dy = y1 - y0;

    // Vertical line
    if dx == 0 {
        for y in y0.min(y1)..=y0.max(y1) {
            set_pixel((r,g,b,a), x0, y, width, buffer);
        }
    }

    // Horizontal line
    else if dy == 0 {
        for x in x0.min(x1)..=x0.max(x1) {
            set_pixel((r,g,b,a), x, y0, width, buffer)
        }
    }

    // Special case diagonal lines since they are common and
    // don't need anti-aliasing
    else if dx.abs() == dy.abs() {
        let xdir = dx.signum();
        let ydir = dy.signum();
        for i in 0..=dx {
            set_pixel((r,g,b,a), i*xdir + x0, i*ydir + y0, width, buffer);
        }
    }

    // X-major
    else if dx.abs() > dy.abs() {
        // We already know that dx is non-zero
        let error_step = (dy as f32 / dx as f32).abs();
        let xdir = dx.signum();
        let ydir = dy.signum();

        let mut error = 0.0;
        let mut y = y0;
        let mut x = x0;
        while x != x1 {
            set_pixel((r,g,b,error), x, y+ydir, width, buffer);
            set_pixel((r,g,b,1.0-error), x, y, width, buffer);
            error += error_step;
            if error >= 1.0 {
                y += ydir;
                error -= 1.0;
            }

            x += xdir;
        }
    }

    // Y-major
    else {
        // We already know that dx is non-zero
        let error_step = (dx as f32 / dy as f32).abs();
        let xdir = dx.signum();
        let ydir = dy.signum();

        let mut error = 0.0;
        let mut y = y0;
        let mut x = x0;
        while y != y1 {
            set_pixel((r,g,b,error), x+xdir, y, width, buffer);
            set_pixel((r,g,b,1.0-error), x, y, width, buffer);
            error += error_step;
            if error >= 1.0 {
                x += xdir;
                error -= 1.0;
            }

            y += ydir;
        }

    }

    set_pixel((r,g,b,a), x1, y1, width, buffer);
}

pub fn linear_to_srgb(x: f32) -> f32 {
    ((-0.9192 * x) + 1.9192) * x
}

pub fn interp(t: f32, x0: u32, x1: u32) -> u32 {
    ((1.0 - t) * x0 as f32 + t * x1 as f32).round() as u32
}

pub fn clear(color: (f32,f32,f32,f32), buffer: &mut Vec<(f32,f32,f32,f32)>) {
    for p in buffer.iter_mut() {
        *p = color;
    }
}

pub fn set_pixel(color: (f32,f32,f32,f32), x: i32, y: i32, width: usize, buffer: &mut Vec<(f32,f32,f32,f32)>) {
    let x = x as usize;
    let y = y as usize;
    let index = x + y * width;
    if index < buffer.len() {
        buffer[index] = color;
    } else {
        panic!("Point out of range x: {}   y: {}", x, y)
    }
}

pub fn gamma_correct_buffer(in_buffer: &[(f32,f32,f32,f32)], out_buffer: &mut Vec<u32>) {
    in_buffer.par_iter()
        .map(|(r,g,b,a)| {
            ((linear_to_srgb(r * a) * 255.0) as u32) << 16 |
            ((linear_to_srgb(g * a) * 255.0) as u32) << 8 |
            (linear_to_srgb(b * a) * 255.0) as u32
        })
        .collect_into_vec(out_buffer);
}


#[test]
fn test_all_points() {
    let width = 71;
    let height = 43;

    let mut buffer: Vec<(f32,f32,f32,f32)> = vec![(0.0, 0.0, 0.0, 0.0); width * height];
    for y0 in 0..height {
        for x0 in 0..width {
            for y1 in 0..height {
                for x1 in 0..width {
                    wu_line((1.0,1.0,1.0,1.0), (x0 as i32, y0 as i32), (x1 as i32, y1 as i32), width, &mut buffer);
                }
            }
        }
    }
}