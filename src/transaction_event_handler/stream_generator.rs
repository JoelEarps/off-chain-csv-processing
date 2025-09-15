/// Why did I create this trait?
use tokio_stream::Stream;

#[async_trait::async_trait]
pub trait GenerateStream {
    type StreamItem;
    type Stream: Stream<Item = anyhow::Result<Self::StreamItem>> + Send + Unpin;
    async fn generate_stream_from_source() -> anyhow::Result<Self::Stream>;
}