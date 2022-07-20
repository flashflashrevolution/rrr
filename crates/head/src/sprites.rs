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
pub(crate) fn blit<'a, S>(screen: &mut [u8], dest: &Point, drawable: &S)
where
    S: Drawable<'a>,
{
    assert!(dest.x + drawable.width() <= WIDTH.try_into().unwrap());
    assert!(dest.y + drawable.height() <= HEIGHT.try_into().unwrap());

    let pixels = drawable.pixels();
    let width = drawable.width() * 4;

    let mut s = 0;
    for row in 0..drawable.height() {
        let i = dest.x * 4
            + dest.y * WIDTH as usize * 4
            + (drawable.height() - row) * WIDTH as usize * 4;

        let pixel_row = pixels.view(0, row as u32, drawable.width() as u32, 1);
        // let raw_image = DynamicImage::from(pixel_row.to_image());
        // let rot_180 = raw_image.rotate180();
        // let itr2 = rot_180.pixels();
        let pixel_iter = pixel_row.pixels();

        // Merge pixels from sprite into screen
        let zipped = screen[i..i + width].array_chunks_mut::<4>().zip(pixel_iter);
        for (left, right) in zipped {
            if right.2[3] > 0 && right.2[0] > 0 || right.2[1] > 0 || right.2[2] > 0 {
                let fundata = &right.2 .0[0..4];
                (*left)[0] = fundata[0];
                (*left)[1] = fundata[1];
                (*left)[2] = fundata[2];
                (*left)[3] = fundata[3];
            }
        }

        s += width;
    }
}
