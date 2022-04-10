use std::result;
use symphonia::core::{
    audio::{AudioBufferRef, SignalSpec},
    units::Duration,
};

pub(crate) trait AudioOutput {
    fn write(
        &mut self,
        decoded: AudioBufferRef<'_>,
        total_samples: usize,
        remaining_samples: usize,
    ) -> Result<usize>;

    fn flush(&mut self);
}

#[allow(dead_code)]
#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub(crate) enum AudioOutputError {
    OpenStreamError,
    PlayStreamError,
    StreamClosedError,
}

pub(crate) type Result<T> = result::Result<T, AudioOutputError>;

#[cfg(target_os = "linux")]
mod pulseaudio {
    use super::{AudioOutput, AudioOutputError, Result};

    use symphonia::core::audio::*;
    use symphonia::core::units::Duration;

    use libpulse_binding as pulse;
    use libpulse_simple_binding as psimple;

    use log::{error, warn};

    pub struct PulseAudioOutput {
        pa: psimple::Simple,
        sample_buf: RawSampleBuffer<f32>,
    }

    impl PulseAudioOutput {
        pub fn try_open(spec: SignalSpec, duration: Duration) -> Result<Box<dyn AudioOutput>> {
            // An interleaved buffer is required to send data to PulseAudio. Use a SampleBuffer to
            // move data between Symphonia AudioBuffers and the byte buffers required by PulseAudio.
            let sample_buf = RawSampleBuffer::<f32>::new(duration, spec);

            // Create a PulseAudio stream specification.
            let pa_spec = pulse::sample::Spec {
                format: pulse::sample::Format::FLOAT32NE,
                channels: spec.channels.count() as u8,
                rate: spec.rate,
            };

            assert!(pa_spec.is_valid());

            let pa_ch_map = map_channels_to_pa_channelmap(spec.channels);

            // PulseAudio seems to not play very short audio buffers, use these custom buffer
            // attributes for very short audio streams.
            //
            // let pa_buf_attr = pulse::def::BufferAttr {
            //     maxlength: std::u32::MAX,
            //     tlength: 1024,
            //     prebuf: std::u32::MAX,
            //     minreq: std::u32::MAX,
            //     fragsize: std::u32::MAX,
            // };

            // Create a PulseAudio connection.
            let pa_result = psimple::Simple::new(
                None,                               // Use default server
                "Symphonia Player",                 // Application name
                pulse::stream::Direction::Playback, // Playback stream
                None,                               // Default playback device
                "Music",                            // Description of the stream
                &pa_spec,                           // Signal specificaiton
                pa_ch_map.as_ref(),                 // Channel map
                None,                               // Custom buffering attributes
            );

            match pa_result {
                Ok(pa) => Ok(Box::new(PulseAudioOutput { pa, sample_buf })),
                Err(err) => {
                    error!("audio output stream open error: {}", err);

                    Err(AudioOutputError::OpenStreamError)
                }
            }
        }
    }

    impl AudioOutput for PulseAudioOutput {
        fn write(&mut self, decoded: AudioBufferRef<'_>) -> Result<()> {
            // Do nothing if there are no audio frames.
            if decoded.frames() == 0 {
                return Ok(());
            }

            // Interleave samples from the audio buffer into the sample buffer.
            self.sample_buf.copy_interleaved_ref(decoded);

            // Write interleaved samples to PulseAudio.
            match self.pa.write(self.sample_buf.as_bytes()) {
                Err(err) => {
                    error!("audio output stream write error: {}", err);

                    Err(AudioOutputError::StreamClosedError)
                }
                _ => Ok(()),
            }
        }

        fn flush(&mut self) {
            // Flush is best-effort, ignore the returned result.
            let _ = self.pa.drain();
        }
    }

