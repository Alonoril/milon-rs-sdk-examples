use milon_client::{self as sdk, demo, token};
use milon_crypto::{Address, secretkey::SecretKey};
use milon_idl_core::{Method, Signer};
use milon_local_wallet::LocalWallet;
use milon_primitives::{ChainId, TxHash};
use milon_provider::{Provider, ProviderError};
use only_sdk_examples::{
    DemoRpc, claim_faucet,
    decode_print::{DecodedReceipt, print_decoded_instructions, print_simulate_receipt},
};
use serde::{Deserialize, Serialize};
use std::{
    env,
    error::Error,
    fs,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

const DEFAULT_HTTP_RPC_URL: &str = "http://127.0.0.1:6380/milon/v1";
// const DEFAULT_HTTP_RPC_URL: &str = "http://47.84.39.153:6280/milon/v1";

const DEFAULT_CHAIN_ID: u64 = 900_000_001;
const TX_STAMP_LEAD_MS: u64 = 30_000;

const CONFIRM_RETRY_ATTEMPTS: usize = 10;
const CONFIRM_RETRY_DELAY: Duration = Duration::from_millis(500);
/*

*/

//
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let input = load_json_input()?;
    let demo_rpc = DemoRpc::connect(&input.rpc_url)?;

    let request =
        build_batch_credit_request(input.chain_id, input.stamp.unwrap_or_else(next_stamp))?;

    print_decoded_instructions(request.instructions());

    let simulated_bytes = demo_rpc
        .provider
        .simulate_transaction(request.clone())
        .await?;

    let receipt = decode_simulate_receipt(&simulated_bytes)?;
    print_simulate_receipt(&receipt);

    // submit tx
    let tx_hash = demo_rpc.provider.submit_transaction(request).await?;
    println!("tx_hash: {tx_hash}");
    wait_tx_hash(&demo_rpc, tx_hash).await?;

    Ok(())
}

fn decode_simulate_receipt(
    bytes: &[u8],
) -> Result<DecodedReceipt, sdk::DecodeError<sdk::idl_core::Error>> {
    sdk::decode_transaction_response(bytes)
}

fn build_batch_credit_request(
    chain_id: ChainId,
    stamp: u64,
) -> Result<milon_primitives::Transaction, Box<dyn Error>> {
    let demo_secret = SecretKey::new_pure();
    let demo_pool = Address::from_public_key(&demo_secret.ed25519_public());
    let demo_recipient = Address::from_bytes(&[11_u8; 20])?;
    let init_pool = demo::InitPool {
        pool: Signer::new(demo_pool),
        label: "simulate batch credit".to_owned(),
    };
    let batch_credit = demo::BatchCredit {
        pool: demo_pool,
        recipients: vec![demo_recipient],
        amount: 42,
    };
    let instructions = vec![
        claim_faucet(demo_pool)?,
        init_pool.pack()?,
        batch_credit.pack()?,
    ];
    let wallet = LocalWallet::try_from(demo_secret)?;
    let mut transaction = milon_primitives::Transaction::new_with_stamp(
        chain_id,
        stamp,
        Some(wallet.default_account()),
        instructions,
    );
    wallet.sign_transaction(&mut transaction)?;
    Ok(transaction)
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
struct SubmitJson {
    #[serde(default = "default_rpc_url")]
    rpc_url: String,
    #[serde(default = "default_chain_id")]
    chain_id: ChainId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    account_secret_hex: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    stamp: Option<u64>,
}

fn load_json_input() -> Result<SubmitJson, Box<dyn Error>> {
    if let Ok(path) = env::var("MILON_SUBMIT_TX_JSON_FILE") {
        return parse_json_input(&fs::read_to_string(path)?);
    }
    if let Ok(raw) = env::var("MILON_SUBMIT_TX_JSON") {
        return parse_json_input(&raw);
    }
    // parse_json_input(&serde_json::to_string(&SubmitJson::default_for_demo())?)
    Ok(SubmitJson::default_for_demo())
}
fn parse_json_input(raw: &str) -> Result<SubmitJson, Box<dyn Error>> {
    Ok(serde_json::from_str(raw)?)
}

fn next_stamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before unix epoch")
        .as_millis() as u64
        + TX_STAMP_LEAD_MS
}

fn default_rpc_url() -> String {
    DEFAULT_HTTP_RPC_URL.to_string()
}

const fn default_chain_id() -> ChainId {
    ChainId::new(DEFAULT_CHAIN_ID)
}

impl SubmitJson {
    fn default_for_demo() -> Self {
        Self {
            rpc_url: default_rpc_url(),
            chain_id: default_chain_id(),
            account_secret_hex: None,
            stamp: None,
        }
    }
}

async fn wait_tx_hash(rpc_demo: &DemoRpc, tx_hash: TxHash) -> Result<(), ProviderError> {
    let mut last_error = None;
    for _ in 0..CONFIRM_RETRY_ATTEMPTS {
        match rpc_demo.provider.get_transaction_by_hash(tx_hash).await {
            Ok(trans) => {
                println!("trans size: {}", trans.len());
                return Ok(());
            },
            Err(error) => last_error = Some(error),
        }
        tokio::time::sleep(CONFIRM_RETRY_DELAY).await;
    }
    Err(last_error.unwrap_or(ProviderError::InvalidResponse {
        message: "Transaction query did not run".to_string(),
    }))
}

#[cfg(test)]
mod tests {
    use super::build_batch_credit_request;
    use milon_primitives::ChainId;

    #[test]
    fn batch_credit_instructions_decode_with_sdk_idl() {
        let transaction = build_batch_credit_request(ChainId::new(900_000_001), 1_700_000_000)
            .expect("build batch credit request");
        let decoded = transaction
            .instructions()
            .iter()
            .map(milon_client::decode_instruction)
            .collect::<Result<Vec<_>, _>>()
            .expect("decode batch credit instructions");

        assert_eq!(decoded.len(), 2);
        assert_eq!(decoded[0].instruction_name, "InitPool");
        assert_eq!(decoded[1].instruction_name, "BatchCredit");
    }
}
