// Ingest a spritesheet of noteskins, with metadata describing the notes.

use crate::sprites::Drawable;
use image::{DynamicImage, GenericImageView, SubImage};
use rrr_core::chart::NoteColor;

pub(crate) struct Definition {
    pub note_width: usize,
    pub note_height: usize,
    pub color_indices: Vec<NoteColor>,
    pub rotations: Vec<usize>,
    pub image: DynamicImage,
    pub rows: usize,
}

impl Default for Definition {
    fn default() -> Self {
        let noteskin_bytes = Self::get_embedded_noteskin();
        let noteskin_image = image::load_from_memory(noteskin_bytes).ok().unwrap();

        Self {
            note_width: 64,
            note_height: 64,
            color_indices: [
                NoteColor::Blue,
                NoteColor::Orange,
                NoteColor::Red,
                NoteColor::Cyan,
                NoteColor::Pink,
                NoteColor::White,
                NoteColor::Green,
                NoteColor::Purple,
                NoteColor::Yellow,
                NoteColor::Receptor,
            ]
            .to_vec(),
            rotations: [0, 90, 180, 270].to_vec(),
            image: noteskin_image,
            rows: 3,
        }
    }
}

pub(crate) struct Note<'a> {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) color: NoteColor,
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
    fn get_embedded_noteskin() -> &'static [u8] {
        include_bytes!("../../../data/default_noteskin.png")
    }

    pub(crate) fn new(
        note_width: usize,
        note_height: usize,
        color_indices: Vec<NoteColor>,
        rotations: Vec<usize>,
        image: DynamicImage,
        rows: usize,
    ) -> Self {
        Self {
            note_width,
            note_height,
            color_indices,
            rotations,
            image,
            rows,
        }
    }

    pub fn get_rotations(&self) -> &[usize] {
        &self.rotations
    }

    pub(crate) fn get_note(&self, color: NoteColor) -> Note<'_> {
        let width = self.note_width;
        let height = self.note_height;
        let color_index = self.color_indices.iter().position(|c| *c == color).unwrap();
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
    use super::*;
    use image::ImageFormat;
    use rrr_core::strum::IntoEnumIterator;

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
                NoteColor::Blue,
                NoteColor::Orange,
                NoteColor::Red,
                NoteColor::Cyan,
                NoteColor::Pink,
                NoteColor::White,
                NoteColor::Green,
                NoteColor::Purple,
                NoteColor::Yellow,
                NoteColor::Receptor,
            ]
            .to_vec(),
            [0, 90, 180, 270].to_vec(),
            noteskin_image,
            3,
        );

        for color in NoteColor::iter() {
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

        for color in NoteColor::iter() {
            match std::fs::remove_file(format!("{color:?}.png")) {
                Ok(_) => {}
                Err(err) => {
                    log::error!("Could not remove temp directory: {}", err);
                }
            }
        }
    }
}
