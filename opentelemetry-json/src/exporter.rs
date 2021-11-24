pub mod void {
    use opentelemetry::sdk::export::trace::{ExportResult, SpanData, SpanExporter};

    #[derive(Debug)]
    pub struct Exporter;

    #[async_trait::async_trait]
    impl SpanExporter for Exporter {
        async fn export(&mut self, _spans: Vec<SpanData>) -> ExportResult {
            Ok(())
        }
    }
}

pub mod json {
    use opentelemetry::sdk::export::trace::{ExportResult, SpanData, SpanExporter};
    use std::fmt::Debug;
    use std::io::Write;

    mod error {
        use opentelemetry::sdk::export::ExportError;

        /// Stdout exporter's error
        #[derive(thiserror::Error, Debug)]
        pub enum Error {
            #[error(transparent)]
            Io(#[from] std::io::Error),
            #[error(transparent)]
            Json(#[from] serde_json::Error),
        }

        impl ExportError for Error {
            fn exporter_name(&self) -> &'static str {
                "json_exporter"
            }
        }
    }
    #[derive(Debug)]
    pub struct Exporter<W: Write + Debug + Send> {
        writer: W,
    }

    impl<W: Write + Debug + Send> Exporter<W> {
        pub fn new(writer: W) -> Self {
            Self { writer }
        }
    }

    #[async_trait::async_trait]
    impl<W: Write + Debug + Send> SpanExporter for Exporter<W> {
        async fn export(&mut self, spans: Vec<SpanData>) -> ExportResult {
            for span in spans {
                let json_serialized = serde_json::to_string(&span).map_err::<error::Error, _>(Into::into)?;
                writeln!(&mut self.writer, "{}", json_serialized).map_err::<error::Error, _>(Into::into)?;
            }
            Ok(())
        }
    }
}
