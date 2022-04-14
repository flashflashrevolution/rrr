use rubato::{
    InterpolationParameters, InterpolationType, ResampleError, Resampler,
    ResamplerConstructionError, SincFixedIn, WindowFunction,
};

#[derive(Copy, Clone)]
pub struct ResamplingSpec {
    pub input_rate: u32,
    pub output_rate: u32,
    pub channels: usize,
}

impl ResamplingSpec {
    #[must_use]
    pub fn output_size(&self, input_size: usize) -> usize {
        (f64::from(self.output_rate) / f64::from(self.input_rate) * input_size as f64) as usize
    }

    #[must_use]
    pub fn input_size(&self, output_size: usize) -> usize {
        (f64::from(self.input_rate) / f64::from(self.output_rate) * output_size as f64) as usize
    }

    #[must_use]
    pub fn ratio(&self) -> f64 {
        f64::from(self.output_rate) / f64::from(self.input_rate)
    }
}

pub struct AudioResampler {
    pub spec: ResamplingSpec,
    pub resampler: SincFixedIn<f32>,
}

impl AudioResampler {
    /// # Errors
    pub fn new(spec: ResamplingSpec) -> Result<Self, ResamplerConstructionError> {
        let params = InterpolationParameters {
            sinc_len: 256,
            f_cutoff: 0.95,
            interpolation: InterpolationType::Linear,
            oversampling_factor: 256,
            window: WindowFunction::BlackmanHarris2,
        };
        let resampler = SincFixedIn::<f32>::new(48000_f64 / 44100_f64, 2.0, params, 1024, 2)?;

        Ok(Self { spec, resampler })
    }

    /// # Panics
    /// # Errors
    pub fn process(
        &mut self,
        input: &[Vec<f32>],
        output: &mut [Vec<f32>],
    ) -> Result<(usize, usize), ResampleError> {
        if self.spec.input_rate != self.spec.output_rate {
            return self
                .resampler
                .process_into_buffer(input, output, None)
                .map(|_| Ok((input.len(), output.len())))?;
        }

        // Bypass conversion completely in case the sample rates are equal.
        let out = &mut output.get_mut(..input.len()).unwrap();
        for i in 0..1 {
            let out_i = out.get_mut(i).unwrap();
            let in_i = input.get(i).unwrap();
            out_i.resize(in_i.len(), 0.0);
            out_i.copy_from_slice(in_i);
        }
        Ok((input.len(), output.len()))
    }
}
