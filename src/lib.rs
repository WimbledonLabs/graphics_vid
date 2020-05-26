use rayon::prelude::*;

pub fn clamp<T: PartialOrd>(value: T, low: T, high: T) -> T {
    if value < low {
        low
    } else if value > high {
        high
    } else {
        value
    }
}

pub fn in_range<T: PartialOrd>(value: T, low: T, high: T) -> bool {
    low <= value && value <= high
}

// Return a new line segment the completely overlaps with the provided
// rectangle. If the line segment is completely outside the rectangle,
// None is returned.
pub fn line_segment_in_rect(
        (x0, y0): (i32, i32),
        (x1, y1): (i32, i32),
        width: usize,
        height: usize,
) -> Option<((i32, i32), (i32, i32))> {
    // Convert from usize to ints so we don't need to do this a ton later
    let width = width as i32;
    let height = height as i32;

    let max_x = width-1;
    let max_y = height-1;

    // Fast common case where both points are in the rect: we don't
    // need to do any clipping in this case
    if     in_range(x0, 0, max_x)
        && in_range(x1, 0, max_x)
        && in_range(y0, 0, max_y)
        && in_range(y1, 0, max_y)
    {
        return Some(((x0, y0), (x1, y1)));
    }

    // Handle the simple cases, so they don't need to be special-cased later

    let vertical_line = x0 == x1;
    let horizontal_line = y0 == y1;

    if vertical_line {
        let y0 = clamp(y0, 0, height-1);
        let y1 = clamp(y1, 0, height-1);

        return if in_range(x0, 0, max_x) {
            Some(((x0, y0), (x1, y1)))
        } else {
            None
        };
    }

    if horizontal_line {
        let x0 = clamp(x0, 0, width-1);
        let x1 = clamp(x1, 0, width-1);

        return if in_range(y0, 0, max_y) {
            Some(((x0, y0), (x1, y1)))
        } else {
            None
        };
    }

    // Re-arrange p0 and p1 so that p0 has a smaller x
    let ((x0, y0), (x1, y1)) = if x0 < x1 {
        ((x0, y0), (x1, y1))
    } else {
        ((x1, y1), (x0, y0))
    };

    let dx = (x1 - x0) as f32;
    let dy = (y1 - y0) as f32;

    let m: f32 = dy / dx;
    let b: f32 = y0 as f32 - m * x0 as f32;

    // To intersect with the lines of the just re-arrange y=mx+b
    let left_intersection = (0, b.round() as i32);
    let right_intersection = ((width-1), (m*((width-1) as f32) + b).round() as i32);
    let top_intersection = ((-b/m).round() as i32, 0);
    let bottom_intersection = ((((height-1) as f32-b)/m).round() as i32, height-1);
    let p0 = (x0, y0);
    let p1 = (x1, y1);

    // Now we have 6 points we care about (some may not be unique).
    // We throw out any points that aren't inside the rects defined
    // by points p0 and p1, and we throw out any points that aren't
    // in the rect defined by width-1 and height-1 (at origin 0,0).

    // x1 is larger than x0, so we only need to bound the max by x1
    // and the min my x0
    let min_x = (0).max(x0);
    let max_x = (width-1).min(x1);
    
    // We don't know which is bigger/smaller between y0 and y1
    let min_y = (0).max(y0.min(y1));
    let max_y = (height-1).min(y0.max(y1));

    let mut new_p0 = None;
    let mut new_p1 = None;

    // Get the point with the smallest x and the point with the largest x (in range).
    // This is our new line.

    fn replace_if_x_smaller_in_range(value: &mut Option<(i32, i32)>, new: (i32, i32), min_x: i32, max_x: i32, min_y: i32, max_y: i32) {
        // If the point isn't in range, don't do anything
        if     !in_range(new.0, min_x, max_x)
            || !in_range(new.1, min_y, max_y) {
            return;
        }

        // So the point _is_ in range, and we need to see if it's better
        match value {
            None => *value = Some(new),
            Some(old) => if new.0 < old.0 {
                *value = Some(new)
            }
        }
    }

    fn replace_if_x_larger_in_range(value: &mut Option<(i32, i32)>, new: (i32, i32), min_x: i32, max_x: i32, min_y: i32, max_y: i32) {
        // If the point isn't in range, don't do anything
        if     !in_range(new.0, min_x, max_x)
            || !in_range(new.1, min_y, max_y) {
            return;
        }

        // So the point _is_ in range, and we need to see if it's better
        match value {
            None => *value = Some(new),
            Some(old) => if new.0 > old.0 {
                *value = Some(new)
            }
        }
    }

    replace_if_x_smaller_in_range(&mut new_p0, p0, min_x, max_x, min_y, max_y);
    replace_if_x_larger_in_range(&mut new_p1, p0, min_x, max_x, min_y, max_y);
    
    replace_if_x_smaller_in_range(&mut new_p0, p1, min_x, max_x, min_y, max_y);
    replace_if_x_larger_in_range(&mut new_p1, p1, min_x, max_x, min_y, max_y);
    
    replace_if_x_smaller_in_range(&mut new_p0, top_intersection, min_x, max_x, min_y, max_y);
    replace_if_x_larger_in_range(&mut new_p1, top_intersection, min_x, max_x, min_y, max_y);
    
    replace_if_x_smaller_in_range(&mut new_p0, bottom_intersection, min_x, max_x, min_y, max_y);
    replace_if_x_larger_in_range(&mut new_p1, bottom_intersection, min_x, max_x, min_y, max_y);
    
    replace_if_x_smaller_in_range(&mut new_p0, left_intersection, min_x, max_x, min_y, max_y);
    replace_if_x_larger_in_range(&mut new_p1, left_intersection, min_x, max_x, min_y, max_y);
    
    replace_if_x_smaller_in_range(&mut new_p0, right_intersection, min_x, max_x, min_y, max_y);
    replace_if_x_larger_in_range(&mut new_p1, right_intersection, min_x, max_x, min_y, max_y);

    match (new_p0, new_p1) {
        (Some(a), Some(b)) => Some((a, b)),
        _ => None
    }
}

