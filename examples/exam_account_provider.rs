use milon_client::{
    self as sdk, AccountProviderExt, AccountSignerList, TokenProviderExt, WalletFiller,
};
use milon_crypto::Address;
use milon_local_wallet::{LocalWallet, Signer as _};
use milon_provider::Provider;
use only_sdk_examples::{
    DemoRpc, claim_faucet, decode_print::print_transaction_history, local_ed25519_signer,
    wait_for_get_txn,
};
use std::{env, error::Error, time::Duration};
use tokio::{time, time::timeout};

// const DEFAULT_HTTP_RPC_URL: &str = "http://127.0.0.1:6280/milon/v1";
const DEFAULT_HTTP_RPC_URL: &str = "http://47.84.39.153:6280/milon/v1";
const DEFAULT_SIGNER_SEED: u8 = 2;

/// 运行方式：
///
///   cargo run --example providers_account_test
///
///   强制测试 create_account：
///
///   MILON_ACCOUNT_METHOD=create cargo run --example providers_account_test
///
///   MILON_ACCOUNT_SIGNER_SEED=123 MILON_ACCOUNT_METHOD=create cargo run --example providers_account_test
///
///   可选参数：
///
///   MILON_RPC_URL=http://127.0.0.1:6280/milon/v1 \
///   MILON_ACCOUNT_SIGNER_SEED=2 \
///   cargo run --example providers_account_test
///
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut rpc = DemoRpc::connect(DEFAULT_HTTP_RPC_URL)?;

    create_get_account(&rpc).await?;

    // let addr = Address::from_bs58("214RxzUxqRR1P4M5Hjw5mstr1Xs8")?;
    // get_account_signers(&rpc, addr).await?;
    Ok(())
}

async fn create_get_account(rpc: &DemoRpc) -> Result<(), Box<dyn Error>> {
    let provider = rpc.provider.clone();
    let account_signer = local_ed25519_signer(1)?;
    let wallet = LocalWallet::new(account_signer);

    // claim_faucet2(rpc, &wallet).await?;

    let provider = provider.with_wallet_filler(WalletFiller::new(wallet.clone()));
    let res = provider.create_account().await?;
    println!("Created account response: {}", res);
    time::sleep(Duration::from_secs(3)).await;

    let account = timeout(
        Duration::from_secs(10),
        provider.get_account(wallet.default_account()),
        // rpc.provider.create_account(&wallet),
    )
    .await??;
    println!("account = {account:?}");
    Ok(())
}

async fn claim_faucet2(rpc: &DemoRpc, wallet: &LocalWallet) -> Result<(), Box<dyn Error>> {
    let _ = (rpc, wallet);

    println!(
        "Created account address: {}",
        wallet.default_account().to_bs58()
    );

    Ok(())
}

async fn get_account_signers(rpc: &DemoRpc, address: Address) -> Result<(), Box<dyn Error>> {
    let account = rpc.provider.account(address).await?;
    println!("account = {account:?}");

    let signers = rpc.provider.list_signers(address).await?;
    println!("signers = {signers:?}");

    for (pk, _, _) in signers.1 {
        let addr = Address::from_public_key(&pk);
        println!("pk.address = {}", addr.to_bs58());
    }

    Ok(())
}

async fn create_account() -> Result<(), Box<dyn Error>> {
    let input = ProviderAccountInput::from_env()?;
    let rpc = DemoRpc::connect(&input.rpc_url)?;
    let account_signer = local_ed25519_signer(input.signer_seed)?;
    let account_public_key = account_signer.public_key().clone();
    let account_address = account_signer.address();
    let wallet = LocalWallet::new(account_signer);

    println!("rpc_url={}", input.rpc_url);
    println!("account_public_key={}", account_public_key.to_bs58());
    println!("account_address={account_address}");
    println!("account_method={}", input.method.as_str());

    let _ = (&input, &rpc, &wallet);

    let account = rpc.provider.account(account_address).await?;
    let signers = rpc.provider.list_signers(account_address).await?;
    println!("account={account:?}");
    println!("signers={signers:?}");

    Ok(())
}

#[derive(Clone, Debug)]
struct ProviderAccountInput {
    rpc_url: String,
    signer_seed: u8,
    method: AccountMethod,
}

impl ProviderAccountInput {
    fn from_env() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            rpc_url: env::var("MILON_RPC_URL").unwrap_or_else(|_| DEFAULT_HTTP_RPC_URL.to_owned()),
            signer_seed: signer_seed_from_env()?,
            method: AccountMethod::from_env()?,
        })
    }
}

