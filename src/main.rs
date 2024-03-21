use clap::Parser;

mod args;
mod stats;
mod url_reader;
mod visitor;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    let args = args::Args::parse();
    println!("{args:?}");
    url_reader::download_url(args.url).await;

    Ok(())
}
