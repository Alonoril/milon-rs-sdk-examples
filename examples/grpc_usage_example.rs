use milon_client::reliable_grpc_transport;
use milon_provider::{Provider, ProviderBuilder};
use milon_rpc_client::RpcClient;
use milon_transport::grpc::{
    GrpcHealthClient, GrpcInvokeTransport, GrpcTransportConfig, HealthStatus,
};
use only_sdk_examples::{build_grpc_config, init};
use std::{env, error::Error, time::Duration};
use tracing::info;

const DEFAULT_GRPC_URL: &str = "http://127.0.0.1:50051";
// const DEFAULT_GRPC_URL: &str = "http://8.218.101.239:50051";

/// Run with:
///
/// cargo run --manifest-path only-sdk-examples/Cargo.toml \
///     --example grpc_usage_example
///
/// For TLS, use an `https://` endpoint and optionally set:
///
/// MILON_GRPC_CA_FILE=/path/to/ca.pem
/// MILON_GRPC_TLS_DOMAIN=node.example.com
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _logger = init()?;
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

    let chain_head = provider.get_chain_head().await?;
    info!("chain_id: {}", chain_head.chain_id.get());
    info!("block_height: {}", chain_head.block_height);
    info!("block_hash: {}", chain_head.block_hash);

    Ok(())
}

