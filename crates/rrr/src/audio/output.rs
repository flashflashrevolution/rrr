use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rb::{RbConsumer, RbProducer, SpscRb, RB};
use symphonia::core::{
    audio::{AudioBufferRef, RawSample, SampleBuffer, SignalSpec},
    conv::ConvertibleSample,
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

pub(crate) type Result<T> = std::result::Result<T, AudioOutputError>;

pub(crate) struct CpalAudioOutput;

trait AudioOutputSample: cpal::Sample + ConvertibleSample + RawSample + Send + 'static {}

impl AudioOutputSample for f32 {}
impl AudioOutputSample for i16 {}
impl AudioOutputSample for u16 {}

impl CpalAudioOutput {
    pub(crate) fn try_open(spec: SignalSpec, duration: Duration) -> Result<Box<dyn AudioOutput>> {
        // Get default host.
        let host = cpal::default_host();

        // Get the default audio output device.
        if let Ok(devices) = host.devices() {
            for device in devices {
                log::info!("{:?}", device.name());

                if let Ok(mut configs) = device.supported_output_configs() {
                    for config in configs.by_ref() {
                        log::info!("{:?}", config);
                    }
                }
            }
        }

        let device = if let Some(device) = host.default_output_device() {
            device
        } else {
            log::error!("failed to get default audio output device");
            return Err(AudioOutputError::OpenStreamError);
        };

        let config = match device.default_output_config() {
            Ok(config) => config,
            Err(err) => {
                log::error!("failed to get default audio output device config: {}", err);
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

struct CpalAudioOutputImpl<T>
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
            .find(|sf| sf.channels() == 2 && sf.sample_rate() == cpal::SampleRate(spec.rate))
            .ok_or(AudioOutputError::OpenStreamError)?;

        log::info!("Supported config: {:?}", config);

        let sample_rate =
            usize::try_from(spec.rate).map_err(|_| AudioOutputError::OpenStreamError)?;
        // Create a ring buffer with a capacity for up-to 200ms of audio.
        let ring_len =
            ((400_usize.saturating_mul(sample_rate)).div_euclid(1000)).saturating_mul(num_channels);

        let ring_buf = SpscRb::new(ring_len);
        let (ring_buf_producer, ring_buf_consumer) = (ring_buf.producer(), ring_buf.consumer());

        let stream_result = device.build_output_stream(
            &config.config(),
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                // Write out as many samples as possible from the ring buffer to the audio output.
                let written = match ring_buf_consumer.read(data) {
                    Ok(written) => written,
                    Err(err) => {
                        log::error!("failed to read from ring buffer: {}", err);
                        0
                    }
                };

                // Mute any remaining samples.
                data.get_mut(written..)
                    .unwrap_or_default()
                    .iter_mut()
                    .for_each(|s| *s = T::MID);
            },
            |err| log::error!("audio output error: {}", err),
        );

        let stream = stream_result.map_err(|err| {
            log::error!("audio output stream open error: {}", err);
            AudioOutputError::OpenStreamError
        })?;

        // Start the output stream.
        stream.play().map_err(|err| {
            log::error!("audio output stream play error: {}", err);
            AudioOutputError::PlayStreamError
        })?;

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
            samples = samples
                .get(total_samples.saturating_sub(remaining_samples)..)
                .ok_or(AudioOutputError::PlayStreamError)?;
        }

        let written = self.ring_buf_producer.write(samples).unwrap_or(0);

        Ok(written)
    }

    fn flush(&mut self) {
        // Flush is best-effort, ignore the returned result.
        let _res = self.stream.pause();
    }
}

pub(crate) fn try_open(spec: SignalSpec, duration: Duration) -> Result<Box<dyn AudioOutput>> {
    CpalAudioOutput::try_open(spec, duration)
}
