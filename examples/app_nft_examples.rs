use milon_client::{self as sdk, WalletFiller, nft, reliable_grpc_transport};
use milon_crypto::{Address, secretkey::SecretKey};
use milon_idl_core::{Method, Signer as InstructionSigner};
use milon_local_wallet::{
    AccountAuthorization, LocalSigner, LocalWallet, SignatureAlgorithm, Signer, SigningPlan,
};
use milon_primitives::PackedInstruction;
use milon_provider::{Provider, ProviderBuilder, SendableTransaction, TransactionRequest};
use milon_rpc_client::RpcClient;
use milon_transport::grpc::{
    GrpcHealthClient, GrpcInvokeTransport, GrpcTransportConfig, HealthStatus,
};
use only_sdk_examples::{
    decode_print::{print_decoded_instructions, print_simulate_receipt, print_transaction_history},
    init, local_ed25519_signer, wait_for_get_txn,
};
use std::{env, error::Error, time::Duration};
use tracing::info;

const BATCH_SEED: u64 = 2;
const DEFAULT_GRPC_URL: &str = "http://127.0.0.1:50051";
// const DEFAULT_GRPC_URL: &str = "http://8.218.101.239:50051";

/// Run with:
///
/// ```text
/// cargo run --manifest-path only-sdk-examples/Cargo.toml \
///     --example app_nft_examples
/// ```
///
/// Set `MILON_NFT_SUBMIT=1` to submit after simulation. Without it the example
/// only builds, signs, and simulates the complete NFT transaction.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _logger = init()?;
    let endpoint = env::var("MILON_GRPC_URL").unwrap_or_else(|_| DEFAULT_GRPC_URL.to_owned());
    let config = build_config(endpoint)?;
    let health = GrpcHealthClient::connect(config.clone()).await?;
    let health_status = health.check_rpc().await?;
    if health_status != HealthStatus::Serving {
        return Err(std::io::Error::other(format!(
            "Milon gRPC RPC service is not serving: {health_status:?}"
        ))
        .into());
    }
    info!(?health_status, "Milon gRPC RPC service is ready");

    let transport = GrpcInvokeTransport::connect(config).await?;
    let client = RpcClient::builder().transport(reliable_grpc_transport(transport), false);
    let base_provider = ProviderBuilder::new().connect_client(client);

    let owner_signer = local_ed25519_signer(12)?;
    let owner = owner_signer.address();
    let recipient = Address::from_bytes(&[13; 20])?;
    let collection_secret = SecretKey::from_bytes(&[31; 32])?;
    let collection_signer =
        LocalSigner::from_secret_key(collection_secret, SignatureAlgorithm::Ed25519)?;
    let collection = collection_signer.address();

    let mut wallet = LocalWallet::new(owner_signer);
    wallet.register_signer(collection_signer)?;
    let provider = base_provider.with_wallet_filler(WalletFiller::new(wallet));
    let request = build_nft_request(collection, owner, recipient)?;
    print_decoded_instructions(request.instructions());

    let sendable = provider.fill(request.clone()).await?;
    let transaction = match sendable {
        SendableTransaction::Transaction(transaction) => transaction,
        SendableTransaction::Request(_) => {
            return Err("wallet filler returned an incomplete NFT transaction".into());
        },
    };
    info!(tx_hash = %transaction.tx_hash(), "local NFT transaction prepared");

    let response = provider.simulate_transaction(transaction).await?;
    let receipt = sdk::decode_transaction_response(&response)?;
    print_simulate_receipt(&receipt);

    if env::var_os("MILON_NFT_SUBMIT").is_some() {
        let tx_hash = provider.send_transaction(request).await?;
        println!("NFT submit tx_hash: {tx_hash}");
        let raw = wait_for_get_txn(&provider, tx_hash).await?;
        let history = sdk::decode_transaction_history(&raw)?;
        print_transaction_history(&history);
    }

    Ok(())
}

fn build_config(endpoint: String) -> Result<GrpcTransportConfig, Box<dyn Error>> {
    let mut config = GrpcTransportConfig::new(endpoint)
        .with_connect_timeout(Duration::from_secs(5))
        .with_request_timeout(Duration::from_secs(10))
        .with_read_timeout(Duration::from_secs(10))
        .with_simulation_timeout(Duration::from_secs(20))
        .with_submit_timeout(Duration::from_secs(30))
        .with_max_concurrent_requests(64)
        .with_http2_keep_alive_interval(Some(Duration::from_secs(30)));

    if let Ok(path) = env::var("MILON_GRPC_CA_FILE") {
        config = config.with_tls_ca(std::fs::read(path)?);
    }
    if let Ok(domain) = env::var("MILON_GRPC_TLS_DOMAIN") {
        config = config.with_tls_domain(domain);
    }
    if let Ok(traceparent) = env::var("MILON_TRACEPARENT") {
        config = config.with_traceparent(traceparent);
    }

    Ok(config)
}

