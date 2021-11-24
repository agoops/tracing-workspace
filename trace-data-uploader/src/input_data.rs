use std::fs::File;
use std::path::Path;

pub use error::Error;
use opentelemetry::sdk::export::trace::SpanData;
use std::io::{BufRead, BufReader, Lines};

pub mod error {
    #[derive(thiserror::Error, Debug)]
    pub enum Error {
        #[error(transparent)]
        Open(std::io::Error),
    }
}
pub struct InputData {
    reader: Lines<BufReader<File>>,
}

impl InputData {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let file = File::open(path.as_ref()).map_err(Error::Open)?;
        let reader = BufReader::new(file).lines();
        Ok(Self { reader })
    }

    pub fn stream(self) -> futures::stream::Iter<Self> {
        futures::stream::iter(self)
    }
}

impl Iterator for InputData {
    type Item = SpanData;

    fn next(&mut self) -> Option<Self::Item> {
        self.reader.next().map(|string_result| {
            let string = string_result.expect("Could not parse next line as a String from input data file");
            let span_data: SpanData =
                serde_json::from_str(&string).expect("Could not parse json line as SpanData from input data file");
            span_data
        })
    }
}
