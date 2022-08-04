// Ingest a spritesheet of noteskins, with metadata describing the notes.

use image::{imageops, DynamicImage, GenericImageView, Pixels, SubImage};
use rrr_core::note::Color;

use crate::sprites::Drawable;

pub(crate) struct Definition {
    pub note_width: usize,
    pub note_height: usize,
    pub color_indexs: Vec<Color>,
    pub rotations: Vec<usize>,
    pub image: DynamicImage,
    pub rows: usize,
}

pub(crate) struct Note<'a> {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) color: Color,
    pub(crate) image: SubImage<&'a DynamicImage>,
}

impl<'a> Drawable<'a> for Note<'a> {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn pixels(&self) -> SubImage<&'a DynamicImage> {
        self.image
    }
}

impl Definition {
    pub(crate) fn new(
        note_width: usize,
        note_height: usize,
        color_indexs: Vec<Color>,
        rotations: Vec<usize>,
        image: DynamicImage,
        rows: usize,
    ) -> Self {
        Self {
            note_width,
            note_height,
            color_indexs,
            rotations,
            image,
            rows,
        }
    }

    pub fn get_rotations(&self) -> &[usize] {
        &self.rotations
    }

    pub(crate) fn get_note(&self, color: Color) -> Note<'_> {
        let width = self.note_width;
        let height = self.note_height;
        let color_index = self.color_indexs.iter().position(|c| *c == color).unwrap();
        let row_offset = (height * color_index) % (self.rows * height);
        let col_offset = (width * color_index) / (self.rows * width) * width;
        let view = self.image.view(
            col_offset.try_into().unwrap(),
            row_offset.try_into().unwrap(),
            width.try_into().unwrap(),
            height.try_into().unwrap(),
        );
        return Note {
            width,
            height,
            color,
            image: view,
        };
    }
}

// rust test block for get_note
#[cfg(test)]
mod tests {
    use std::env;

    use image::{
        imageops, ColorType, DynamicImage, GenericImage, GenericImageView, ImageBuffer,
        ImageFormat, RgbImage,
    };
    use rrr_core::strum::{EnumIter, IntoEnumIterator};

    use super::*;

    #[test]
    fn test_get_note() {
        // TODO: Convert this include_bytes to FetchWorker. (Also change FetchWorker to BinaryFetchWorker)
        let noteskin_bytes = include_bytes!("../../../data/default_noteskin.png");
        let mut noteskin_image = match image::load_from_memory(noteskin_bytes) {
            Ok(image) => image,
            Err(err) => {
                log::error!("Could not load noteskin: {}", err);
                return;
            }
        };

        let mut definition = Definition::new(
            64,
            64,
            [
                Color::Blue,
                Color::Orange,
                Color::Red,
                Color::Cyan,
                Color::Pink,
                Color::White,
                Color::Green,
                Color::Purple,
                Color::Yellow,
                Color::Receptor,
            ]
            .to_vec(),
            [0, 90, 180, 270].to_vec(),
            noteskin_image,
            3,
        );

        for color in Color::iter() {
            let note = definition.get_note(color);
            assert_eq!(note.color, color);
            assert_eq!(note.width, 64);
            assert_eq!(note.height, 64);

            let image_out = note
                .image
                .to_image()
                .save_with_format(format!("{color:?}.png"), ImageFormat::Png);
            assert!(image_out.is_ok());
        }

        for color in Color::iter() {
            match std::fs::remove_file(format!("{color:?}.png")) {
                Ok(_) => {}
                Err(err) => {
                    log::error!("Could not remove temp directory: {}", err);
                }
            }
        }
    }
}