    /// Maps a set of Symphonia `Channels` to a PulseAudio channel map.
    fn map_channels_to_pa_channelmap(channels: Channels) -> Option<pulse::channelmap::Map> {
        let mut map: pulse::channelmap::Map = Default::default();
        map.init();
        map.set_len(channels.count() as u8);

        let is_mono = channels.count() == 1;

        for (i, channel) in channels.iter().enumerate() {
            map.get_mut()[i] = match channel {
                Channels::FRONT_LEFT if is_mono => pulse::channelmap::Position::Mono,
                Channels::FRONT_LEFT => pulse::channelmap::Position::FrontLeft,
                Channels::FRONT_RIGHT => pulse::channelmap::Position::FrontRight,
                Channels::FRONT_CENTRE => pulse::channelmap::Position::FrontCenter,
                Channels::REAR_LEFT => pulse::channelmap::Position::RearLeft,
                Channels::REAR_CENTRE => pulse::channelmap::Position::RearCenter,
                Channels::REAR_RIGHT => pulse::channelmap::Position::RearRight,
                Channels::LFE1 => pulse::channelmap::Position::Lfe,
                Channels::FRONT_LEFT_CENTRE => pulse::channelmap::Position::FrontLeftOfCenter,
                Channels::FRONT_RIGHT_CENTRE => pulse::channelmap::Position::FrontRightOfCenter,
                Channels::SIDE_LEFT => pulse::channelmap::Position::SideLeft,
                Channels::SIDE_RIGHT => pulse::channelmap::Position::SideRight,
                Channels::TOP_CENTRE => pulse::channelmap::Position::TopCenter,
                Channels::TOP_FRONT_LEFT => pulse::channelmap::Position::TopFrontLeft,
                Channels::TOP_FRONT_CENTRE => pulse::channelmap::Position::TopFrontCenter,
                Channels::TOP_FRONT_RIGHT => pulse::channelmap::Position::TopFrontRight,
                Channels::TOP_REAR_LEFT => pulse::channelmap::Position::TopRearLeft,
                Channels::TOP_REAR_CENTRE => pulse::channelmap::Position::TopRearCenter,
                Channels::TOP_REAR_RIGHT => pulse::channelmap::Position::TopRearRight,
                _ => {
                    // If a Symphonia channel cannot map to a PulseAudio position then return None
                    // because PulseAudio will not be able to open a stream with invalid channels.
                    warn!("failed to map channel {:?} to output", channel);
                    return None;
                }
            }
        }

        Some(map)
    }
}

#[cfg(not(target_os = "linux"))]
mod cpal {
    use super::{AudioOutput, AudioOutputError, Result};
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    use log::{error, info};
    use rb::{RbConsumer, RbProducer, SpscRb, RB};
    use symphonia::core::audio::{AudioBufferRef, RawSample, SampleBuffer, SignalSpec};
    use symphonia::core::conv::ConvertibleSample;
    use symphonia::core::units::Duration;

    pub(crate) struct CpalAudioOutput;

    trait AudioOutputSample:
        cpal::Sample + ConvertibleSample + RawSample + std::marker::Send + 'static
    {
    }

    impl AudioOutputSample for f32 {}
    impl AudioOutputSample for i16 {}
    impl AudioOutputSample for u16 {}

    impl CpalAudioOutput {
        pub(crate) fn try_open(
            spec: SignalSpec,
            duration: Duration,
        ) -> Result<Box<dyn AudioOutput>> {
            // Get default host.
            let host = cpal::default_host();

            // Get the default audio output device.
            if let Ok(devices) = host.devices() {
                for device in devices {
                    println!("{:?}", device.name());

                    if let Ok(mut configs) = device.supported_output_configs() {
                        for config in configs.by_ref() {
                            println!("{:?}", config);
                        }
                    }
                }
            }

            let device = if let Some(device) = host.default_output_device() {
                device
            } else {
                error!("failed to get default audio output device");
                return Err(AudioOutputError::OpenStreamError);
            };

            let config = match device.default_output_config() {
                Ok(config) => config,
                Err(err) => {
                    error!("failed to get default audio output device config: {}", err);
                    return Err(AudioOutputError::OpenStreamError);
                }
            };

            // Select proper playback routine based on sample format.
            match config.sample_format() {
                cpal::SampleFormat::F32 => {
                    CpalAudioOutputImpl::<f32>::try_open(spec, duration, &device)
                }
                cpal::SampleFormat::I16 => {
                    CpalAudioOutputImpl::<i16>::try_open(spec, duration, &device)
                }
                cpal::SampleFormat::U16 => {
                    CpalAudioOutputImpl::<u16>::try_open(spec, duration, &device)
                }
            }
        }
    }

    struct CpalAudioOutputImpl<T: AudioOutputSample>
    where
        T: AudioOutputSample,
    {
        ring_buf_producer: rb::Producer<T>,
        sample_buf: SampleBuffer<T>,
        stream: cpal::Stream,
    }

