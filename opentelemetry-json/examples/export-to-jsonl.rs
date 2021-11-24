use std::fs::File;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

fn main() {
    // Set up target file
    let cwd = std::env::current_dir().unwrap();
    let target_file = cwd.join("sample-trace-data.jsonl");
    let my_file = File::create(target_file).unwrap();
    println!("Writing trace data to: {:?}", my_file);

    // Build json_exporter with writer
    let json_exporter = opentelemetry_json::exporter::json::Exporter::new(my_file);

    let tracer = opentelemetry_json::pipeline::Builder::default()
        .with_exporter(json_exporter)
        .install_simple();

    let json_file_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::Registry::default()
        .with(json_file_layer)
        .try_init()
        .expect("Failed initializing global subscriber");

    trace_me();

    // Ensures all traces finish getting exported
    opentelemetry::global::shutdown_tracer_provider();
}

#[tracing::instrument]
fn trace_me() {
    for i in 0..2 {
        do_work(i);
    }
}

#[tracing::instrument]
fn do_work(arg: usize) {
    let res = arg * 10;
    tracing::info!(iteration_times_10 = res)
}