#[derive(Clone, Copy, Debug)]
enum AccountMethod {
    Create,
    Ensure,
}

impl AccountMethod {
    fn from_env() -> Result<Self, Box<dyn Error>> {
        match env::var("MILON_ACCOUNT_METHOD") {
            Ok(raw) if raw.eq_ignore_ascii_case("create") => Ok(Self::Create),
            Ok(raw) if raw.eq_ignore_ascii_case("ensure") => Ok(Self::Ensure),
            Ok(raw) => Err(format!("unsupported MILON_ACCOUNT_METHOD={raw}").into()),
            Err(_) => Ok(Self::Ensure),
        }
    }

    const fn as_str(self) -> &'static str {
        match self {
            Self::Create => "create",
            Self::Ensure => "ensure",
        }
    }

    async fn submit(
        self,
        rpc: &DemoRpc,
        wallet: &LocalWallet,
    ) -> sdk::ClientResult<sdk::primitives::TxHash> {
        match self {
            Self::Create | Self::Ensure => panic!("use ProviderBuilder::wallet for write examples"),
        }
    }
}

fn signer_seed_from_env() -> Result<u8, Box<dyn Error>> {
    match env::var("MILON_ACCOUNT_SIGNER_SEED") {
        Ok(raw) => Ok(raw.parse()?),
        Err(_) => Ok(DEFAULT_SIGNER_SEED),
    }
}

/*
rpc_url=http://47.84.39.153:6280/milon/v1
account_public_key=9hSR6S7WPtxmTojgo6GG3k4yDPecgJY292j7xrsUGWBu
account_address=3WoBgRDRzQ9omYBfXF8H6yFUaKWA
account_method=ensure
submit_tx_hash=HdQEXo5Rhj9JVz4hk6byU7jhuNuHJyU8fVZap3wy14mQ
transaction history:
  stamp: 1784986630728
  payer_signature_index: Some(0)
  signature_count: 1
  tx_id: 000000000000106e00000000
  tx_hash: HdQEXo5Rhj9JVz4hk6byU7jhuNuHJyU8fVZap3wy14mQ
  state: 1 (success)
  error: None
  gas_charged: 908
  instruction_count: 1
    instruction[0]: {"app_id":1,"app_name":"account","instruction_name":"EnsureAccount","token":{"method":"EnsureAccount","fields":[{"name":"owner_pk","value":"9hSR6S7WPtxmTojgo6GG3k4yDPecgJY292j7xrsUGWBu"}]}}
  access_resource_count: 3
    access[0].resource_id: FixedBytes([1, 145, 133, 92, 160, 147, 123, 24, 6, 180, 180, 59, 77, 41, 127, 210, 104, 210])
      access[0].first_snapshot: inline DecodedResource { name: "Account", type_tag: 17390915333023917609, token: Struct { name: "Account", fields: [NamedToken { name: "bitmap", value: Bitmap64(Bitmap64(1)) }, NamedToken { name: "weight", value: U8(1) }, NamedToken { name: "threshold", value: U8(1) }] } }
      access[0].last_written: inline DecodedResource { name: "Account", type_tag: 17390915333023917609, token: Struct { name: "Account", fields: [NamedToken { name: "bitmap", value: Bitmap64(Bitmap64(1)) }, NamedToken { name: "weight", value: U8(1) }, NamedToken { name: "threshold", value: U8(1) }] } }
    access[1].resource_id: FixedBytes([2, 34, 137, 224, 68, 189, 172, 80, 49, 188, 203, 77, 4, 136, 157, 135, 66, 25])
      access[1].first_snapshot: inline DecodedResource { name: "u64", type_tag: 5563585020063213298, token: U64(9935743) }
      access[1].last_written: inline DecodedResource { name: "u64", type_tag: 5563585020063213298, token: U64(9934835) }
    access[2].resource_id: FixedBytes([2, 34, 219, 135, 88, 254, 179, 118, 85, 215, 237, 148, 129, 233, 50, 129, 207, 16])
      access[2].first_snapshot: inline DecodedResource { name: "u64", type_tag: 5563585020063213298, token: U64(80863020) }
      access[2].last_written: inline DecodedResource { name: "u64", type_tag: 5563585020063213298, token: U64(80863928) }
  event_count: 0
account=Account { bitmap: Bitmap64(1), weight: 1, threshold: 1 }
signers=(Account { bitmap: Bitmap64(1), weight: 1, threshold: 1 }, [(9hSR6S7WPtxmTojgo6GG3k4yDPecgJY292j7xrsUGWBu, 0, 1)])

 */
