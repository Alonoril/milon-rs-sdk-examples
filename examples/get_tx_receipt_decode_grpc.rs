use infra_core::map_err_logged;
use infra_tracing::tests::setup_logger;
use milon_client::{self as sdk, primitives::TxHash, reliable_grpc_transport};
use milon_provider::{Provider, ProviderBuilder};
use milon_rpc_client::RpcClient;
use milon_transport::grpc::{GrpcHealthClient, GrpcInvokeTransport, HealthStatus};
use only_sdk_examples::{
    LocalProvider, build_grpc_config, decode_print::print_transaction_history, errors::ExmErr,
};
use std::{env, error::Error};
use tracing::{Level, info};

// const DEFAULT_GRPC_URL: &str = "http://127.0.0.1:50051";
const DEFAULT_GRPC_URL: &str = "http://8.218.101.239:50051";
// const DEFAULT_TX_HASH_BS58: &str = "B7UyYPhBC1pwvrhkQ5WSQhq2FA3aGfcpsL7U95ABJhgz";
const DEFAULT_TX_HASH_BS58: &str = "B7UyYPhBC1pwvrhkQ5WSQhq2FA3aGfcpsL7U95ABJhgz";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ss = setup_logger(Level::INFO)?;
    let provider = grpc_provider().await?;

    let chain_state = provider.get_chain_head().await?;
    info!("chain_head: {:?}", chain_state);

    // get_transaction_history(&provider).await?;

    Ok(())
}

async fn grpc_provider() -> Result<LocalProvider, Box<dyn Error>> {
    let endpoint = env::var("MILON_GRPC_URL").unwrap_or_else(|_| DEFAULT_GRPC_URL.to_owned());
    let config = build_grpc_config(endpoint)?;

    // Health is optional, but it is useful at startup to verify the versioned
    // Milon RPC service before exposing the provider to application code.
    let health = GrpcHealthClient::connect(config.clone()).await?;
    let health_status = health.check_rpc().await?;
    if health_status != HealthStatus::Serving {
        return Err(std::io::Error::other(format!(
            "Milon gRPC RPC service is not serving: {health_status:?}"
        ))
        .into());
    }
    info!("Milon gRPC RPC service: {health_status:?}");

    // Connect the postcard-over-gRPC transport, then reuse the existing
    // RpcClient and Provider APIs. No gRPC-specific provider is required.
    let transport = GrpcInvokeTransport::connect(config).await?;
    let client = RpcClient::builder().transport(reliable_grpc_transport(transport), false);
    let provider = ProviderBuilder::new().connect_client(client);

    Ok(provider)

    // let chain_head = provider.get_chain_head().await?;
    // info!("chain_id: {}", chain_head.chain_id.get());
    // info!("block_height: {}", chain_head.block_height);
    // info!("block_hash: {}", chain_head.block_hash);
}

async fn get_transaction_history(provider: &LocalProvider) -> Result<(), Box<dyn Error>> {
    let tx_hash = load_tx_hash()?;

    // let transport = HttpInvokeTransport::new(Url::parse(rpc_url)?);
    // let client = RpcClient::builder().transport(transport, false);
    // let raw: RawBytes = client
    //     .request(
    //         SdkMethod::GetTransactionByHash,
    //         TxHashParam::new(tx_hash.to_bytes()),
    //     )
    //     .await?;
    let raw: Vec<u8> = provider.get_transaction_by_hash(tx_hash).await?;
    let history =
        sdk::decode_transaction_history(&raw).map_err(map_err_logged!(ExmErr::SdkDecodeErr))?;
    print_transaction_history(&history);

    Ok(())
}

fn load_tx_hash() -> Result<TxHash, Box<dyn Error>> {
    if let Ok(value) = env::var("MILON_TX_HASH_BS58") {
        return Ok(TxHash::from_bs58(value.trim())?);
    }
    if let Ok(value) = env::var("MILON_TX_HASH_HEX") {
        return parse_tx_hash_hex(value.trim());
    }
    Ok(TxHash::from_bs58(DEFAULT_TX_HASH_BS58)?)
}

fn parse_tx_hash_hex(value: &str) -> Result<TxHash, Box<dyn Error>> {
    let bytes = hex::decode(value)?;
    let hash: [u8; 32] = bytes.try_into().map_err(|bytes: Vec<u8>| {
        format!("expected 32-byte tx hash hex, got {} bytes", bytes.len())
    })?;
    Ok(TxHash::new(hash))
}

#[cfg(test)]
mod tests {
    use milon_client::demo;
    use milon_idl_core::{NamedToken, Token};
    use only_sdk_examples::decode_print::decode_inline_resource;

    #[test]
    fn inline_resource_decodes_with_sdk_type_tag() {
        let raw = postcard::to_allocvec(&demo::Label {
            text: "resource".to_owned(),
        })
        .expect("encode resource");

        let decoded = decode_inline_resource(demo::Label::TYPE_TAG, &raw).expect("decode resource");

        assert_eq!(decoded.name, "Label");
        assert_eq!(decoded.token, Token::Struct {
            name: "Label",
            fields: vec![NamedToken {
                name: "text",
                value: Token::String("resource".to_owned()),
            }],
        });
    }
}
