# Tracing Workspace

This repo is currently acting as a playground for developing libraries and tools to 
interact with the [tracing][tracing] 
and [opentelemetry-rust][opentelemetry-rust] ecosystems.

## Crates

- **[opentelemetry-json](./opentelemetry-json)** - this crate offers a way to configure one's tracing exporter
to json-serialize [`SpanData`][SpanData] to a given `Write` destination.
- **[trace-data-uploader](./trace-data-uploader)** - Given a file(s) where each line is a json-serialized [`SpanData`][SpanData], this CLI uploads the span data to a backend for visualization (e.g. [Jaeger UI][jaeger]) 
- **[managed-tracing](./managed-tracing)** - WIP. Does nothing yet.

## Example

Run this [example](./opentelemetry-json/examples/export-to-jsonl.rs) to generate trace data and save to file with: 
```bash
cargo run --example export-to-jsonl
```

Start a local jager instance with:
```bash
docker run -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 jaegertracing/all-in-one:latest
```

And then run this [example](./trace-data-uploader/examples/upload-to-jaeger.rs) upload your trace data file with:
```bash
cargo run --example upload-to-jaeger
```


[SpanData]: https://docs.rs/opentelemetry/0.16.0/opentelemetry/sdk/export/trace/struct.SpanData.html
[tracing]: https://docs.rs/tracing/0.1.29/tracing/
[opentelemetry-rust]: https://docs.rs/opentelemetry/0.16.0/opentelemetry/
[jaeger]: https://github.com/jaegertracing/jaeger-ui

