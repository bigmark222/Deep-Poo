use clap::Parser;

use colon_sim::cli::{AppArgs, RunMode};
use colon_sim::run_app;

fn main() {
    let mut args = AppArgs::parse();
    args.mode = RunMode::Datagen;
    // Default to headless, with a tiny frame cap for fast smoke runs if unset.
    args.headless = true;
    args.max_frames = args.max_frames.or(Some(5));
    run_app(args);
}
