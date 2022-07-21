use crate::{Point, HEIGHT, WIDTH};
use image::{DynamicImage, GenericImageView, Rgba, SubImage};
use rrr::Direction;
use std::rc::Rc;
use std::time::Duration;

/// Drawables can be blitted to the pixel buffer and animated.
pub(crate) trait Drawable<'a> {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn pixels(&self) -> SubImage<&'a DynamicImage>;
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
    let width = drawable.width() * 4;

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

        // let raw_image = DynamicImage::from(pixel_row.to_image());
        // let rot_180 = raw_image.rotate180();
        // let itr2 = rot_180.pixels();
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
