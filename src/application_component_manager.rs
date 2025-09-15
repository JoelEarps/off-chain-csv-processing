use crate::{transaction_event_handler::{stream_generator::GenerateStream, transaction_event_handler::TxEvent}, user_accounts_cache::errors::ErrorReportAnyhow, CacheHandler};
use futures::StreamExt;

pub struct ApplicationComponentManager<A: GenerateStream> {
    user_accounts_task_handler: CacheHandler,
    stream_task_handler: A::Stream,
    client_tx_error_report: ErrorReportAnyhow,
    stream_error_handler: ErrorReportAnyhow
}

impl<A: GenerateStream> ApplicationComponentManager<A> where
A: GenerateStream<StreamItem = TxEvent> 
{
    pub async fn new() -> anyhow::Result<Self> {
        Ok(Self {
            user_accounts_task_handler: CacheHandler::new(),
            stream_task_handler: A::generate_stream_from_source().await?,
            client_tx_error_report: ErrorReportAnyhow::new(),
            stream_error_handler: ErrorReportAnyhow::new(),
        })
    }

    /// Function to handle record, any failures are added to an error log collection which would enable error dumps when in debug mode
    /// All account data is handled in the cache via the interior mutability pattern
    pub async fn handle_stream(&mut self) {
        while let Some(record) = self.stream_task_handler.next().await {
        match record {
            Ok(transaction_event) => {
                match self.user_accounts_task_handler.handle_account_update(&transaction_event) {
                    Ok(_) => {
                        // println!("Transaction handled successfully");
                    }
                    Err(error) => {
                        self.client_tx_error_report.push(error);
                    }
                }
            }
            Err(error) => {
                self.stream_error_handler.push(error);
            }
        }
    }
    }

    /// Simple print for output, enabled by the implementation of display
    pub fn print_output(&self) {
        println!("{}", self.user_accounts_task_handler);
    }
}