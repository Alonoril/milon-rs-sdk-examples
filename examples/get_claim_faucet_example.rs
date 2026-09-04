use infra_tracing::tests::setup_logger;
use milon_client::{self as sdk, primitives::TxHash, TokenProviderExt, WalletFiller};
use milon_provider::Provider;
use only_sdk_examples::{DemoRpc, decode_print::print_transaction_history, local_ed25519_signer};
use std::{env, error::Error};
use tracing::{Level, info};
use milon_local_wallet::LocalWallet;

// const DEFAULT_HTTP_RPC_URL: &str = "http://127.0.0.1:6280/milon/v1";
const DEFAULT_HTTP_RPC_URL: &str = "http://8.218.101.239:6280/milon/v1";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ss = setup_logger(Level::INFO)?;
    let rpc_url = env::var("MILON_RPC_URL").unwrap_or_else(|_| DEFAULT_HTTP_RPC_URL.to_owned());
    let rpc = DemoRpc::connect(&rpc_url)?;

    let signer = local_ed25519_signer(202).expect("Failed to create signer");
    let wallet = LocalWallet::new(signer);

    let provider = &rpc.provider;
    let provider = provider.with_wallet_filler(WalletFiller::new(wallet));

    let res = provider.claim_faucet_with_cooldown_remaining().await?;
    info!("claim_faucet returned: {:?}", res);
    Ok(())
}
