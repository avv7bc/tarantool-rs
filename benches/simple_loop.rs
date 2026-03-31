use std::time::{Duration, Instant};

use futures::{stream::repeat_with, StreamExt};
use tarantool_rs::{Connection, ExecutorExt};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let container = tarantool_test_container::TarantoolTestContainer::default_container().await;

    let conn = Connection::builder()
        .internal_simultaneous_requests_threshold(1000)
        .build(format!("127.0.0.1:{}", container.connect_port()))
        .await?;

    let mut counter = 0u64;
    let mut last_measured_counter = 0;
    let mut last_measured_ts = Instant::now();

    let interval_secs = 2;
    let interval = Duration::from_secs(interval_secs);

    let mut stream = repeat_with(|| conn.ping()).buffer_unordered(1000);
    while stream.next().await.is_some() {
        counter += 1;
        if last_measured_ts.elapsed() > interval {
            last_measured_ts = Instant::now();
            let counter_diff = counter - last_measured_counter;
            last_measured_counter = counter;
            println!(
                "Iterations over last {interval_secs} seconds: {counter_diff}, per second: {}",
                counter_diff / interval_secs
            );
        }
    }

    Ok(())
}
