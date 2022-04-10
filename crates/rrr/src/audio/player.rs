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
        formats::{FormatOptions, FormatReader, Packet},
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
    remaining_samples: usize,
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
            remaining_samples: 0,
        }
    }

    pub fn tick(&mut self) -> Option<TimeStamp> {
        loop {
            // Demux an encoded packet from the media format.
            let packet = if self.remaining_samples == 0 {
                match self.reader.next_packet() {
                    Ok(packet) => Some(packet),
                    Err(SymphoniaError::IoError(io))
                        if io.kind() == io::ErrorKind::UnexpectedEof =>
                    {
                        return None; // End of this stream.
                    }
                    Err(err) => {
                        log::error!("format error: {}", err);
                        return None; // We cannot recover from format errors, quit.
                    }
                }
            } else {
                None
            };

            while !self.reader.metadata().is_latest() {
                // Consume any new metadata that has been read since the last
                // packet.
            }

            // Decode the packet into an audio buffer.
            let temp_decoded = if self.remaining_samples > 0 {
                Ok(self.decoder.last_decoded())
            } else {
                self.decoder.decode(&packet.clone().unwrap())
            };

            match temp_decoded {
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

                    let frame_count = decoded.frames();

                    let total_samples = frame_count * 2;

                    // Write the decoded audio samples to the audio output if the presentation timestamp
                    // for the packet is >= the seeked position (0 if not seeking).
                    let written = if let Some(out) = &mut self.output {
                        match out.write(decoded, total_samples, self.remaining_samples) {
                            Ok(written) => written,
                            Err(_) => 0,
                        }
                    } else {
                        0
                    };

                    let timestamp = if let Some(pak) = &packet {
                        pak.ts()
                    } else {
                        TimeStamp::default()
                    };

                    if written > 0 && self.remaining_samples > 0 {
                        self.remaining_samples -= written;
                        continue;
                    } else if written == 0 && self.remaining_samples > 0 {
                        return Some(timestamp);
                    }

                    if written < total_samples {
                        self.remaining_samples = total_samples - written;
                        return Some(timestamp);
                    }
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

    pub fn stop(&mut self) {
        let player = &mut self.output;
        player.as_mut().unwrap().flush();
    }
}

impl Debug for AudioPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Player").finish()
    }
}
