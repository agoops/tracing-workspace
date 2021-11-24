use crate::exporter::void::Exporter;
use opentelemetry::trace::TracerProvider;
use opentelemetry::{sdk, sdk::export::trace::SpanExporter};

/// Pipeline builder
#[derive(Debug)]
pub struct Builder<SE: SpanExporter + 'static> {
    trace_config: Option<sdk::trace::Config>,
    exporter: SE,
}

impl Default for Builder<Exporter> {
    /// Return the default pipeline builder.
    fn default() -> Self {
        Self {
            trace_config: None,
            exporter: Exporter,
        }
    }
}

impl<SE: SpanExporter> Builder<SE> {
    pub fn with_exporter<SE2: SpanExporter>(self, exporter: SE2) -> Builder<SE2> {
        Builder {
            trace_config: self.trace_config,
            exporter,
        }
    }

    /// Install the exporter pipeline with the recommended defaults.
    /// Copied with mods from [here](https://github.com/open-telemetry/opentelemetry-rust/blob/b3fa55361d233451c1e4f2c9c39f6557e83c47ec/opentelemetry/src/sdk/export/trace/stdout.rs#L94)
    pub fn install_simple(mut self) -> sdk::trace::Tracer {
        let exporter = self.exporter;

        let mut provider_builder = sdk::trace::TracerProvider::builder().with_simple_exporter(exporter);
        if let Some(config) = self.trace_config.take() {
            provider_builder = provider_builder.with_config(config);
        }
        let provider = provider_builder.build();
        // Passing empty string and None will use sdk defaults.
        // See: https://github.com/open-telemetry/opentelemetry-rust/blob/b3fa55361d233451c1e4f2c9c39f6557e83c47ec/opentelemetry/src/sdk/trace/provider.rs#L81
        let tracer = provider.tracer("", None);
        opentelemetry::global::set_tracer_provider(provider);

        tracer
    }
}
