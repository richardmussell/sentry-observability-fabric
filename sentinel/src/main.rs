use prometheus::{Encoder, IntGauge, Registry, TextEncoder};
use std::sync::{Arc, Mutex};
use sysinfo::{CpuExt, System, SystemExt};
use tracing::{info, Level};
use warp::Filter;

#[tokio::main]
async fn main() {
    // 1. Initialize Structured Observability
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    info!("--- 🛡️ SENTRY SENTINEL INITIALIZED ---");
    info!("Identity: Richard J. Mussell | Architect of the Zero-Lag Civilization");

    // 2. Register Metrics
    let r = Registry::new();
    let cpu_gauge =
        IntGauge::new("sentry_cpu_usage", "Current CPU utilization percentage").unwrap();
    let mem_gauge =
        IntGauge::new("sentry_memory_usage", "Current Memory utilization in MB").unwrap();
    r.register(Box::new(cpu_gauge.clone())).unwrap();
    r.register(Box::new(mem_gauge.clone())).unwrap();

    let registry = Arc::new(Mutex::new(r));
    let sys = Arc::new(Mutex::new(System::new_all()));

    // 3. Telemetry Loop (Background Task)
    let cpu_tracker = cpu_gauge.clone();
    let mem_tracker = mem_gauge.clone();
    let sys_clone = Arc::clone(&sys);

    tokio::spawn(async move {
        loop {
            {
                let mut sys = sys_clone.lock().unwrap();
                sys.refresh_all();

                // Calculate aggregate CPU usage
                let cpu_usage = sys.global_cpu_info().cpu_usage() as i64;
                let mem_usage = (sys.used_memory() / 1024 / 1024) as i64;

                cpu_tracker.set(cpu_usage);
                mem_tracker.set(mem_usage);
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
    });

    // 4. Expose Prometheus Metrics via HTTP
    let metrics_route = warp::path("metrics").map(move || {
        let mut buffer = Vec::new();
        let encoder = TextEncoder::new();
        let metric_families = registry.lock().unwrap().gather();
        encoder.encode(&metric_families, &mut buffer).unwrap();
        warp::reply::with_header(buffer, "Content-Type", encoder.format_type())
    });

    info!("Sentinel listening on 0.0.0.0:8080/metrics");
    warp::serve(metrics_route).run(([0, 0, 0, 0], 8080)).await;
}
