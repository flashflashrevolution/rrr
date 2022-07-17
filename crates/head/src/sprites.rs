use crate::{Point, HEIGHT, WIDTH};
use rrr::Direction;
use std::rc::Rc;
use std::time::Duration;

// This is the type stored in the `Assets` hash map
pub(crate) type CachedSprite = (usize, usize, Rc<[u8]>);

/// SpriteRef can be drawn and animated.
///
/// They reference their pixel data (instead of owning it).
#[derive(Debug)]
pub(crate) struct Sprite {
    width: usize,
    height: usize,
    sheet: Rc<[u8]>,
    index: usize,
}

/// Drawables can be blitted to the pixel buffer and animated.
pub(crate) trait Drawable {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn pixels(&self) -> &[u8];
}

impl Sprite {
    pub(crate) fn new(
        sprite_sheet: &Rc<[u8]>,
        width: usize,
        height: usize,
        index: usize,
    ) -> Sprite {
        Sprite {
            width,
            height,
            sheet: Rc::clone(sprite_sheet),
            index,
        }
    }
}

impl Drawable for Sprite {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn pixels(&self) -> &[u8] {
        &self.sheet
    }
}

/// Blit a drawable to the pixel buffer.
pub(crate) fn blit<S>(screen: &mut [u8], dest: &Point, sprite: &S)
where
    S: Drawable,
{
    assert!(dest.x + sprite.width() <= WIDTH.try_into().unwrap());
    assert!(dest.y + sprite.height() <= HEIGHT.try_into().unwrap());

    let pixels = sprite.pixels();
    let width = sprite.width() * 4;

    let mut s = 0;
    for y in 0..sprite.height() {
        let i = dest.x * 4 + dest.y * WIDTH as usize * 4 + y * WIDTH as usize * 4;

        // Merge pixels from sprite into screen
        let zipped = screen[i..i + width].iter_mut().zip(&pixels[s..s + width]);
        for (left, &right) in zipped {
            if right > 0 {
                *left = right;
            }
        }

        s += width;
    }
}
