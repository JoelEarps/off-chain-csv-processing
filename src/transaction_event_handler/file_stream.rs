use std::{env};

use csv_async::{AsyncReaderBuilder};
use futures::{stream::BoxStream, StreamExt, TryStreamExt};
use tokio::fs::File;

use crate::transaction_event_handler::{stream_generator::GenerateStream, transaction_event_handler::TxEvent};

#[derive(Default)]
pub struct CsvStreamHandler;

/// Trait Implementation for create a stream for the CSV streamer
#[async_trait::async_trait]
impl GenerateStream for CsvStreamHandler {
    type StreamItem = TxEvent;
    type Stream = BoxStream<'static, anyhow::Result<Self::StreamItem>>;

    async fn generate_stream_from_source() -> anyhow::Result<Self::Stream> {
        let args: Vec<String> = env::args().collect();
        let file_path = args.get(1).expect("please provide CSV file path");
        let file = File::open(file_path).await?;
        let rdr = AsyncReaderBuilder::new()
            .trim(csv_async::Trim::All)
            .create_deserializer(file);
        Ok(rdr.into_deserialize::<TxEvent>().map_err(|e| anyhow::anyhow!(e)).boxed())
    }
}