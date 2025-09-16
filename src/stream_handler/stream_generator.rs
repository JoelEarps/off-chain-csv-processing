use tokio_stream::Stream;

/// A generic async trait that has two associated types, StreamItem and Stream
/// This way any function that generates a stream of the associated StreamItem can be used in the system.
#[async_trait::async_trait]
pub trait GenerateStream {
    type StreamItem;
    type Stream: Stream<Item = anyhow::Result<Self::StreamItem>> + Send + Unpin;
    async fn generate_stream_from_source() -> anyhow::Result<Self::Stream>;
}
