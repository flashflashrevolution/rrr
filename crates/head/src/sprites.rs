use crate::{Point, HEIGHT, WIDTH};
use image::{DynamicImage, GenericImageView, Rgba, SubImage};
use rrr_core::note::Direction;
use std::f64;

/// Drawables can be blitted to the pixel buffer and animated.
pub(crate) trait Drawable<'a> {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn pixels(&self) -> SubImage<&'a DynamicImage>;
}

pub(crate) fn trumpet_blit<'a, S>(screen: &mut [u8], dest_x: f64, dest_y: f64, drawable: &S)
where
    S: Drawable<'a>,
{
    let pixels = drawable.pixels();
    let width: f64 = drawable.width() as f64;
    let height: f64 = drawable.height() as f64;

    let x_min:f64 = f64::max(0., dest_x);
    let x_max:f64 = f64::min(WIDTH as f64, dest_x + width);
    let y_min:f64 = f64::max(0., dest_y);
    let y_max:f64 = f64::min(HEIGHT as f64, dest_y + height);
    
    let x_min_u: usize = x_min.round() as usize;
    let x_max_u: usize = x_max.round() as usize;
    let y_min_u: usize = y_min.round() as usize;
    let y_max_u: usize = y_max.round() as usize;
    
    for screen_y in y_min_u..y_max_u {
        for screen_x in x_min_u..x_max_u {
            let i: usize = (screen_y * (WIDTH as usize) + screen_x) * 4;
            
            let mut source_x = ((screen_x as f64) - dest_x).round() as u32;
            let mut source_y = ((screen_y as f64) - dest_y).round() as u32;
            source_x = source_x.clamp(0, drawable.width() as u32 - 1);
            source_y = source_y.clamp(0, drawable.height() as u32 - 1);
            
            let source_pixel = pixels.get_pixel(source_x, source_y);

            screen[i] = source_pixel[0];
            screen[i + 1] = source_pixel[1];
            screen[i + 2] = source_pixel[2];
            screen[i + 3] = source_pixel[3];
        }
    }
}

/// Blit a drawable to the pixel buffer.
pub(crate) fn blit<'a, S>(screen: &mut [u8], dest: &Point, dir: &Direction, drawable: &S)
where
    S: Drawable<'a>,
{
    let drawable_width: usize = drawable.width();
    let drawable_height: usize = drawable.height();

    assert!(dest.x + drawable_width <= WIDTH.try_into().unwrap());
    assert!(dest.y + drawable_height <= HEIGHT.try_into().unwrap());

    let dest_pixel_width = drawable_width * 4;
    let pixels = drawable.pixels();

    for row in 0..drawable.height() {
        let i = match *dir {
            Direction::Up | Direction::Left => {
                dest.x * 4
                    + dest.y * WIDTH as usize * 4
                    + (drawable_height - row) * WIDTH as usize * 4
            }
            Direction::Right | Direction::Down => {
                dest.x * 4 + dest.y * WIDTH as usize * 4 + row * WIDTH as usize * 4
            }
        };

        let pixel_row = match *dir {
            Direction::Left | Direction::Right => {
                pixels.view(row as u32, 0, 1, drawable_height as u32)
            }
            Direction::Up | Direction::Down => pixels.view(0, row as u32, drawable_width as u32, 1),
        };

        let pixel_iter = pixel_row.pixels();
        match *dir {
            Direction::Left => {
                let screen_iterator = screen[i..i + dest_pixel_width]
                    .array_chunks_mut::<4>()
                    .rev()
                    .zip(pixel_iter);
                for (left, right) in screen_iterator {
                    copy_pixels(left, right);
                }
            }
            Direction::Down | Direction::Up | Direction::Right => {
                let screen_iterator = screen[i..i + dest_pixel_width]
                    .array_chunks_mut::<4>()
                    .zip(pixel_iter);
                for (left, right) in screen_iterator {
                    copy_pixels(left, right);
                }
            }
        };
    }
}

fn copy_pixels(left: &mut [u8], right: (u32, u32, Rgba<u8>)) {
    if right.2[3] > 0 && right.2[0] > 0 || right.2[1] > 0 || right.2[2] > 0 {
        let fundata = &right.2 .0[0..4];
        (*left)[0] = fundata[0];
        (*left)[1] = fundata[1];
        (*left)[2] = fundata[2];
        (*left)[3] = fundata[3];
    }
}
