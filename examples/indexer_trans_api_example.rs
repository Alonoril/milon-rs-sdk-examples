use milon_client::indexer::{
    self as indexer, B256, IndexerTransactionExt, Provider, TransactionQuery,
};
use only_sdk_examples::{connect_indexer, init};
use std::{env, error::Error};

// const DEFAULT_INDEXER_URL: &str = "http://127.0.0.1:6088";
const DEFAULT_INDEXER_URL: &str = "http://47.84.39.153:6088";
const MILON_TX_HASH_BS58: &str = "EGxFuywXGokSLV1dhXKCqiAnDt1EJNwNEDJCmNXQcgLv";

/// cargo run --manifest-path only-sdk-examples/Cargo.toml \
///     --example indexer_trans_api_example
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _guard = init()?;
    let indexer_url =
        env::var("MILON_INDEXER_URL").unwrap_or_else(|_| DEFAULT_INDEXER_URL.to_owned());
    tracing::info!(indexer_url = %indexer_url, "connecting to indexer");
    let provider = connect_indexer(&indexer_url)?;

    transactions_overview(&provider).await?;
    // transactions(&provider).await?;
    transaction(&provider, B256::from_bs58(MILON_TX_HASH_BS58)?).await?;

    Ok(())
}

async fn transactions_overview<P: Provider>(provider: &P) -> Result<(), Box<dyn Error>> {
    let overview = provider.transactions_overview().await?;
    tracing::info!(?overview, "transactions overview");
    Ok(())
}

async fn transactions<P: Provider>(provider: &P) -> Result<(), Box<dyn Error>> {
    let page = provider
        .transactions(TransactionQuery {
            page_size: 10,
            ..TransactionQuery::default()
        })
        .await?;

    tracing::info!(?page, "transactions page");
    tracing::info!(
        count = page.items.len(),
        has_more = page.has_more,
        "transactions page summary"
    );
    if let Some(cursor) = page.next_cursor.as_deref() {
        tracing::info!(next_cursor = %cursor, "transactions next cursor");
    }
    Ok(())
}

async fn transaction<P>(provider: &P, hash: B256) -> Result<(), Box<dyn Error>>
where
    P: Provider,
{
    let detail = provider.transaction(hash).await?;
    tracing::info!(?detail, "transaction detail");
    Ok(())
}
