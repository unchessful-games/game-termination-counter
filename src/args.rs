use clap::Parser;

/// Downloads and parses a Zstandard-compressed PGN file to produce cumulative statistics on game terminations.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// URL of pgn.zst file
    #[arg(short, long)]
    pub url: String,

    /// Name of JSON file to store stats
    #[arg(short, long)]
    pub file: String,
}
