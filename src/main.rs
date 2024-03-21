use clap::Parser;
use tokio::io::AsyncWriteExt;

mod args;
mod stats;
mod url_reader;
mod visitor;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    let args = args::Args::parse();
    let file = tokio::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(args.file)
        .await
        .unwrap();
    let mut file = tokio::io::BufWriter::with_capacity(128 * 1024, file);
    let stats = url_reader::download_url(args.url).await?;
    let result = serde_json::to_vec(&stats)?;
    file.write_all(&result).await?;

    Ok(())
}
