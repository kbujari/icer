use anyhow::Result;
use clap::{ArgAction::*, Parser};
use std::path;

use libicer::Icer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Image to compress
    #[arg(value_parser = clap::value_parser!(path::PathBuf))]
    image_path: path::PathBuf,

    /// Specify wavelet transform to be used
    #[arg(short, long)]
    transform: libicer::FilterParams,

    /// Enable verbose logging
    #[arg(short, long, action = SetTrue)]
    debug: bool,
}

fn main() -> Result<()> {
    let cli = Args::parse();

    let icer = Icer {
        filter_param: cli.transform,
    };

    println!("{:?}", icer.compress(&cli.image_path));

    Ok(())
}
