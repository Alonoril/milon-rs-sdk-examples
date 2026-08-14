use milon_client::token;
use milon_crypto::Address;
use milon_provider::{IdlProviderExt, ProviderBuilder, ViewResult};
use milon_rpc_client::RpcClient;
use milon_transport::http::HttpInvokeTransport;
use std::{env, error::Error};
use url::Url;

// const DEFAULT_HTTP_RPC_URL: &str = "http://127.0.0.1:6380/milon/v1";
const DEFAULT_HTTP_RPC_URL: &str = "http://47.84.39.153:6280/milon/v1";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let input = MultiCallInput::from_env()?;
    let provider = connect_provider(&input.rpc_url)?;
    let (names, calls) = build_calls()?;
    let outputs: Vec<ViewResult<u64>> = provider.typed_multicall(calls).await?;

    print_outputs(&names, &outputs)?;
    Ok(())
}

fn connect_provider(rpc_url: &str) -> Result<impl milon_provider::Provider, Box<dyn Error>> {
    let transport = HttpInvokeTransport::new(Url::parse(rpc_url)?);
    let client = RpcClient::builder().transport(transport, false);
    Ok(ProviderBuilder::new().connect_client(client))
}

fn build_calls() -> Result<(Vec<&'static str>, Vec<token::BalanceOf>), Box<dyn Error>> {
    let names = vec!["token::BalanceOf"];
    let calls = vec![
        token::BalanceOf {
            token: Address::from_bs58("M11on1111111111111111111111").unwrap(),
            account: Address::from_bs58("214RxzUxqRR1P4M5Hjw5mstr1Xs8").unwrap(),
        },
        token::BalanceOf {
            token: Address::from_bs58("M11on1111111111111111111111").unwrap(),
            account: Address::from_bs58("3pHqrfVpw4ziiWZ2S6graADk8sXu").unwrap(),
        },
        token::BalanceOf {
            token: Address::from_bs58("M11on1111111111111111111111").unwrap(),
            account: Address::from_bs58("bDgxrQC5eqoMm6D13sU3oET1Zbz").unwrap(),
        },
        token::BalanceOf {
            token: Address::from_bs58("M11on1111111111111111111111").unwrap(),
            account: Address::from_bs58("3FELz7YwgHc2nfD3Rb55iuCVTSsh").unwrap(),
        },
        token::BalanceOf {
            token: Address::from_bs58("M11on1111111111111111111111").unwrap(),
            account: Address::from_bs58("2EBm6QUpKBSfVtUBbHedMCmfvAAD").unwrap(),
        },
        token::BalanceOf {
            token: Address::from_bs58("M11on1111111111111111111111").unwrap(),
            account: Address::from_bs58("2LYoqzv4XsBGFuG1spSeLhjNxUYA").unwrap(),
        },
        token::BalanceOf {
            token: Address::from_bs58("M11on1111111111111111111111").unwrap(),
            account: Address::from_bs58("2U8fNBEtoTr6ud2jkRaTod1PCHJJ").unwrap(),
        },
    ];
    Ok((names, calls))
}

fn print_outputs(address: &[&str], outputs: &[ViewResult<u64>]) -> Result<(), Box<dyn Error>> {
    print!("address: {address:?} outputs: {outputs:?}");
    Ok(())
}

#[derive(Clone, Debug)]
struct MultiCallInput {
    rpc_url: String,
}

impl MultiCallInput {
    fn from_env() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            rpc_url: env::var("MILON_RPC_URL").unwrap_or_else(|_| DEFAULT_HTTP_RPC_URL.to_owned()),
        })
    }
}
