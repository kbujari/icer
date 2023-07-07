use clap::{ArgAction::*, Parser};
use anyhow::Result;
use std::path;

use libicer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

    #[arg(value_parser = clap::value_parser!(path::PathBuf))]
    /// Image to compress
    image_path: path::PathBuf,

    #[arg(short, long, default_value_t = String::from("q"))]
    /// Specify wavelet transform to be used
    transform: String,

    #[arg(short, long, action = SetTrue)]
    /// Enable verbose logging
    debug: bool,
}

fn main() -> Result<()> {
    let cli = Args::parse();
    libicer::def_img(&cli.image_path);

    let param = &cli.transform
        .parse::<libicer::params::FilterParams>()
        .expect("invalid filter")
        .to_params();

    println!("{:?}", param);

    Ok(())
}
