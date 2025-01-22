mod benchmark;
mod execution;
mod support;

use anyhow::Result;
use clap::Parser;

use crate::benchmark::Report;
use crate::execution::ino_run;
use crate::support::{Args, Settings};
use indicatif::ProgressBar;
use tokio::sync::{mpsc, watch};

#[tokio::main]
async fn main() -> Result<()> {
    let settings: Settings = Args::parse().ino_to_string()?;
    let mut report = Report::new(settings.clients);
    settings.ino_print_banner();
    let pb = ProgressBar::new(settings.requests as u64);
    let (tx_sigint, rx_sigint) = watch::channel(None);
    let (benchmark_tx, mut benchmark_rx) = mpsc::channel(settings.requests);

    ctrlc::set_handler(move || {
        tx_sigint.send(Some(())).unwrap_or(());
    })?;
    ino_run(settings.clone(), benchmark_tx, rx_sigint).await?;
    while let Some(value) = benchmark_rx.recv().await {
        match settings.verbose {
            true => println!("{}", value),
            false => pb.inc(1),
        }
        report.ino_add_result(value);
    }
    report.ino_show_result();
    Ok(())
}