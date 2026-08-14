use milon_client::token;
use milon_crypto::Address;
use milon_idl_core::{Method, Token};
use milon_primitives::PackedInstruction;
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
    let (names, calls) = build_calls(&input)?;
    let outputs = provider.dyn_multicall(calls).await?;

    print_outputs(&names, &outputs)?;
    Ok(())
}

fn connect_provider(rpc_url: &str) -> Result<impl milon_provider::Provider, Box<dyn Error>> {
    let transport = HttpInvokeTransport::new(Url::parse(rpc_url)?);
    let client = RpcClient::builder().transport(transport, false);
    Ok(ProviderBuilder::new().connect_client(client))
}

fn build_calls(
    input: &MultiCallInput,
) -> Result<(Vec<&'static str>, Vec<PackedInstruction>), Box<dyn Error>> {
    let names = vec![
        "token::BalanceOf",
        // "token::FrozenOf",
        "token::TotalSupply",
        "token::GetMetadata",
    ];
    let calls = vec![
        token::BalanceOf {
            token: input.token,
            account: input.account,
        }
        .pack()?,
        // token::FrozenOf {
        //     token: input.token,
        //     account: input.account,
        // }
        // .pack()?,
        token::TotalSupply { token: input.token }.pack()?,
        token::GetMetadata { token: input.token }.pack()?,
    ];
    Ok((names, calls))
}

fn print_outputs(names: &[&str], outputs: &[ViewResult<Token>]) -> Result<(), Box<dyn Error>> {
    for (name, output) in names.iter().zip(outputs) {
        println!("{name}: {}", serde_json::to_string_pretty(output)?);
    }
    Ok(())
}

#[derive(Clone, Debug)]
struct MultiCallInput {
    rpc_url: String,
    token: Address,
    account: Address,
    validator: Address,
}

impl MultiCallInput {
    fn from_env() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            rpc_url: env::var("MILON_RPC_URL").unwrap_or_else(|_| DEFAULT_HTTP_RPC_URL.to_owned()),
            token: env_address("MILON_TOKEN_ADDRESS", default_mil_token_address())?,
            account: env_address("MILON_ACCOUNT_ADDRESS", default_account_address())?,
            validator: env_address("MILON_VALIDATOR_ADDRESS", default_validator_address())?,
        })
    }
}

fn env_address(name: &str, default: Address) -> Result<Address, Box<dyn Error>> {
    match env::var(name) {
        Ok(value) => Ok(Address::from_str_relaxed(&value)?),
        Err(env::VarError::NotPresent) => Ok(default),
        Err(error) => Err(Box::new(error)),
    }
}

fn default_mil_token_address() -> Address {
    Address::from_bytes(&[
        0x18, 0xC0, 0x6A, 0x68, 0x95, 0x94, 0x2A, 0x9A, 0xF0, 0x19, 0x1B, 0x02, 0x32, 0x46, 0x2D,
        0x6F, 0x7C, 0x40, 0x00, 0x00,
    ])
    .expect("MIL token address has 20 bytes")
}

fn default_account_address() -> Address {
    // Address::from_bytes(&[11_u8; 20]).expect("default account address has 20 bytes")
    Address::from_bs58("214RxzUxqRR1P4M5Hjw5mstr1Xs8").unwrap()
}

fn default_validator_address() -> Address {
    // Address::from_bytes(&[12_u8; 20]).expect("default validator address has 20 bytes")
    Address::from_bs58("214RxzUxqRR1P4M5Hjw5mstr1Xs8").unwrap()
}
