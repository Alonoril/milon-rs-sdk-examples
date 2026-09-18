use infra_tracing::tests::setup_logger;
use milon_client::{self as sdk, primitives::TxHash};
use milon_provider::Provider;
use only_sdk_examples::{DemoRpc, decode_print::print_transaction_history};
use std::{env, error::Error};
use tracing::{Level, info};

// const DEFAULT_HTTP_RPC_URL: &str = "http://127.0.0.1:6280/v1/rpc";
// const DEFAULT_HTTP_RPC_URL: &str = "http://8.218.101.239:6280/v1/rpc";
const DEFAULT_HTTP_RPC_URL: &str = "http://47.238.69.82:6280/v1/rpc";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ss = setup_logger(Level::INFO)?;
    let rpc_url = env::var("MILON_RPC_URL").unwrap_or_else(|_| DEFAULT_HTTP_RPC_URL.to_owned());
    let rpc = DemoRpc::connect(&rpc_url)?;

    let chain_state = rpc.provider.get_chain_head().await?;
    info!("chain_head: {chain_state}");

    let res = rpc.provider.get_block_by_height(chain_state.block_height).await?;
    // let res = rpc.provider.get_block_by_height(56).await?;
    // info!("get_block_by_height: {res}");
    // let res = rpc.provider.get_block_by_height(57).await?;
    // info!("get_block_by_height: {res}");
    // let res = rpc.provider.get_block_by_height(58).await?;
    // info!("get_block_by_height: {res}");
    // let res = rpc.provider.get_block_by_height(59).await?;
    info!("get_block_by_height: {res}");

    Ok(())
}