    impl<T: AudioOutputSample> CpalAudioOutputImpl<T> {
        pub(crate) fn try_open(
            spec: SignalSpec,
            duration: Duration,
            device: &cpal::Device,
        ) -> Result<Box<dyn AudioOutput>> {
            const HZ_44100: cpal::SampleRate = cpal::SampleRate(44_100);

            let num_channels = spec.channels.count();
            let mut supported: Vec<_> = device.supported_output_configs().unwrap().collect();
            supported.sort_by(|a, b| b.cmp_default_heuristics(a));

            let mut formats = supported.into_iter().flat_map(|sf| {
                let max_rate = sf.max_sample_rate();
                let min_rate = sf.min_sample_rate();
                let mut formats = vec![sf.clone().with_max_sample_rate()];
                if HZ_44100 < max_rate && HZ_44100 > min_rate {
                    formats.push(sf.clone().with_sample_rate(HZ_44100));
                }
                formats.push(sf.with_sample_rate(min_rate));
                formats
            });

            let config = formats
                .find(|sf| {
                    sf.channels() as usize == 2 && sf.sample_rate() == cpal::SampleRate(spec.rate)
                })
                .ok_or(AudioOutputError::OpenStreamError)?;
            info!("Supported config: {:?}", config);

            // Create a ring buffer with a capacity for up-to 200ms of audio.
            let ring_len = ((400 * spec.rate as usize) / 1000) * num_channels;

            let ring_buf = SpscRb::new(ring_len);
            let (ring_buf_producer, ring_buf_consumer) = (ring_buf.producer(), ring_buf.consumer());

            let stream_result = device.build_output_stream(
                &config.config(),
                move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                    // Write out as many samples as possible from the ring buffer to the audio
                    // output.
                    let written = match ring_buf_consumer.read(data) {
                        Ok(written) => written,
                        Err(err) => {
                            error!("failed to read from ring buffer: {}", err);
                            0
                        }
                    };

                    //println!("Read {} samples.", written);

                    // Mute any remaining samples.
                    data[written..].iter_mut().for_each(|s| *s = T::MID);
                },
                move |err| error!("audio output error: {}", err),
            );

            if let Err(err) = stream_result {
                error!("audio output stream open error: {}", err);

                return Err(AudioOutputError::OpenStreamError);
            }

            let stream = stream_result.unwrap();

            // Start the output stream.
            if let Err(err) = stream.play() {
                error!("audio output stream play error: {}", err);

                return Err(AudioOutputError::PlayStreamError);
            }

            let sample_buf = SampleBuffer::<T>::new(duration, spec);

            Ok(Box::new(CpalAudioOutputImpl {
                ring_buf_producer,
                sample_buf,
                stream,
            }))
        }
    }

    impl<T: AudioOutputSample> AudioOutput for CpalAudioOutputImpl<T> {
        fn write(
            &mut self,
            decoded: AudioBufferRef<'_>,
            total_samples: usize,
            remaining_samples: usize,
        ) -> Result<usize> {
            // Audio samples must be interleaved for cpal. Interleave the samples in the audio
            // buffer into the sample buffer.
            self.sample_buf.copy_interleaved_ref(decoded);

            // Write all the interleaved samples to the ring buffer.
            let mut samples = self.sample_buf.samples();

            if remaining_samples > 0 {
                samples = &samples[total_samples - remaining_samples..];
            }

            let written = match self.ring_buf_producer.write(samples) {
                Ok(written) => written,
                Err(rb::RbError::Full) => 0,
                Err(err) => 0,
            };

            Ok(written)
        }

        fn flush(&mut self) {
            // Flush is best-effort, ignore the returned result.
            let _res = self.stream.pause();
        }
    }
}

#[cfg(target_os = "linux")]
pub fn try_open(spec: SignalSpec, duration: Duration) -> Result<Box<dyn AudioOutput>> {
    pulseaudio::PulseAudioOutput::try_open(spec, duration)
}

#[cfg(not(target_os = "linux"))]
pub(crate) fn try_open(spec: SignalSpec, duration: Duration) -> Result<Box<dyn AudioOutput>> {
    cpal::CpalAudioOutput::try_open(spec, duration)
}
