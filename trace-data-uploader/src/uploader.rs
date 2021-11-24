pub use jaeger::error::Error as JaegerError;
pub use jaeger::Jaeger;

mod jaeger {
    use futures::StreamExt;
    use opentelemetry::sdk::export::trace::{SpanData, SpanExporter};

    pub mod error {
        use opentelemetry::trace::TraceError;

        #[derive(thiserror::Error, Debug)]
        pub enum Error {
            #[error(transparent)]
            BuildExporter(#[from] TraceError),
            #[error(transparent)]
            Export(TraceError),
        }
    }

    pub struct Jaeger {
        exporter: opentelemetry_jaeger::Exporter,
    }

    impl Jaeger {
        pub fn new() -> Result<Self, error::Error> {
            let builder = opentelemetry_jaeger::PipelineBuilder::default();
            let exporter = builder.init_sync_exporter()?;
            Ok(Self { exporter })
        }

        pub async fn upload<Stream: futures::stream::Stream<Item = SpanData> + std::marker::Unpin + Send>(
            mut self,
            mut spans: Stream,
        ) -> Result<(), error::Error> {
            while let Some(s) = spans.next().await {
                self.exporter.export(vec![s]).await.map_err(error::Error::Export)?;
            }
            Ok(())
        }
    }
}
