use off_chain_csv_processing::ApplicationComponentManager;
use off_chain_csv_processing::CsvStreamHandler;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut application_manager = ApplicationComponentManager::<CsvStreamHandler>::new().await?;
    application_manager.handle_stream().await;
    application_manager.print_output();
    Ok(())
}
