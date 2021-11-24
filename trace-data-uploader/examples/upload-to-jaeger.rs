//! --------------------- Jaeger UI----------------
//! Start a local Jaeger visualizer with:
//! ```
//! docker run -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 jaegertracing/all-in-one:latest
//! ```
//! Visit `http://localhost:16686/` for the UI
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // CLI input
    let cwd = std::env::current_dir().unwrap();
    let trace_data_file_path = cwd.join("sample-trace-data.jsonl");

    let cli_args = trace_data_uploader::cli::Args {
        input: trace_data_file_path,
    };

    // Call CLI
    trace_data_uploader::cli::run(cli_args).await.unwrap();
}
