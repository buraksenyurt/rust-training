use std::time::Duration;
use tokio::task;
use tokio::time::sleep;

pub async fn run() {
    let i_robot = task::spawn(fetch_sensor_data(12456, 1));
    let ai = task::spawn(fetch_sensor_data(11101, 3));
    let optimus_prime = task::spawn(fetch_sensor_data(19023, 2));

    let _ = tokio::join!(i_robot, ai, optimus_prime);
}

async fn fetch_sensor_data(sensor_id: u32, interval: u64) {
    for i in 1..=10 {
        println!(
            "Sensor ID: {}. Fetching data...{}. Result is {}",
            sensor_id,
            i,
            simulate_sensor_reading()
        );
        sleep(Duration::from_secs(interval)).await;
    }
}

fn simulate_sensor_reading() -> f64 {
    rand::random::<f64>() * 10.0
}
