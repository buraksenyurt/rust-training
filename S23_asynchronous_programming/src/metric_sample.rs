use rand::Rng;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task;
use tokio::time::sleep;

pub async fn run() {
    let (log_transmitter, mut log_receiver) = mpsc::channel(100);

    let cpu_task = task::spawn(fetch_metrics("CPU Service", log_transmitter.clone()));
    let memory_task = task::spawn(fetch_metrics("Memory Service", log_transmitter.clone()));
    let disk_task = task::spawn(fetch_metrics("Disk Service", log_transmitter));

    let logger_task = task::spawn(async move {
        while let Some(metric) = log_receiver.recv().await {
            println!("LOG: {}", metric);
        }
    });

    let _ = tokio::join!(cpu_task, memory_task, disk_task, logger_task);
}

async fn fetch_metrics(service_name: &str, tx: mpsc::Sender<String>) {
    let interval = Duration::from_secs(5);
    for i in 1..=10 {
        let metric = format!("{} - Metric {}: {}", service_name, i, get_metric());
        if tx.send(metric).await.is_err() {
            println!("{}: Channel isn't active!", service_name);
            break;
        }
        sleep(interval).await;
    }
}

fn get_metric() -> f64 {
    let mut rng = rand::rng();
    rng.random_range(50.0..100.0)
}
