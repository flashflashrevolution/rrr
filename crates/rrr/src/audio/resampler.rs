use rubato::{
    InterpolationParameters, InterpolationType, ResampleError, Resampler, SincFixedIn,
    WindowFunction,
};

#[derive(Copy, Clone)]
pub struct ResamplingSpec {
    pub input_rate: u32,
    pub output_rate: u32,
    pub channels: usize,
}

impl ResamplingSpec {
    pub fn output_size(&self, input_size: usize) -> usize {
        (self.output_rate as f64 / self.input_rate as f64 * input_size as f64) as usize
    }

    pub fn input_size(&self, output_size: usize) -> usize {
        (self.input_rate as f64 / self.output_rate as f64 * output_size as f64) as usize
    }

    pub fn ratio(&self) -> f64 {
        self.output_rate as f64 / self.input_rate as f64
    }
}

pub struct AudioResampler {
    pub spec: ResamplingSpec,
    pub resampler: SincFixedIn<f32>,
}

impl AudioResampler {
    pub fn new(&self, spec: ResamplingSpec) -> Self {
        let params = InterpolationParameters {
            sinc_len: 256,
            f_cutoff: 0.95,
            interpolation: InterpolationType::Linear,
            oversampling_factor: 256,
            window: WindowFunction::BlackmanHarris2,
        };
        let resampler =
            SincFixedIn::<f32>::new(48000 as f64 / 44100 as f64, 2.0, params, 1024, 2).unwrap();

        Self { spec, resampler }
    }

    pub fn process(
        &mut self,
        input: &Vec<Vec<f32>>,
        output: &mut Vec<Vec<f32>>,
    ) -> Result<(usize, usize), ResampleError> {
        // Bypass conversion completely in case the sample rates are equal.
        if self.spec.input_rate == self.spec.output_rate {
            let output = &mut output[..input.len()];
            for i in 0..1 {
                output[i].resize(input[i].len(), 0.0);
                output[i].copy_from_slice(&input[i]);
            }
            return Ok((input.len(), output.len()));
        }

        if let Err(error) = self.resampler.process_into_buffer(&input, output, None) {
            Err(error)
        } else {
            Ok((input.len(), output.len()))
        }
    }
}