pub fn wu_line(
    (r,g,b,_): (f32,f32,f32,f32),
    (x0, y0): (i32, i32),
    (x1, y1): (i32, i32),
    width: usize,
    buffer: &mut Vec<(f32,f32,f32,f32)>,
) {
    let height = buffer.len() / width;

    let ((x0, y0), (x1, y1)) = match line_segment_in_rect(
        (x0, y0),
        (x1, y1),
        width,
        height
    ) {
        None => return,
        Some(segment) => segment
    };

    // These checks will only fail if line_segment_in_rect
    // was implemented improperly. Once more confidence has
    // been built up the it _has_ been implemented properly,
    // then this check could be removed.
    if !in_range(x0, 0, width as i32 - 1) {
        panic!("x0 not in range {}", x0);
    };
    if !in_range(x1, 0, width as i32 - 1) {
        panic!("x1 not in range {}", x1);
    };
    if !in_range(y0, 0, height as i32 - 1) {
        panic!("y0 not in range {}", y0);
    };
    if !in_range(y1, 0, height as i32 - 1) {
        panic!("y1 not in range {}", y1);
    };

    // Update the second point to ensure it's within bounds

    let dx = x1 - x0;
    let dy = y1 - y0;

    // Vertical line
    if dx == 0 {
        for y in y0.min(y1)..=y0.max(y1) {
            set_pixel((r,g,b,1.0), x0, y, width, buffer);
        }
    }

    // Horizontal line
    else if dy == 0 {
        let start_index = coord_to_index(x0.min(x1) as usize, y0 as usize, width);
        let end_index = coord_to_index(x0.max(x1) as usize, y0 as usize, width);
        for x in buffer[start_index..=end_index].iter_mut() {
            *x = (r,g,b,1.0);
        }
    }

    // Special case diagonal lines since they are common and
    // don't need anti-aliasing
    else if dx.abs() == dy.abs() {
        let xdir = dx.signum();
        let ydir = dy.signum();
        for i in 0..=dx.abs() {
            set_pixel((r,g,b,1.0), i*xdir + x0, i*ydir + y0, width, buffer);
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

    set_pixel((r,g,b,1.0), x1, y1, width, buffer);
}

pub fn linear_to_srgb(x: f32) -> f32 {
    ((-0.9192 * x) + 1.9192) * x
}

pub fn interp(t: f32, x0: u32, x1: u32) -> u32 {
    ((1.0 - t) * x0 as f32 + t * x1 as f32).round() as u32
}

pub fn interpf(t: f32, x0: f32, x1: f32) -> f32 {
    (1.0 - t) * x0 + t * x1
}

pub fn clear(color: (f32,f32,f32,f32), buffer: &mut Vec<(f32,f32,f32,f32)>) {
    for p in buffer.iter_mut() {
        *p = color;
    }
}

pub fn coord_to_index(x: usize, y: usize, width: usize) -> usize {
    x + y*width
}

pub fn set_pixel((new_r, new_g, new_b, a): (f32,f32,f32,f32), x: i32, y: i32, width: usize, buffer: &mut Vec<(f32,f32,f32,f32)>) {
    let x = x as usize;
    let y = y as usize;
    let index = x + y * width;
    if index < buffer.len() {
        let (old_r, old_g, old_b, _) = buffer[index];
        buffer[index] = (
            new_r * a + old_r * (1.0-a),
            new_g * a + old_g * (1.0-a),
            new_b * a + old_b * (1.0-a),
            1.0,
        );
    } else {
        panic!("Point out of range x: {}   y: {}", x, y)
    }
}

pub fn gamma_correct_buffer(in_buffer: &[(f32,f32,f32,f32)], out_buffer: &mut Vec<u32>) {
    in_buffer.par_iter()
        .map(|(r,g,b,_a)| {
            ((linear_to_srgb(*r) * 255.0) as u32) << 16 |
            ((linear_to_srgb(*g) * 255.0) as u32) << 8 |
             (linear_to_srgb(*b) * 255.0) as u32
        })
        .collect_into_vec(out_buffer);
}

pub fn draw_text(
    (r, g, b): (f32, f32, f32),
    mut origin: (i32, i32),
    size: f32,
    text: &str,
    width: usize,
    buffer: &mut Vec<(f32,f32,f32,f32)>)
{
    let char_width = (size / 1.618) as i32;
    let char_height = size as i32;
    let spacing = (size / 1.618 / 1.618) as i32;

    for c in text.chars() {
        for (p0, p1) in char_lines(c) {
            wu_line(
                (r,g,b,1.0),
                (origin.0 + (p0.0 * char_width as f32).round() as i32, origin.1 + (p0.1 * char_height as f32).round() as i32),
                (origin.0 + (p1.0 * char_width as f32).round() as i32, origin.1 + (p1.1 * char_height as f32).round() as i32),
                width,
                buffer
            );
        }

        origin.0 += char_width + spacing;
    }
}

fn char_lines(c: char) -> &'static [((f32, f32), (f32, f32))] {
    match c {
        ' ' => &[],
        '!' => &[
            ((0.5, 0.0), (0.5, 0.6)),
            ((0.5, 0.9), (0.5, 1.0)),
        ],
        '"' => &[
            ((0.3, 0.0), (0.3, 0.2)),
            ((0.7, 0.0), (0.7, 0.2)),
        ],
        '#' => &[
            ((0.25, 0.0), (0.25, 1.0)),
            ((0.75, 0.0), (0.75, 1.0)),
            ((0.0, 0.25), (1.0, 0.25)),
            ((0.0, 0.75), (1.0, 0.75)),
        ],
        '$' => &[
            ((0.5, 0.0), (0.5, 1.0)),

            ((1.0, 0.1), (1.0, 0.3)),
            ((0.0, 0.1), (1.0, 0.1)),
            ((0.0, 0.1), (0.0, 0.5)),
            ((0.0, 0.5), (1.0, 0.5)),
            ((1.0, 0.5), (1.0, 0.9)),
            ((1.0, 0.9), (0.0, 0.9)),
            ((0.0, 0.9), (0.0, 0.7)),
        ],
        '%' => &[
            ((1.0, 0.0), (0.0, 1.0)),

            ((0.0, 0.0), (0.3, 0.0)),
            ((0.3, 0.0), (0.3, 0.3)),
            ((0.3, 0.3), (0.0, 0.3)),
            ((0.0, 0.3), (0.0, 0.0)),

            ((0.7, 0.7), (1.0, 0.7)),
            ((1.0, 0.7), (1.0, 1.0)),
            ((1.0, 1.0), (0.7, 1.0)),
            ((0.7, 1.0), (0.7, 0.7)),
        ],
        '&' => &[
            ((1.0, 1.0), (0.0, 0.25)),
            ((0.0, 0.25), (0.25, 0.0)),
            ((0.25, 0.0), (0.5, 0.0)),
            ((0.5, 0.0), (0.75, 0.25)),
            ((0.75, 0.25), (0.0, 0.75)),
            ((0.0, 0.75), (0.0, 1.0)),
            ((0.0, 1.0), (0.5, 1.0)),
            ((0.5, 1.0), (1.0, 0.5)),
        ],
        '\'' => &[
            ((0.5, 0.0), (0.5, 0.2)),
        ],
        '(' => &[
            ((0.25, 0.0), (0.0, 0.25)),
            ((0.0, 0.25), (0.0, 0.75)),
            ((0.0, 0.75), (0.25, 1.0)),
        ],
        ')' => &[
            ((0.75, 0.0), (1.0, 0.25)),
            ((1.0, 0.25), (1.0, 0.75)),
            ((1.0, 0.75), (0.75, 1.0)),
        ],
        '*' => &[
            ((0.2, 0.1), (0.8, 0.3)),
            ((0.8, 0.1), (0.2, 0.3)),
            ((0.5, 0.0), (0.5, 0.4)),
        ],
        '+' => &[
            ((0.5, 0.25), (0.5, 0.75)),
            ((0.2, 0.5), (0.8, 0.5)),
        ],
        ',' => &[
            ((0.4, 0.7), (0.5, 1.0)),
        ],
        '-' => &[
            ((0.1, 0.5), (0.9, 0.5)),
        ],
        '.' => &[
            ((0.3, 0.8), (0.7, 0.8)),
            ((0.7, 0.8), (0.7, 1.0)),
            ((0.7, 1.0), (0.3, 1.0)),
            ((0.3, 1.0), (0.3, 0.8)),
        ],
        '/' => &[
            ((0.0, 1.0), (1.0, 0.0)),
        ],
        '0' => &[
            ((0.0, 0.0), (0.0, 1.0)),
            ((0.0, 1.0), (1.0, 1.0)),
            ((1.0, 1.0), (1.0, 0.0)),
            ((1.0, 0.0), (0.0, 0.0)),
            ((1.0, 0.0), (0.0, 1.0)),
        ],
        '1' => &[
            ((0.0, 0.5), (0.5, 0.0)),
            ((0.5, 0.0), (0.5, 1.0)),
            ((0.0, 1.0), (1.0, 1.0)),
        ],
        '2' => &[
            ((0.0, 1./3.), (0.0, 0.0)),
            ((0.0, 0.0), (1.0, 0.0)),
            ((1.0, 0.0), (1.0, 1./3.)),
            ((1.0, 1./3.), (0.0, 1.0)),
            ((0.0, 1.0), (1.0, 1.0)),
        ],
        '3' => &[
            ((0.0, 0.0), (1.0, 0.0)),
            ((1.0, 0.0), (1.0, 1.0)),
            ((1.0, 1.0), (0.0, 1.0)),
            ((1./3., 0.5), (1.0, 0.5)),
        ],
        '4' => &[
            ((0.0, 0.0), (0.0, 0.5)),
            ((0.0, 0.5), (1.0, 0.5)),
            ((1.0, 0.0), (1.0, 1.0)),
        ],
        '5' => &[
            ((1.0, 0.0), (0.0, 0.0)),
            ((0.0, 0.0), (0.0, 0.5)),
            ((0.0, 0.5), (1.0, 0.5)),
            ((1.0, 0.5), (1.0, 1.0)),
            ((1.0, 1.0), (0.0, 1.0)),
        ],
        '6' => &[
            ((0.0, 0.0), (0.0, 1.0)),
            ((0.0, 1.0), (1.0, 1.0)),
            ((1.0, 1.0), (1.0, 0.5)),
            ((1.0, 0.5), (0.0, 0.5)),
        ],
        '7' => &[
            ((0.0, 0.0), (1.0, 0.0)),
            ((1.0, 0.0), (0.0, 1.0)),
        ],
        '8' => &[
            ((0.0, 0.0), (1.0, 0.0)),
            ((1.0, 0.0), (1.0, 1.0)),
            ((1.0, 1.0), (0.0, 1.0)),
            ((0.0, 1.0), (0.0, 0.0)),
            ((0.0, 0.5), (1.0, 0.5)),
        ],
        '9' => &[
            ((1.0, 0.5), (0.0, 0.5)),
            ((0.0, 0.5), (0.0, 0.0)),
            ((0.0, 0.0), (1.0, 0.0)),
            ((1.0, 0.0), (1.0, 1.0)),
        ],
        ':' => &[
            ((0.3, 0.2), (0.7, 0.2)),
            ((0.7, 0.2), (0.7, 0.4)),
            ((0.7, 0.4), (0.3, 0.4)),
            ((0.3, 0.4), (0.3, 0.2)),

            ((0.3, 0.6), (0.7, 0.6)),
            ((0.7, 0.6), (0.7, 0.8)),
            ((0.7, 0.8), (0.3, 0.8)),
            ((0.3, 0.8), (0.3, 0.6)),
        ],
        ';' => &[
            ((0.3, 0.2), (0.7, 0.2)),
            ((0.7, 0.2), (0.7, 0.4)),
            ((0.7, 0.4), (0.3, 0.4)),
            ((0.3, 0.4), (0.3, 0.2)),

            ((0.5, 0.7), (0.6, 1.0)),
        ],
        '<' => &[
            ((0.0, 0.5), (1.0, 0.3)),
            ((0.0, 0.5), (1.0, 0.7)),
        ],
        '=' => &[
            ((0.0, 0.4), (1.0, 0.4)),
            ((0.0, 0.6), (1.0, 0.6)),
        ],
        '>' => &[
            ((1.0, 0.5), (0.0, 0.3)),
            ((1.0, 0.5), (0.0, 0.7)),
        ],
        '?' => &[
            ((0.0, 0.25), (0.0, 0.0)),
            ((0.0, 0.0), (1.0, 0.0)),
            ((1.0, 0.0), (1.0, 0.5)),
            ((1.0, 0.5), (0.5, 0.5)),
            ((0.5, 0.5), (0.5, 0.7)),
            ((0.5, 0.8), (0.5, 1.0)),
        ],
        '@' => &[
            ((1.0, 0.5), (0.5, 0.5)),
            ((0.5, 0.5), (0.5, 0.8)),
            ((0.5, 0.8), (1.0, 0.8)),
            ((1.0, 0.8), (1.0, 0.0)),
            ((1.0, 0.0), (0.0, 0.0)),
            ((0.0, 0.0), (0.0, 1.0)),
            ((0.0, 1.0), (1.0, 1.0)),
        ],
        'A' => &[
            ((0.5, 0.0), (0.0, 1.0)),
            ((0.5, 0.0), (1.0, 1.0)),
            ((1./6., 2./3.), (5./6., 2./3.)),
        ],
        'B' => &[
            ((0.0, 0.0), (2./3., 0.0)),
            ((2./3., 0.0), (1.0, 1./6.)),
            ((1.0, 1./6.), (1.0, 2./6.)),
            ((1.0, 2./6.), (2./3., 0.5)),
            ((2./3., 0.5), (1.0, 4./6.)),
            ((1.0, 4./6.), (1.0, 5./6.)),
            ((1.0, 5./6.), (2./3., 1.0)),
            ((2./3., 1.0), (0.0, 1.0)),
            ((0.0, 0.0), (0.0, 1.0)),
            ((0.0, 0.5), (2./3., 0.5)),
        ],
        'C' => &[
            ((0.0, 0.0), (1.0, 0.0)),
            ((0.0, 0.0), (0.0, 1.0)),
            ((0.0, 1.0), (1.0, 1.0)),
        ],
        'D' => &[
            ((0.0, 0.0), (0.0, 1.0)),
            ((0.0, 0.0), (2./3., 0.0)),
            ((2./3., 0.0), (1.0, 1./3.)),
            ((1.0, 1./3.), (1.0, 2./3.)),
            ((1.0, 2./3.), (2./3., 1.0)),
            ((2./3., 1.0), (0.0, 1.0)),
        ],
        'E' => &[
            ((0.0, 0.0), (0.0, 1.0)),
            ((0.0, 0.0), (1.0, 0.0)),
            ((0.0, 0.5), (2./3., 0.5)),
            ((0.0, 1.0), (1.0, 1.0)),
        ],
        'F' => &[
            ((0.0, 0.0), (0.0, 1.0)),
            ((0.0, 0.0), (1.0, 0.0)),
            ((0.0, 0.5), (2./3., 0.5)),
        ],
        'G' => &[
            ((0.0, 0.0), (1.0, 0.0)),
            ((0.0, 0.0), (0.0, 1.0)),
            ((0.0, 1.0), (1.0, 1.0)),
            ((1.0, 1.0), (1.0, 0.5)),
            ((1.0, 0.5), (0.5, 0.5)),
        ],
        'H' => &[
            ((0.0, 0.0), (0.0, 1.0)),
            ((0.0, 0.5), (1.0, 0.5)),
            ((1.0, 0.0), (1.0, 1.0)),
        ],
        'I' => &[
            ((0.0, 0.0), (1.0, 0.0)),
            ((0.5, 0.0), (0.5, 1.0)),
            ((0.0, 1.0), (1.0, 1.0)),
        ],
        'J' => &[
            ((1.0, 0.0), (1.0, 1.0)),
            ((0.0, 0.5), (0.0, 1.0)),
            ((0.0, 1.0), (1.0, 1.0)),
        ],
        'K' => &[
            ((0.0, 0.0), (0.0, 1.0)),
            ((0.0, 0.5), (1.0, 0.0)),
            ((0.0, 0.5), (1.0, 1.0)),
        ],
        'L' => &[
            ((0.0, 0.0), (0.0, 1.0)),
            ((0.0, 1.0), (1.0, 1.0)),
        ],
        'M' => &[
            ((0.0, 0.0), (0.0, 1.0)),
            ((0.0, 0.0), (0.5, 1.0)),
            ((0.5, 1.0), (1.0, 0.0)),
            ((1.0, 0.0), (1.0, 1.0)),
        ],
        'N' => &[
            ((0.0, 0.0), (0.0, 1.0)),
            ((0.0, 0.0), (1.0, 1.0)),
            ((1.0, 0.0), (1.0, 1.0)),
        ],
        'O' => &[
            ((0.0, 0.0), (0.0, 1.0)),
            ((0.0, 1.0), (1.0, 1.0)),
            ((1.0, 1.0), (1.0, 0.0)),
            ((1.0, 0.0), (0.0, 0.0)),
        ],
        'P' => &[
            ((0.0, 0.0), (0.0, 1.0)),
            ((0.0, 0.0), (1.0, 0.0)),
            ((1.0, 0.0), (1.0, 0.5)),
            ((1.0, 0.5), (0.0, 0.5)),
        ],
        'Q' => &[
            ((2./3., 2./3.), (1.1, 1.1)),

            ((0.0, 0.0), (0.0, 1.0)),
            ((0.0, 1.0), (1.0, 1.0)),
            ((1.0, 1.0), (1.0, 0.0)),
            ((1.0, 0.0), (0.0, 0.0)),
        ],
        'R' => &[
            ((0.0, 0.0), (0.0, 1.0)),
            ((0.0, 0.0), (1.0, 0.0)),
            ((1.0, 0.0), (1.0, 0.5)),
            ((1.0, 0.5), (0.0, 0.5)),
            ((0.0, 0.5), (1.0, 1.0)),
        ],
        'S' => &[
            ((1.0, 0.0), (1.0, 0.2)),

            ((0.0, 0.0), (1.0, 0.0)),
            ((0.0, 0.0), (0.0, 0.5)),
            ((0.0, 0.5), (1.0, 0.5)),
            ((1.0, 0.5), (1.0, 1.0)),
            ((1.0, 1.0), (0.0, 1.0)),

            ((0.0, 1.0), (0.0, 0.8)),
        ],
        'T' => &[
            ((0.0, 0.0), (1.0, 0.0)),
            ((0.5, 0.0), (0.5, 1.0)),
        ],
        'U' => &[
            ((0.0, 0.0), (0.0, 1.0)),
            ((0.0, 1.0), (1.0, 1.0)),
            ((1.0, 1.0), (1.0, 0.0)),
        ],
        'V' => &[
            ((0.0, 0.0), (0.5, 1.0)),
            ((1.0, 0.0), (0.5, 1.0)),
        ],
        'W' => &[
            ((0.0, 0.0), (0.0, 1.0)),
            ((0.0, 1.0), (0.5, 0.5)),
            ((0.5, 0.5), (1.0, 1.0)),
            ((1.0, 1.0), (1.0, 0.0)),
        ],
        'X' => &[
            ((0.0, 0.0), (1.0, 1.0)),
            ((1.0, 0.0), (0.0, 1.0)),
        ],
        'Y' => &[
            ((0.0, 0.0), (0.5, 0.5)),
            ((0.5, 0.5), (1.0, 0.0)),
            ((0.5, 0.5), (0.5, 1.0)),
        ],
        'Z' => &[
            ((0.0, 0.0), (1.0, 0.0)),
            ((1.0, 0.0), (0.0, 1.0)),
            ((0.0, 1.0), (1.0, 1.0)),
        ],
        '[' => &[
            ((0.0, 0.0), (0.5, 0.0)),
            ((0.0, 0.0), (0.0, 1.0)),
            ((0.0, 1.0), (0.5, 1.0)),
        ],
        '\\' => &[
            ((0.0, 0.0), (1.0, 1.0)),
        ],
        ']' => &[
            ((0.5, 0.0), (1.0, 0.0)),
            ((1.0, 0.0), (1.0, 1.0)),
            ((0.5, 1.0), (1.0, 1.0)),
        ],
        '^' => &[
            ((0.5, 0.0), (0.1, 0.5)),
            ((0.5, 0.0), (0.9, 0.5)),
        ],
        '_' => &[
            ((0.0, 1.0), (1.0, 1.0)),
        ],
        '`' => &[
            ((0.2, 0.0), (0.6, 0.25)),
        ],
        'a' => &[
            ((1.0, 0.75), (0.0, 0.75)),
            ((0.0, 0.75), (0.0, 1.0)),
            ((0.0, 1.0), (1.0, 1.0)),
            ((1.0, 1.0), (1.0, 0.5)),
            ((1.0, 0.5), (0.0, 0.5)),
        ],
        'b' => &[
            ((0.0, 0.0), (0.0, 1.0)),
            ((0.0, 1.0), (1.0, 1.0)),
            ((1.0, 1.0), (1.0, 0.5)),
            ((1.0, 0.5), (0.0, 0.5)),
        ],
        'c' => &[
            ((0.0, 0.5), (1.0, 0.5)),
            ((0.0, 0.5), (0.0, 1.0)),
            ((0.0, 1.0), (1.0, 1.0)),
        ],
        'd' => &[
            ((1.0, 0.0), (1.0, 1.0)),
            ((1.0, 1.0), (0.0, 1.0)),
            ((0.0, 1.0), (0.0, 0.5)),
            ((0.0, 0.5), (1.0, 0.5)),
        ],
        'e' => &[
            ((0.0, 0.75), (1.0, 0.75)),
            ((1.0, 0.75), (1.0, 0.5)),
            ((1.0, 0.5), (0.0, 0.5)),
            ((0.0, 0.5), (0.0, 1.0)),
            ((0.0, 1.0), (1.0, 1.0)),
        ],
        'f' => &[
            ((0.5, 0.0), (0.5, 1.0)),
            ((0.5, 0.0), (1.0, 0.0)),
            ((0.0, 0.38), (1.0, 0.38)),
        ],
        'g' => &[
            ((0.0, 0.5), (1.0, 0.5)),
            ((0.0, 0.5), (0.0, 1.0)),
            ((0.0, 1.0), (1.0, 1.0)),
            ((1.0, 0.5), (1.0, 1.5)),
            ((1.0, 1.5), (0.0, 1.5)),
        ],
        'h' => &[
            ((0.0, 0.0), (0.0, 1.0)),
            ((0.0, 0.5), (1.0, 0.5)),
            ((1.0, 0.5), (1.0, 1.0)),
        ],
        'i' => &[
            ((0.5, 0.2), (0.5, 0.3)),
            ((0.5, 0.5), (0.5, 1.0)),
            ((0.3, 0.5), (0.5, 0.5)),
            ((0.2, 1.0), (0.8, 1.0)),
        ],
        'j' => &[
            ((1.0, 0.5), (1.0, 1.5)),
            ((1.0, 1.5), (0.0, 1.5)),
            ((0.0, 1.5), (0.0, 4./3.)),
            ((1.0, 0.2), (1.0, 0.3)),
        ],
        'k' => &[
            ((0.0, 0.0), (0.0, 1.0)),
            ((0.0, 0.7), (1.0, 0.4)),
            ((0.0, 0.7), (1.0, 1.0)),
        ],
        'l' => &[
            ((0.5, 0.0), (0.5, 1.0)),
        ],
        'm' => &[
            ((1.0, 0.5), (1.0, 1.0)),
            ((0.0, 0.5), (0.0, 1.0)),
            ((0.0, 0.5), (1.0, 0.5)),
            ((0.5, 0.5), (0.5, 1.0)),
        ],
        'n' => &[
            ((1.0, 0.5), (1.0, 1.0)),
            ((0.0, 0.5), (0.0, 1.0)),
            ((0.0, 0.5), (1.0, 0.5)),
        ],
        'o' => &[
            ((0.0, 0.5), (0.0, 1.0)),
            ((0.0, 1.0), (1.0, 1.0)),
            ((1.0, 1.0), (1.0, 0.5)),
            ((1.0, 0.5), (0.0, 0.5)),
        ],
        'p' => &[
            ((0.0, 0.5), (0.0, 1.5)),
            ((0.0, 0.5), (1.0, 0.5)),
            ((1.0, 0.5), (1.0, 1.0)),
            ((1.0, 1.0), (0.0, 1.0)),
        ],
        'q' => &[
            ((0.0, 0.5), (1.0, 0.5)),
            ((0.0, 0.5), (0.0, 1.0)),
            ((0.0, 1.0), (1.0, 1.0)),
            ((1.0, 0.5), (1.0, 1.5)),
            ((1.0, 1.5), (1.2, 1.3)),
        ],
        'r' => &[
            ((0.0, 0.5), (0.0, 1.0)),
            ((0.0, 0.5), (1.0, 0.5)),
        ],
        's' => &[
            ((0.0, 0.5), (1.0, 0.5)),
            ((0.0, 0.5), (0.0, 0.75)),
            ((0.0, 0.75), (1.0, 0.75)),
            ((1.0, 0.75), (1.0, 1.0)),
            ((1.0, 1.0), (0.0, 1.0)),
        ],
        't' => &[
            ((0.5, 0.0), (0.5, 1.0)),
            ((0.0, 0.38), (1.0, 0.38)),
        ],
        'u' => &[
            ((0.0, 0.5), (0.0, 1.0)),
            ((0.0, 1.0), (1.0, 1.0)),
            ((1.0, 1.0), (1.0, 0.5)),
        ],
        'v' => &[
            ((0.0, 0.5), (0.5, 1.0)),
            ((0.5, 1.0), (1.0, 0.5)),
        ],
        'w' => &[
            ((0.0, 0.5), (0.0, 1.0)),
            ((0.0, 1.0), (1.0, 1.0)),
            ((1.0, 1.0), (1.0, 0.5)),
            ((0.5, 1.0), (0.5, 0.7)),
        ],
        'x' => &[
            ((0.0, 0.5), (1.0, 1.0)),
            ((1.0, 0.5), (0.0, 1.0)),
        ],
        'y' => &[
            ((0.0, 0.5), (0.5, 1.0)),
            ((1.0, 0.5), (0.0, 1.5)),
        ],
        'z' => &[
            ((0.0, 0.5), (1.0, 0.5)),
            ((1.0, 0.5), (0.0, 1.0)),
            ((0.0, 1.0), (1.0, 1.0)),
        ],
        '{' => &[
            ((0.5, 0.0), (0.25, 0.0)),
            ((0.25, 0.0), (0.25, 0.4)),
            ((0.25, 0.4), (0.0, 0.5)),
            ((0.0, 0.5), (0.25, 0.6)),
            ((0.25, 0.6), (0.25, 1.0)),
            ((0.25, 1.0), (0.5, 1.0)),
        ],
        '|' => &[
            ((0.5, 0.0), (0.5, 1.0)),
        ],
        '}' => &[
            ((0.5, 0.0), (0.75, 0.0)),
            ((0.75, 0.0), (0.75, 0.4)),
            ((0.75, 0.4), (1.0, 0.5)),
            ((1.0, 0.5), (0.75, 0.6)),
            ((0.75, 0.6), (0.75, 1.0)),
            ((0.75, 1.0), (0.5, 1.0)),
        ],
        '~' => &[
            ((0.0, 0.5), (0.25, 0.4)),
            ((0.25, 0.4), (0.75, 0.6)),
            ((0.75, 0.6), (1.0, 0.5)),
        ],
        _ => &[],
    }
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
