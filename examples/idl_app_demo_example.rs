use milon_client::{self as sdk, WalletFiller, demo};
use milon_crypto::{Address, secretkey::SecretKey};
use milon_idl_core::{Method, Signer as InstructionSigner};
use milon_local_wallet::{
    AccountAuthorization, LocalSigner, LocalWallet, SignatureAlgorithm, Signer, SigningPlan,
};
use milon_provider::{IdlProviderExt, TransactionRequest};
use only_sdk_examples::{
    DemoRpc, decode_print::print_transaction_history, local_ed25519_signer, wait_for_get_txn,
};
use std::error::Error;

const DEFAULT_HTTP_RPC_URL: &str = "http://47.84.39.153:6280/milon/v1";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rpc = DemoRpc::connect(DEFAULT_HTTP_RPC_URL)?;
    echo_mode(&rpc).await?;
    // init_pool(&rpc).await
    Ok(())
}

async fn echo_mode(rpc: &DemoRpc) -> Result<(), Box<dyn Error>> {
    let result = rpc.provider.call(build_echo_mode_method()).await?;
    println!("echo_mode result: {result:?}");
    Ok(())
}

async fn init_pool(rpc: &DemoRpc) -> Result<(), Box<dyn Error>> {
    let payer_signer = local_ed25519_signer(12)?;
    let payer = payer_signer.address();
    let pool_secret = SecretKey::new_pure();
    let pool = Address::from_public_key(&pool_secret.ed25519_public());
    let instruction = build_init_pool_instruction(pool)?;

    let mut wallet = LocalWallet::new(payer_signer);
    wallet.register_signer(LocalSigner::from_secret_key(
        pool_secret,
        SignatureAlgorithm::Ed25519,
    )?)?;

    let plan = SigningPlan::new(payer)
        .authorize(AccountAuthorization::new(payer, vec![0]).with_payer())
        .authorize(AccountAuthorization::new(pool, vec![0]));
    let request = TransactionRequest::new(vec![instruction])?.with_signing_plan(plan);
    let provider = rpc.provider.with_wallet_filler(WalletFiller::new(wallet));
    let tx_hash = provider.send_transaction(request).await?;
    println!("init_pool submit tx_hash: {tx_hash}");

    let raw = wait_for_get_txn(&provider, tx_hash).await?;
    let history = sdk::decode_transaction_history(&raw)?;
    print_transaction_history(&history);
    Ok(())
}

fn build_echo_mode_method() -> demo::EchoMode {
    demo::EchoMode {
        mode: demo::DemoMode::Three { 0: 101, 1: 213 },
    }
}

fn build_init_pool_instruction(
    pool: Address,
) -> Result<sdk::PackedInstruction, sdk::idl_core::Error> {
    demo::InitPool {
        pool: InstructionSigner::new(pool),
        label: "idl app demo pool".to_owned(),
    }
    .pack()
}

#[cfg(test)]
mod tests {
    use super::{build_echo_mode_method, build_init_pool_instruction};
    use milon_crypto::Address;
    use milon_idl_core::Method;

    #[test]
    fn builds_echo_mode_three_instruction() {
        let instruction = build_echo_mode_method().pack().unwrap();

        assert!(!instruction.as_slice().is_empty());
    }

    #[test]
    fn builds_init_pool_instruction() {
        let instruction =
            build_init_pool_instruction(Address::from_bytes(&[7; 20]).unwrap()).unwrap();

        assert!(!instruction.as_slice().is_empty());
    }
}
