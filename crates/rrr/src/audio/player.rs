use super::output::AudioOutput;
use crate::audio::output;
use log::info;
use std::{
    fmt::Debug,
    io::{self, Cursor},
};
use symphonia::{
    core::{
        codecs::{Decoder, DecoderOptions},
        errors::Error as SymphoniaError,
        formats::{FormatOptions, FormatReader},
        io::{MediaSourceStream, MediaSourceStreamOptions},
        units::TimeStamp,
    },
    default::{codecs::Mp3Decoder, formats::Mp3Reader},
};

pub struct AudioPlayer {
    output: Option<Box<dyn AudioOutput>>,
    decoder: Box<Mp3Decoder>,
    reader: Box<Mp3Reader>,
    track_id: u32,
}

impl AudioPlayer {
    pub fn new(mp3: &Vec<u8>) -> Self {
        let mss = MediaSourceStream::new(
            Box::new(Cursor::new(mp3.to_vec())),
            MediaSourceStreamOptions::default(),
        );
        let reader = Box::new(Mp3Reader::try_new(mss, &FormatOptions::default()).unwrap());
        let track = reader.default_track().unwrap();
        let decoder =
            Box::new(Mp3Decoder::try_new(&track.codec_params, &DecoderOptions::default()).unwrap());

        Self {
            decoder,
            output: None,
            track_id: track.id,
            reader,
        }
    }

    pub fn tick(&mut self) -> Option<TimeStamp> {
        loop {
            // Demux an encoded packet from the media format.
            let packet = match self.reader.next_packet() {
                Ok(packet) => packet,
                Err(SymphoniaError::IoError(io)) if io.kind() == io::ErrorKind::UnexpectedEof => {
                    return None; // End of this stream.
                }
                Err(err) => {
                    log::error!("format error: {}", err);
                    return None; // We cannot recover from format errors, quit.
                }
            };

            while !self.reader.metadata().is_latest() {
                // Consume any new metadata that has been read since the last
                // packet.
            }

            // If the packet does not belong to the selected track, skip over it.
            if packet.track_id() != self.track_id {
                continue;
            }

            // Decode the packet into an audio buffer.
            match self.decoder.decode(&packet) {
                Ok(decoded) => {
                    // If the audio output is not open, try to open it.
                    if self.output.is_none() {
                        // Get the audio buffer specification. This is a description of the decoded
                        // audio buffer's sample format and sample rate.
                        let spec = *decoded.spec();

                        // Get the capacity of the decoded buffer. Note that this is capacity, not
                        // length! The capacity of the decoded buffer is constant for the life of the
                        // decoder, but the length is not.
                        let duration = decoded.capacity() as u64;

                        // Try to open the audio output.
                        self.output
                            .replace(output::try_open(spec, duration).unwrap());
                    }

                    // Write the decoded audio samples to the audio output if the presentation timestamp
                    // for the packet is >= the seeked position (0 if not seeking).
                    info!("write buffer to device");
                    if let Some(out) = &mut self.output {
                        out.write(decoded).unwrap()
                    }

                    return Some(packet.ts());
                }
                Err(SymphoniaError::IoError(err)) => {
                    // The packet failed to decode due to an IO error, skip the packet.
                    log::error!("io decode error: {}", err);
                    continue;
                }
                Err(SymphoniaError::DecodeError(err)) => {
                    // The packet failed to decode due to invalid data, skip the packet.
                    log::error!("decode error: {}", err);
                    continue;
                }
                Err(err) => {
                    log::error!("fatal decode error: {}", err);
                    return None;
                }
            };
        }
    }
}

impl Debug for AudioPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Player").finish()
    }
}
