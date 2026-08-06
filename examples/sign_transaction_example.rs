use milon_client as sdk;
use milon_crypto::Address;
use milon_local_wallet::{LocalSigner, LocalWallet, Signer};
use milon_primitives::{ChainId, Transaction};
use milon_provider::Provider;
use only_sdk_examples::{
    DemoRpc, claim_faucet, create_account, decode_print::print_transaction_history,
    load_json_input, local_ed25519_signer, next_stamp, wait_for_get_txn,
};
use std::error::Error;

// const DEFAULT_HTTP_RPC_URL: &str = "http://127.0.0.1:6380/milon/v1";
const DEFAULT_HTTP_RPC_URL: &str = "http://47.84.39.153:6280/milon/v1";
const DEFAULT_PUBKEY_BS58: &str = "AKnL4NNf3DGWZJS6cPknBuEGnVsV4A4m5tgebLHaRSZ9";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let input = load_json_input()?;
    let rpc = DemoRpc::connect(&input.rpc_url)?;
    let signer_a = local_ed25519_signer(2)?;

    let pubkey = signer_a.public_key();
    println!("publickey-hex: {}", pubkey.to_hex());
    println!("publick-bs58: {}", pubkey.to_bs58());

    let tx = build_signed_account_create_transaction(
        input.chain_id,
        input.stamp.unwrap_or_else(next_stamp),
        signer_a,
    )?;
    println!("local tx_hash: {}", tx.tx_hash());

    // submit tx
    let tx_hash = rpc.provider.submit_transaction(tx).await?;
    println!("submit txn tx_hash: {tx_hash}");

    let raw: Vec<u8> = wait_for_get_txn(&rpc.provider, tx_hash).await?;
    let history = sdk::decode_transaction_history(&raw)?;
    print_transaction_history(&history);
    Ok(())
}

fn build_signed_account_create_transaction(
    chain_id: ChainId,
    stamp: u64,
    account_signer: LocalSigner,
) -> Result<Transaction, Box<dyn Error>> {
    let account_pk = account_signer.public_key().clone();
    let account = Address::from_public_key(&account_pk);
    let wallet = LocalWallet::new(account_signer);

    let instructions = vec![claim_faucet(account)?, create_account(account_pk)?];
    let mut transaction = Transaction::new_with_stamp(chain_id, stamp, Some(account), instructions);
    wallet.sign_transaction(&mut transaction)?;
    Ok(transaction)
}
