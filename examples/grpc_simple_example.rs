use milon_client::reliable_grpc_transport;
use milon_provider::{Provider, ProviderBuilder};
use milon_rpc_client::RpcClient;
use milon_transport::grpc::{
    GrpcHealthClient, GrpcInvokeTransport, GrpcTransportConfig, HealthStatus,
};
use std::{env, error::Error, time::Duration};

const DEFAULT_GRPC_URL: &str = "http://127.0.0.1:50051";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let endpoint = env::var("MILON_GRPC_URL").unwrap_or_else(|_| DEFAULT_GRPC_URL.to_owned());
    let config = GrpcTransportConfig::new(endpoint)
        .with_connect_timeout(Duration::from_secs(5))
        .with_request_timeout(Duration::from_secs(10))
        .with_read_timeout(Duration::from_secs(10))
        .with_simulation_timeout(Duration::from_secs(20))
        .with_submit_timeout(Duration::from_secs(30))
        .with_max_concurrent_requests(64)
        .with_http2_keep_alive_interval(Some(Duration::from_secs(30)));
    let health = GrpcHealthClient::connect(config.clone()).await?;
    if health.check_rpc().await? != HealthStatus::Serving {
        return Err(std::io::Error::other("gRPC RPC service is not serving").into());
    }
    let transport = GrpcInvokeTransport::connect(config).await?;
    let client = RpcClient::builder().transport(reliable_grpc_transport(transport), false);
    let provider = ProviderBuilder::new().connect_client(client);
    let chain_head = provider.get_chain_head().await?;
    println!("{} {} {}", chain_head.chain_id.get(), chain_head.block_height, chain_head.block_hash);
    Ok(())
}
