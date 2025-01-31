use colored::Colorize;
use hdrhistogram::Histogram;
use std::fmt::{Display, Formatter};
use tokio::time::Instant;



pub trait Metrics {
    fn ino_avg(&self) -> u64;
    fn ino_max(&self) -> u64;
    fn ino_min(&self) -> u64;
}

#[derive(Debug)]
pub struct BenchmarkResult {
    pub status: String,
    pub duration: u64,
    pub execution: usize,
    pub num_client: usize,
}


#[derive(Debug)]
pub struct Report {
    clients: usize,
    pub results: Vec<BenchmarkResult>,
    hist: Histogram<u64>,
    start: Instant,
}

impl Metrics for Vec<BenchmarkResult> {

    /**
    *=================================================================
    * ino_avg()
    *=================================================================
    *
    *
    *
    *
    *
    *
    *
    *=================================================================
    *
    *
    *
    */
    fn ino_avg(&self) -> u64 {
        let total: u64 = self.iter().map(|r| r.duration).sum();
        let size: u64 = self.iter().len() as u64;
        total / size
    }

    /**
    *=================================================================
    * ino_max()
    *=================================================================
    *
    *
    *
    *
    *
    *
    *
    *=================================================================
    *
    *
    *
    */
    fn ino_max(&self) -> u64 {
        return self.iter().map(|r| r.duration).max().unwrap_or(0)
    }

    /**
    *=================================================================
    * ino_min()
    *=================================================================
    *
    *
    *
    *
    *
    *
    *
    *=================================================================
    *
    *
    *
    */
    fn ino_min(&self) -> u64 {
        return self.iter().map(|r| r.duration).min().unwrap_or(0)
    }
}



impl Display for BenchmarkResult {

    /**
    *=================================================================
    * Formatter()
    *=================================================================
    *
    *
    *
    *
    *
    *
    *
    *=================================================================
    *
    *
    *
    */
    fn fmt(&self, f: &mut Formatter<'_> ) -> std::fmt::Result {
        let report = format!("[{} {} {} {}] {} {}{}", "Client".bold().green(), self.num_client.to_string().bold().green(), "Iteration".bold().green(),
            self.execution.to_string().bold().green(),
            self.status.to_string().bold().yellow(),
            self.duration.to_string().cyan(),
            "ms".cyan()
        );
        write!(f, "{}", report)
    }
}



impl Report {

    /**
    *=================================================================
    * new()
    *=================================================================
    *
    *
    *
    *
    *
    *
    *
    *=================================================================
    *
    *
    *
    */
    pub fn new(clients: usize) -> Self {
        Report {
            clients,
            results: vec![],
            hist: Histogram::<u64>::new(5).unwrap(),
            start: Instant::now()
        }
    }


    /**
    *=================================================================
    * ino_add_result()
    *=================================================================
    *
    *
    *
    *
    *
    *
    *
    *=================================================================
    *
    *
    *
    */
    pub fn ino_add_result(&mut self, result: BenchmarkResult) {
        let duration = result.duration;
        self.results.push(result);
        self.hist.record(duration).expect("");
    }


    /**
    *=================================================================
    * ino_show_result()
    *=================================================================
    *
    *
    *
    *
    *
    *
    *
    *=================================================================
    *
    *
    *
    */
    pub fn ino_show_result(&self) {
        let elapsed = &self.start.elapsed();
        println!();
        println!();
        println!();

        println!("{} {}", "Concurrency level".yellow().bold(), self.clients.to_string().purple());
        println!("{} {} {}", "Time taken".yellow().bold(), elapsed.as_secs().to_string().purple(), "seconds".purple());
        println!("{} {}", "Total requests ".yellow().bold(), self.hist.len().to_string().purple());
        println!("{} {} {}", "Mean request time".yellow().bold(), self.hist.mean().to_string().purple(), "ms".purple());
        println!("{} {} {}", "Max request time".yellow().bold(), self.results.ino_max().to_string().purple(), "ms".purple());
        println!("{} {} {}", "Min request time".yellow().bold(), self.results.ino_min().to_string().purple(), "ms".purple());
        println!("{} {} {}", "95'th percentile:".yellow().bold(), self.hist.value_at_quantile(0.95).to_string().purple(), "ms".purple());
        println!("{} {} {}", "99.9'th percentile:".yellow().bold(), self.hist.value_at_quantile(0.999).to_string().purple(), "ms".purple());
    }
}