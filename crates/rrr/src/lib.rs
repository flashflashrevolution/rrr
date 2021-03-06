#![warn(
    elided_lifetimes_in_paths,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    variant_size_differences,
    clippy::all,
    clippy::cargo,
    clippy::pedantic,
    clippy::as_conversions,
    clippy::clone_on_ref_ptr,
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::exit,
    clippy::expect_used,
    clippy::filetype_is_file,
    clippy::float_cmp_const,
    clippy::get_unwrap,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::integer_division,
    clippy::let_underscore_must_use,
    clippy::lossy_float_literal,
    clippy::mem_forget,
    clippy::multiple_inherent_impl,
    clippy::panic,
    clippy::pattern_type_mismatch,
    clippy::print_stdout,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::shadow_reuse,
    clippy::todo,
    clippy::unimplemented,
    clippy::unneeded_field_pattern,
    clippy::unreachable,
    clippy::unwrap_used,
    clippy::use_debug,
    clippy::verbose_file_reads,
    clippy::wrong_pub_self_convention
)]
#![allow(clippy::module_name_repetitions, clippy::multiple_crate_versions)]
#![forbid(unsafe_code)]

mod chart;
mod settings;

pub use chart::*;
pub use settings::*;

#[derive(Debug, Default)]
pub struct RRR {
    settings: Settings,
}

impl RRR {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_settings(settings: Settings) -> Self {
        Self { settings }
    }

    #[allow(clippy::unused_self)]
    pub fn start_chart(&self, _chart: &CompiledChart) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_settings() {
        let settings = Settings::new();
        let _rrr = RRR::with_settings(settings);
    }

    #[test]
    fn start_chart() {
        let rrr = RRR::new();
        let chart = Chart::default();
        let compiled_chart = chart.compile();
        rrr.start_chart(&compiled_chart);
    }
}
