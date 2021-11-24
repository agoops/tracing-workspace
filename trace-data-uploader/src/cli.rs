use crate::input_data::InputData;
use crate::uploader;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Args {
    /// Input file
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,
}

pub mod error {
    use crate::{input_data, uploader};

    #[derive(thiserror::Error, Debug)]
    pub enum Error {
        #[error(transparent)]
        Input(#[from] input_data::Error),
        #[error(transparent)]
        Upload(#[from] uploader::JaegerError),
    }
}

pub async fn run(args: Args) -> Result<(), error::Error> {
    let input_data: InputData = InputData::from_file(args.input)?;
    let uploader = uploader::Jaeger::new()?;
    uploader.upload(input_data.stream()).await?;
    Ok(())
}
