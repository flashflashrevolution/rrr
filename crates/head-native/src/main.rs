#![deny(rust_2018_idioms)]
#![warn(
    elided_lifetimes_in_paths,
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
    clippy::verbose_file_reads
)]
#![allow(clippy::module_name_repetitions, clippy::multiple_crate_versions)]
#![forbid(unsafe_code)]

use futures::executor;
use log::error;
use rrr_head::{
    platform::platform::time::Time,
    prelude::{anyhow::Error, futures, log, winit::event_loop::EventLoop},
    query::SettingsMerge,
};
use std::env;

const WIDTH: u32 = 512;
const HEIGHT: u32 = 720;

fn main() {
    if simple_logger::init().is_err() {
        error!("Could not initialize simple_logger, quitting.");
        return;
    }

    match executor::block_on(run()) {
        Ok(_) => {}
        Err(err) => {
            error!(
                "{:?} exited with bad form: {}",
                env::current_exe().ok(),
                err
            );
        }
    }
}

async fn run() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let window = rrr_head::build_window(&event_loop, WIDTH, HEIGHT)?;
    let extracted_settings: Option<SettingsMerge> = { None };

    let mut game = rrr_head::Game::<Time>::new(None, WIDTH, HEIGHT);
    game.with_settings(extracted_settings);

    rrr_head::run_game_loop(window, event_loop, game).await
}