fn build_nft_request(
    collection: Address,
    owner: Address,
    recipient: Address,
) -> Result<TransactionRequest, Box<dyn Error>> {
    let instructions = vec![
        build_create_collection(collection, owner)?,
        build_create_unique(collection, owner)?,
        build_transfer(owner, collection, 1, recipient, None)?,
        build_create_batch(collection, vec![owner], vec![10])?,
        build_mint_batch(collection, BATCH_SEED, vec![owner], vec![5])?,
        build_transfer(owner, collection, BATCH_SEED, recipient, Some(2))?,
    ];
    let signing_plan = SigningPlan::new(owner)
        .authorize(AccountAuthorization::new(collection, vec![0]))
        .authorize(AccountAuthorization::new(owner, vec![1, 2, 3, 4, 5]).with_payer());
    Ok(TransactionRequest::new(instructions)?.with_signing_plan(signing_plan))
}

fn build_create_collection(
    collection: Address,
    owner: Address,
) -> Result<PackedInstruction, sdk::idl_core::Error> {
    nft::CreateCollection {
        collection: InstructionSigner::new(collection),
        owner,
        update_author: owner,
        freeze_author: owner,
        transferable: true,
        metadata: nft::Metadata {
            name: "SDK NFT Example".to_owned(),
            symbol: "SDKNFT".to_owned(),
            cover_uri: "ipfs://sdk-example-collection".to_owned(),
            external_url: "https://example.test/sdk-nft".to_owned(),
        },
        royalty: nft::Royalty {
            recipient: owner,
            bps: 250,
        },
    }
    .pack()
}

fn build_create_unique(
    collection: Address,
    to: Address,
) -> Result<PackedInstruction, sdk::idl_core::Error> {
    nft::CreateUnique {
        collection,
        to,
        metadata: item_metadata("Unique SDK NFT", "UNIQ"),
        attributes: vec![nft::MetadataAttribute {
            trait_type: "mode".to_owned(),
            value: "unique".to_owned(),
        }],
        properties: vec![nft::MetadataProperty {
            key: "source".to_owned(),
            value: "sdk-example".to_owned(),
        }],
    }
    .pack()
}

fn build_create_batch(
    collection: Address,
    to: Vec<Address>,
    amounts: Vec<u64>,
) -> Result<PackedInstruction, sdk::idl_core::Error> {
    nft::CreateBatch {
        collection,
        to,
        amounts,
        metadata: item_metadata("Batch SDK NFT", "BATCH"),
        attributes: vec![nft::MetadataAttribute {
            trait_type: "mode".to_owned(),
            value: "batch".to_owned(),
        }],
        properties: vec![nft::MetadataProperty {
            key: "source".to_owned(),
            value: "sdk-example".to_owned(),
        }],
    }
    .pack()
}

fn build_mint_batch(
    collection: Address,
    seed: u64,
    to: Vec<Address>,
    amounts: Vec<u64>,
) -> Result<PackedInstruction, sdk::idl_core::Error> {
    nft::MintBatch {
        collection,
        seed,
        to,
        amounts,
    }
    .pack()
}

fn build_transfer(
    from: Address,
    collection: Address,
    seed: u64,
    to: Address,
    amount: Option<u64>,
) -> Result<PackedInstruction, sdk::idl_core::Error> {
    nft::Transfer {
        from: InstructionSigner::new(from),
        collection,
        seed,
        to,
        amount,
    }
    .pack()
}

fn item_metadata(name: &str, symbol: &str) -> nft::Metadata {
    nft::Metadata {
        name: name.to_owned(),
        symbol: symbol.to_owned(),
        cover_uri: format!("ipfs://sdk-example-{symbol}"),
        external_url: format!("https://example.test/sdk-nft/{symbol}"),
    }
}

#[cfg(test)]
mod tests {
    use super::{build_create_batch, build_create_unique, build_mint_batch, build_transfer};
    use milon_client::decode_instruction;
    use milon_crypto::Address;

    fn address(seed: u8) -> Address {
        Address::from_bytes(&[seed; 20]).expect("valid test address")
    }

    fn instruction_name(instruction: milon_primitives::PackedInstruction) -> &'static str {
        decode_instruction(instruction)
            .expect("instruction decodes with the SDK NFT IDL")
            .instruction_name
    }

    #[test]
    fn nft_creation_methods_pack_with_expected_idl_names() {
        assert_eq!(
            instruction_name(
                build_create_unique(address(1), address(2)).expect("pack CreateUnique")
            ),
            "CreateUnique"
        );
        assert_eq!(
            instruction_name(
                build_create_batch(address(1), vec![address(2)], vec![10])
                    .expect("pack CreateBatch")
            ),
            "CreateBatch"
        );
    }

    #[test]
    fn nft_issue_and_transfer_methods_pack_with_expected_idl_names() {
        assert_eq!(
            instruction_name(
                build_mint_batch(address(1), 7, vec![address(2)], vec![10])
                    .expect("pack MintBatch")
            ),
            "MintBatch"
        );
        assert_eq!(
            instruction_name(
                build_transfer(address(2), address(1), 7, address(3), Some(2))
                    .expect("pack Transfer")
            ),
            "Transfer"
        );
    }
}
