use std::str::FromStr;

use anyhow::{Context, Result};
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use tokio::sync::mpsc::Sender;
use tokio::sync::watch::Receiver;
use tokio::time::Instant;

use crate::benchmark::BenchmarkResult;
use crate::support::{Operation, Settings};
use crate::support::Operation::Head;

/**
 *=================================================================
 * ino_run()
 *=================================================================
 *
 * Asynchronously starts the benchmarking process by creating HTTP
 * clients and spawning tasks to execute requests. The function is
 * responsible for orchestrating the execution.
 *
 *=================================================================
 */
pub async fn ino_run(settings: Settings, tx: Sender<BenchmarkResult>, rx_sigint: Receiver<Option<()>>) -> Result<()> {
    let mut clients = Vec::with_capacity(settings.clients);
    for _ in 0..settings.clients {
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .tcp_keepalive(settings.keep_alive)
            .build()
            .with_context(|| "Can not create http Client".to_string())?;
        clients.push(client);
    }
    for (id, client) in clients.into_iter().enumerate() {
        tokio::spawn(ino_exec_iterator(
            id,
            settings.clone(),
            client,
            tx.clone(),
            rx_sigint.clone(),
        ));
    }
    Ok(())
}

/**
 *=================================================================
 * ino_exec_iterator()
 *=================================================================
 * This function determines the appropriate execution method based
 * on the provided settings. It supports two modes:
 * - Fixed iterations: Executes a specific number of requests.
 * - Timed execution: Runs for a given duration.
 *
 *
 *
 *=================================================================
 *
 *
 *
 */
async fn ino_exec_iterator(num_client: usize, settings: Settings, client: Client, tx: Sender<BenchmarkResult>, mut rx_sigint: Receiver<Option<()>>) {
    match settings.duration {
        None => {
            ino_by_iterations(num_client, &settings, &client, &tx, &mut rx_sigint).await;
        }
        Some(duration) => {
            ino_by_time(num_client, &settings, &client, tx, &mut rx_sigint, duration).await;
        }
    }
}

/**
 *=================================================================
 * ino_by_time()
 *=================================================================
 *
 *  This function runs benchmarking requests for a specified duration.
 * It repeatedly sends requests until the time limit is reached or
 * a stop signal is received. Results are sent to the provided channel.
 *
 *
 *=================================================================
 *
 *
 */
async fn ino_by_time(num_client: usize, settings: &Settings, client: &Client, tx: Sender<BenchmarkResult>, rx_sigint: &mut Receiver<Option<()>>, duration: u64) {
    let begin = Instant::now();
    let mut execution_number = 0;
    while begin.elapsed().as_secs() < duration {
        let stop_signal = rx_sigint.changed();
        let benchmark_result = ino_exec(num_client, execution_number, client, settings);
        let ack_send_result = tx.send(benchmark_result.await);
        execution_number += 1;
        match tokio::select! {
        _ = ack_send_result =>  None,
        _ = stop_signal => Some(())
        } {
            None => {}
            Some(_) => break,
        }
    }
}

/**
 *=================================================================
 * ino_by_iterations()
 *=================================================================
 *
 * This function executes a predefined number of benchmarking
 * requests
 * for a specific client. The number of iterations
 * is determined by
 * the settings. The function listens for a stop
 * signal (SIGINT) to
 * gracefully terminate execution if required.
 *
 *
 *=================================================================
 *
 *
 *
 */
async fn ino_by_iterations(num_client: usize, settings: &Settings, client: &Client, tx: &Sender<BenchmarkResult>, rx_sigint: &mut Receiver<Option<()>>) {
    for execution_number in 0..settings.ino_requests_by_client() {
        let stop_signal = rx_sigint.changed();
        let benchmark_result = ino_exec(num_client, execution_number, client, settings);
        let ack_send_result = tx.send(benchmark_result.await);

        match tokio::select! {
        _ = ack_send_result =>  None,
        _ = stop_signal => Some(())
        } {
            None => {}
            Some(_) => break,
        }
    }
}

/**
 *=================================================================
 * ino_exec()
 *=================================================================
 *
 * Executes a single HTTP request using the specified client and
 * benchmarking settings. Configures the HTTP method, headers, and
 * body as needed.
 *
 *
 *=================================================================
 *
 *
 */
async fn ino_exec(num_client: usize, execution: usize, client: &Client, settings: &Settings) -> BenchmarkResult {
    let request_builder = match settings.ino_operation() {
        Operation::Get => client.get(settings.ino_target()),
        Operation::Post => client.post(settings.ino_target()),
        Operation::Head => client.head(settings.ino_target()),
        Operation::Patch => client.patch(settings.ino_target()),
        Operation::Put => client.put(settings.ino_target()),
        Operation::Delete => client.delete(settings.ino_target()),
    };
    let headers_map: HeaderMap = match &settings.headers {
        None => HeaderMap::new(),
        Some(headers) => {
            let mut headers_map: HeaderMap = HeaderMap::new();
            headers.iter().for_each(|h| {
                let name = h.key.as_str();
                let value = h.value.as_str();

                let name = HeaderName::from_str(name).unwrap();
                let value = HeaderValue::from_str(value).unwrap();
                headers_map.insert(name, value);
            });
            headers_map
        }
    };
    let request_builder = match &settings.body {
        None => request_builder,
        Some(body) => request_builder.body(body.to_string()),
    };
    let request = request_builder.headers(headers_map);
    let begin = Instant::now();
    let response = request.send().await;
    let duration_ms = begin.elapsed().as_millis() as u64;
    match response {
        Ok(r) => BenchmarkResult {
            status: r.status().to_string(),
            duration: duration_ms,
            num_client,
            execution,
        },
        Err(e) => {
            let status = match e.status() {
                None => {
                    "Failed to connect".to_string()
                }
                Some(s) => s.to_string(),
            };
            BenchmarkResult {
                status,
                duration: duration_ms,
                num_client,
                execution,
            }
        }
    }
}