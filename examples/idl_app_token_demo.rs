use milon_client::TokenProviderExt;
use milon_crypto::Address;
use only_sdk_examples::DemoRpc;
use std::error::Error;

// const DEFAULT_HTTP_RPC_URL: &str = "http://127.0.0.1:6280/milon/v1";
const DEFAULT_HTTP_RPC_URL: &str = "http://47.84.39.153:6280/milon/v1";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rpc = DemoRpc::connect(DEFAULT_HTTP_RPC_URL)?;

    get_total_supply(&rpc).await?;
    balance_of(&rpc).await?;

    Ok(())
}

async fn get_total_supply(rpc: &DemoRpc) -> Result<(), Box<dyn Error>> {
    let token_addr = Address::from_bs58("M11on1111111111111111111111")?;

    let total_supply = rpc.provider.total_supply(token_addr).await?;
    println!("total_supply = {total_supply}");
    Ok(())
}

async fn balance_of(rpc: &DemoRpc) -> Result<(), Box<dyn Error>> {
    let token_addr = Address::from_bs58("M11on1111111111111111111111")?;
    let acct_addr = Address::from_bs58("LsJBjvf6zx63BZsv6Q4YkurAETN")?;

    let balance = rpc.provider.balance_of(token_addr, acct_addr).await?;
    println!("balance: {}", balance);
    Ok(())
}
