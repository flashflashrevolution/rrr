use crate::{HEIGHT, WIDTH};
use image::{DynamicImage, GenericImageView, SubImage};
use rrr_core::note::Direction;
use std::f64;

/// Drawables can be blitted to the pixel buffer and animated.
pub(crate) trait Drawable<'a> {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn pixels(&self) -> SubImage<&'a DynamicImage>;
}

pub(crate) fn blit<'a, S>(
    screen: &mut [u8],
    dest_x: f64,
    dest_y: f64,
    dir: &Direction,
    drawable: &S,
) where
    S: Drawable<'a>,
{
    let pixels = drawable.pixels();
    let width: f64 = drawable.width() as f64;
    let height: f64 = drawable.height() as f64;

    let x_min: f64 = f64::max(0., dest_x);
    let x_max: f64 = f64::min(WIDTH as f64, dest_x + width);
    let y_min: f64 = f64::max(0., dest_y);
    let y_max: f64 = f64::min(HEIGHT as f64, dest_y + height);

    let x_min_u: usize = x_min.round() as usize;
    let x_max_u: usize = x_max.round() as usize;
    let y_min_u: usize = y_min.round() as usize;
    let y_max_u: usize = y_max.round() as usize;

    for screen_y in y_min_u..y_max_u {
        for screen_x in x_min_u..x_max_u {
            let i: usize = (screen_y * (WIDTH as usize) + screen_x) * 4;

            // I make no guarantees that this will work with a non-square drawable
            let mut source_x: u32;
            let mut source_y: u32;
            match *dir {
                Direction::Down => {
                    source_x = ((screen_x as f64) - dest_x).round() as u32;
                    source_y = ((screen_y as f64) - dest_y).round() as u32;
                }
                Direction::Right => {
                    source_x = (width - 1. - ((screen_y as f64) - dest_y)).round() as u32;
                    source_y = ((screen_x as f64) - dest_x).round() as u32;
                }
                Direction::Up => {
                    source_x = (width - 1. - ((screen_x as f64) - dest_x)).round() as u32;
                    source_y = (height - 1. - ((screen_y as f64) - dest_y)).round() as u32;
                }
                Direction::Left => {
                    source_x = ((screen_y as f64) - dest_y).round() as u32;
                    source_y = (height - 1. - ((screen_x as f64) - dest_x)).round() as u32;
                }
            }

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
