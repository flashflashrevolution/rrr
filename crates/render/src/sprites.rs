use image::{DynamicImage, GenericImageView, SubImage};

pub enum Direction {
    Left,
    Down,
    Up,
    Right,
}

pub trait DirectionValue {
    fn value(&self) -> Direction;
}

impl DirectionValue for Direction {
    fn value(&self) -> Direction {
        match self {
            Direction::Left => Direction::Left,
            Direction::Down => Direction::Down,
            Direction::Up => Direction::Up,
            Direction::Right => Direction::Right,
        }
    }
}

/// Drawables can be blitted to the pixel buffer and animated.
pub trait Drawable<'a> {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn pixels(&self) -> SubImage<&'a DynamicImage>;
}

pub fn blit<'a, S, NoteDir: DirectionValue>(
    screen: &mut [u8],
    screen_width: u32,
    screen_height: u32,
    dest_x: f32,
    dest_y: f32,
    dir: &NoteDir,
    drawable: &S,
) where
    S: Drawable<'a>,
{
    let pixels = drawable.pixels();
    let width: f32 = drawable.width() as f32;
    let height: f32 = drawable.height() as f32;

    let x_min = f32::max(0., dest_x);
    let x_max = f32::min(screen_width as f32, dest_x + width);
    let y_min = f32::max(0., dest_y);
    let y_max = f32::min(screen_height as f32, dest_y + height);

    let x_min_u: usize = x_min.round() as usize;
    let x_max_u: usize = x_max.round() as usize;
    let y_min_u: usize = y_min.round() as usize;
    let y_max_u: usize = y_max.round() as usize;

    for screen_y in y_min_u..y_max_u {
        for screen_x in x_min_u..x_max_u {
            let i: usize = (screen_y * (screen_width as usize) + screen_x) * 4;

            // I make no guarantees that this will work with a non-square drawable
            let mut source_x: u32;
            let mut source_y: u32;
            match dir.value() {
                Direction::Down => {
                    source_x = ((screen_x as f32) - dest_x).round() as u32;
                    source_y = ((screen_y as f32) - dest_y).round() as u32;
                }
                Direction::Right => {
                    source_x = (width - 1. - ((screen_y as f32) - dest_y)).round() as u32;
                    source_y = ((screen_x as f32) - dest_x).round() as u32;
                }
                Direction::Up => {
                    source_x = (width - 1. - ((screen_x as f32) - dest_x)).round() as u32;
                    source_y = (height - 1. - ((screen_y as f32) - dest_y)).round() as u32;
                }
                Direction::Left => {
                    source_x = ((screen_y as f32) - dest_y).round() as u32;
                    source_y = (height - 1. - ((screen_x as f32) - dest_x)).round() as u32;
                }
            }

            source_x = source_x.clamp(0, drawable.width() as u32 - 1);
            source_y = source_y.clamp(0, drawable.height() as u32 - 1);

            let source_pixel = pixels.get_pixel(source_x, source_y);

            if source_pixel[3] != 0 {
                screen[i + 3] = 255
                    - ((255 - screen[i + 3]) as f32 * ((255 - source_pixel[3]) as f32) / 255.)
                        as u8;

                screen[i + 0] = ((screen[i + 0] as f32 * (255 - source_pixel[3]) as f32
                    + source_pixel[0] as f32 * source_pixel[3] as f32)
                    / 255.) as u8;

                screen[i + 1] = ((screen[i + 1] as f32 * (255 - source_pixel[3]) as f32
                    + source_pixel[1] as f32 * source_pixel[3] as f32)
                    / 255.) as u8;

                screen[i + 2] = ((screen[i + 2] as f32 * (255 - source_pixel[3]) as f32
                    + source_pixel[2] as f32 * source_pixel[3] as f32)
                    / 255.) as u8;
            }
        }
    }
}
