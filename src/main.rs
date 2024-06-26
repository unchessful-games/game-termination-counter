use clap::Parser;
use stats::{ByOpening, GameCounter};

mod args;
mod generic_stats;
mod stats;
mod url_reader;
mod visitor;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    let args = args::Args::parse();
    let file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(args.file)
        .unwrap();
    let file = std::io::BufWriter::with_capacity(128 * 1024, file);
    let stats: ByOpening<GameCounter> = url_reader::download_url(args.url).await?;
    serde_json::to_writer(file, &stats)?;

    Ok(())
}
