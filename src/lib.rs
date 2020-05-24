pub fn wu_line(
    color: u32,
    (x0, y0): (i32, i32),
    (x1, y1): (i32, i32),
    width: usize,
    height: usize,
    buffer: &mut Vec<u32>,
) {
    let r = color & 0x00ff_0000 >> 16;
    let g = color & 0x0000_ff00 >> 8;
    let b = color & 0x0000_00ff;

    let fr = r as f32 / 255.0;
    let fg = g as f32 / 255.0;
    let fb = b as f32 / 255.0;

    let dx = x1 - x0;
    let dy = y1 - y0;

    // Vertical line
    if dx == 0 {
        for y in y0.min(y1)..=y0.max(y1) {
            set_pixel(color, x0, y, width, height, buffer);
        }
    }

    // Horizontal line
    else if dy == 0 {
        for x in x0.min(x1)..=x0.max(x1) {
            set_pixel(color, x, y0, width, height, buffer)
        }
    }

    // Special case diagonal lines since they are common and
    // don't need anti-aliasing
    else if dx.abs() == dy.abs() {
        let xdir = dx.signum();
        let ydir = dy.signum();
        for i in 0..=dx {
            set_pixel(color, i*xdir + x0, i*ydir + y0, width, height, buffer);
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
            let fr = fr * error;
            let fg = fg * error;
            let fb = fb * error;

            let r0 = (linear_to_srgb(fr) * 255.0) as u32;
            let g0 = (linear_to_srgb(fg) * 255.0) as u32;
            let b0 = (linear_to_srgb(fb) * 255.0) as u32;

            let r1 = (linear_to_srgb(1.0 - fr) * 255.0) as u32;
            let g1 = (linear_to_srgb(1.0 - fg) * 255.0) as u32;
            let b1 = (linear_to_srgb(1.0 - fb) * 255.0) as u32;

            let color0 = r0 << 16 | g0 << 8 | b0;
            let color1 = r1 << 16 | g1 << 8 | b1;

            set_pixel(color0, x, y+ydir, width, height, buffer);
            set_pixel(color1, x, y, width, height, buffer);
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
            let fr = fr * error;
            let fg = fg * error;
            let fb = fb * error;

            let r0 = (linear_to_srgb(fr) * 255.0) as u32;
            let g0 = (linear_to_srgb(fg) * 255.0) as u32;
            let b0 = (linear_to_srgb(fb) * 255.0) as u32;

            let r1 = (linear_to_srgb(1.0 - fr) * 255.0) as u32;
            let g1 = (linear_to_srgb(1.0 - fg) * 255.0) as u32;
            let b1 = (linear_to_srgb(1.0 - fb) * 255.0) as u32;

            let color0 = r0 << 16 | g0 << 8 | b0;
            let color1 = r1 << 16 | g1 << 8 | b1;

            set_pixel(color0, x+xdir, y, width, height, buffer);
            set_pixel(color1, x, y, width, height, buffer);
            error += error_step;
            if error >= 1.0 {
                x += xdir;
                error -= 1.0;
            }

            y += ydir;
        }

    }

    set_pixel(color, x1, y1, width, height, buffer);
}

pub fn linear_to_srgb(x: f32) -> f32 {
    ((-0.9192 * x) + 1.9192) * x
}

pub fn interp(t: f32, x0: u32, x1: u32) -> u32 {
    ((1.0 - t) * x0 as f32 + t * x1 as f32).round() as u32
}

pub fn clear(color: u32, buffer: &mut Vec<u32>) {
    for p in buffer.iter_mut() {
        *p = color;
    }
}

pub fn set_pixel(color: u32, x: i32, y: i32, width: usize, height: usize, buffer: &mut Vec<u32>) {
    let x = x as usize;
    let y = y as usize;
    if x < width && y < height {
        // TODO: alpha blending
        buffer[x + y * width] = color.max(buffer[x + y * width]);
    } else {
        panic!("Point out of range x: {}   y: {}", x, y)
    }
}

#[test]
fn test_all_points() {
    let width = 71;
    let height = 43;

    let mut buffer: Vec<u32> = vec![0; width * height];
    for y0 in 0..height {
        for x0 in 0..width {
            for y1 in 0..height {
                for x1 in 0..width {
                    wu_line(0xffff_ffff, (x0 as i32, y0 as i32), (x1 as i32, y1 as i32), width, height, &mut buffer);
                }
            }
        }
    }
}