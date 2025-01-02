use std::path::PathBuf;

use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};

#[derive(Parser)]
struct Arguments {
    #[command(flatten)]
    pub verbose: Verbosity<InfoLevel>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Build {
        /// Path to the file Tungsten should compile
        file_name: PathBuf,

        /// Optimization level used in codegen
        #[arg(short = 'O', default_value_t = 0)]
        opt_level: u8,

        /// Path to emit build artifacts
        #[arg(long = "out-dir", default_value = "target")]
        out_dir: PathBuf,
    },
}

pub fn get_command() -> Command {
    let args = Arguments::parse();

    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    args.command
}
