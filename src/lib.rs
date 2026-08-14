use infra_tracing::{LoggerGuard, tests::setup_logger};
use milon_client::{self as sdk, account, indexer, reliable_transport, token};
use milon_crypto::{Address, PublicKey};
use milon_idl_core::{Method, Signer as InstructionSigner};
use milon_primitives::{ChainId, TxHash};
use milon_provider::{
    FillProvider, NoTerminal, Provider, ProviderBuilder, ProviderError, RecommendedFillers,
    RootProvider,
};
use milon_rpc_client::RpcClient;
use milon_transport::http::HttpInvokeTransport;
use serde::{Deserialize, Serialize};
use std::{
    env,
    error::Error,
    fs, thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use url::Url;

pub mod decode_print;
pub mod errors;
mod signer;
pub use signer::*;

const DEFAULT_HTTP_RPC_URL: &str = "http://47.84.39.153:6280/milon/v1";
const DEFAULT_CHAIN_ID: u64 = 900_000_001;
const TX_STAMP_LEAD_MS: u64 = 30_000;
const CONFIRM_RETRY_ATTEMPTS: usize = 10;
const CONFIRM_RETRY_DELAY: Duration = Duration::from_millis(500);

pub type LocalProvider = FillProvider<RecommendedFillers, NoTerminal, RootProvider>;

pub fn init() -> anyhow::Result<LoggerGuard> {
    setup_logger()
}

pub fn mil_token_address() -> Address {
    Address::from_bytes(&[
        0x18, 0xC0, 0x6A, 0x68, 0x95, 0x94, 0x2A, 0x9A, 0xF0, 0x19, 0x1B, 0x02, 0x32, 0x46, 0x2D,
        0x6F, 0x7C, 0x40, 0x00, 0x00,
    ])
    .expect("MIL token address has 20 bytes")
}

pub struct DemoRpc {
    pub provider: FillProvider<RecommendedFillers, NoTerminal, RootProvider>,
}

impl DemoRpc {
    pub fn connect(rpc_url: &str) -> Result<Self, Box<dyn Error>> {
        // let url: Url = rpc_url.parse()?;
        // let transport = HttpInvokeTransport::new(url);
        // let client = RpcClient::builder().transport(reliable_transport(transport), false);
        // let provider = ProviderBuilder::new().connect_client(client);
        let provider = build_provider(rpc_url)?;
        Ok(Self { provider })
    }
}

pub fn build_provider(rpc_url: &str) -> Result<LocalProvider, Box<dyn Error>> {
    let url: Url = rpc_url.parse()?;
    let transport = HttpInvokeTransport::new(url);
    let client = RpcClient::builder().transport(reliable_transport(transport), false);
    let provider = ProviderBuilder::new().connect_client(client);
    Ok(provider)
}

const INDEXER_API_ROOT: &str = "/v1/milon-idx/";
pub fn connect_indexer(
    indexer_url: &str,
) -> Result<impl milon_client::indexer::Provider, Box<dyn Error>> {
    let origin = Url::parse(indexer_url)?;
    let transport = indexer::HttpTransport::new(origin, INDEXER_API_ROOT)?;
    let client = indexer::HttpClient::builder().transport(indexer::reliable_transport(transport));

    Ok(indexer::ProviderBuilder::new().connect_client(client))
}

pub fn claim_faucet(claimer: Address) -> Result<sdk::PackedInstruction, sdk::idl_core::Error> {
    token::ClaimFaucet {
        claimer: InstructionSigner::new(claimer),
    }
    .pack()
}

pub fn create_account(owner_pk: PublicKey) -> Result<sdk::PackedInstruction, sdk::idl_core::Error> {
    account::Create { owner_pk }.pack()
}

pub async fn wait_for_get_txn<P: Provider>(
    provider: &P,
    tx_hash: TxHash,
) -> Result<Vec<u8>, ProviderError> {
    let mut last_error = None;
    for _ in 0..CONFIRM_RETRY_ATTEMPTS {
        match provider.get_transaction_by_hash(tx_hash).await {
            Ok(view) => return Ok(view),
            Err(error) => last_error = Some(error),
        }
        thread::sleep(CONFIRM_RETRY_DELAY);
    }

    Err(last_error.unwrap_or(ProviderError::InvalidResponse {
        message: "get_txn did not run".to_string(),
    }))
}

// impl RpcDemo {
//     fn connect(rpc_url: &str) -> Result<Self, Box<dyn Error>> {
//         let url: Url = rpc_url.parse()?;
//         let transport = HttpInvokeTransport::new(url);
//         // let svc = ServiceBuilder::new()
//         //     .retry(RpcRetryPolicy::default())
//         //     .timeout(Duration::from_secs(20))
//         //     .service(RpcLatencyFailoverLayer::default().layer(transport));
//         let client = RpcClient::builder().transport(transport, false);
//         let provider = ProviderBuilder::new().connect_client(client);
//
//         Ok(Self { provider })
//     }
// }
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct SubmitJson {
    #[serde(default = "default_rpc_url")]
    pub rpc_url: String,
    #[serde(default = "default_chain_id")]
    pub chain_id: ChainId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stamp: Option<u64>,
}

pub fn load_json_input() -> Result<SubmitJson, Box<dyn Error>> {
    if let Ok(path) = env::var("MILON_SUBMIT_TX_JSON_FILE") {
        return parse_json_input(&fs::read_to_string(path)?);
    }
    if let Ok(raw) = env::var("MILON_SUBMIT_TX_JSON") {
        return parse_json_input(&raw);
    }
    Ok(SubmitJson::default_for_demo())
}

pub fn parse_json_input(raw: &str) -> Result<SubmitJson, Box<dyn Error>> {
    Ok(serde_json::from_str(raw)?)
}

fn default_rpc_url() -> String {
    DEFAULT_HTTP_RPC_URL.to_owned()
}

const fn default_chain_id() -> ChainId {
    ChainId::new(DEFAULT_CHAIN_ID)
}

impl SubmitJson {
    fn default_for_demo() -> Self {
        Self {
            rpc_url: default_rpc_url(),
            chain_id: default_chain_id(),
            stamp: None,
        }
    }
}

pub fn next_stamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before unix epoch")
        .as_millis() as u64
        + TX_STAMP_LEAD_MS
}
