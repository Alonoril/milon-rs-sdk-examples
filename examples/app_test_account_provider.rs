use milon_client::{self as sdk, AccountProviderExt, TokenProviderExt, WalletFiller, token};
use milon_crypto::Address;
use milon_idl_core::{Method, Signer as ChainSigner};
use milon_local_wallet::{LocalWallet, Signer};
use milon_primitives::{AccountAuthorization, SigningPlan};
use milon_provider::{IdlProviderExt, Provider, TransactionRequest};
use only_sdk_examples::{
    DemoRpc, decode_print::print_transaction_history, local_ed25519_signer, wait_for_get_txn,
};
use std::{error::Error, time::Duration};
use tokio::{time, time::timeout};

// const DEFAULT_HTTP_RPC_URL: &str = "http://127.0.0.1:6280/milon/v1";
const DEFAULT_HTTP_RPC_URL: &str = "http://47.84.39.153:6280/milon/v1";

async fn create_account(rpc: &DemoRpc) -> Result<(), Box<dyn Error>> {
    let provider = &rpc.provider;
    let account_signer = local_ed25519_signer(1)?;
    let owner = account_signer.address();

    let wallet = LocalWallet::new(account_signer);
    let provider = provider.with_wallet_filler(WalletFiller::new(wallet));

    let res = provider.claim_faucet_with_cooldown_remaining().await?;
    println!("claim_faucet result: {res:?}");

    let res = provider.create_account().await?;
    println!("Created account tx_hash: {res}");
    time::sleep(Duration::from_secs(1)).await;

    let account = provider.account(owner).await?;
    println!("Created account = {account:?}");
    Ok(())
}

/*
create_multisig tx_hash: HgMc13BLmdZA8FKahLHRBYwqy9fgAExe1h5zBbo8HiXA
multisig list_signers = (Account { bitmap: Bitmap64(7), weight: 6, threshold: 3 },
[(FMUEmtxhU46GzhKF4FW9MLJdQWiLgjiXP9TYRWSrqTpV, 0, 1),
(6TcyBfPdBt1kjsvDZLzmBFnuMaLWiTaAt4RjUr9VA5YD, 1, 2),
(4MfyR4G3NWfVRDWo6iNAHDBZqWMgwZX6FNtMqEW3a9JT, 2, 3)])
 */
async fn create_multisig(rpc: &DemoRpc) -> Result<(), Box<dyn Error>> {
    let provider = &rpc.provider;
    let account_signer = local_ed25519_signer(2)?;
    let pk1_signer = local_ed25519_signer(21)?;
    let pk2_signer = local_ed25519_signer(22)?;
    let pk3_signer = local_ed25519_signer(23)?;
    let owner = account_signer.address();

    let wallet = LocalWallet::new(account_signer);
    let provider = provider.with_wallet_filler(WalletFiller::new(wallet));

    let res = provider.claim_faucet_with_cooldown_remaining().await?;
    println!("claim_faucet result: {res:?}");

    let pks = vec![
        pk1_signer.public_key().clone(),
        pk2_signer.public_key().clone(),
        pk3_signer.public_key().clone(),
    ];
    let res = provider.create_multisig(pks, vec![1, 2, 3], 3).await?;
    println!("create_multisig tx_hash: {res}");
    time::sleep(Duration::from_secs(1)).await;

    let signers = provider.list_signers(owner).await?;
    println!("multisig list_signers = {signers:?}");
    Ok(())
}

/*
add_signer tx_hash: FJHbUCw5p3GJzepNRyyuRB3CP9xVQzT9goA1woomi6ZS
add_signer list_signers = (Account { bitmap: Bitmap64(3), weight: 2, threshold: 1 },
[(AKnL4NNf3DGWZJS6cPknBuEGnVsV4A4m5tgebLHaRSZ9, 0, 1), (7v54NWdBtkjuAFJrLGsS2SXnuk8nKam81mZJeeYxVFi9, 1, 1)])
*/
async fn add_signer(rpc: &DemoRpc) -> Result<(), Box<dyn Error>> {
    let provider = &rpc.provider;
    let account_signer = local_ed25519_signer(1)?;
    let pk1_signer = local_ed25519_signer(11)?;
    let owner = account_signer.address();

    let wallet = LocalWallet::new(account_signer);
    let provider = provider.with_wallet_filler(WalletFiller::new(wallet));

    let res = provider.claim_faucet_with_cooldown_remaining().await?;
    println!("claim_faucet result: {res:?}");

    let res = provider
        .add_signer(pk1_signer.public_key().clone(), 1u8)
        .await?;
    println!("add_signer tx_hash: {res}");
    time::sleep(Duration::from_secs(1)).await;

    let signers = provider.list_signers(owner).await?;
    println!("add_signer list_signers = {signers:?}");
    Ok(())
}

/*
add_signers tx_hash: 5XNMXrSRfquF1Aiq1rKVNS5uM1rc3hExGhYJPZVdzAqH
add_signers list_signers = (Account { bitmap: Bitmap64(15), weight: 7, threshold: 2 },
[(AKnL4NNf3DGWZJS6cPknBuEGnVsV4A4m5tgebLHaRSZ9, 0, 1), (7v54NWdBtkjuAFJrLGsS2SXnuk8nKam81mZJeeYxVFi9, 1, 1),
(mBKqcnGotbsSb5vNrdyhzZ5EhqZdids9QYiTRckvi7v, 2, 2), (AoVsGaj8MSJ6xwKxfFxo9iZWH3enC8RRTXKH2fx2F8os, 3, 3)])
 */
async fn add_signers(rpc: &DemoRpc) -> Result<(), Box<dyn Error>> {
    let provider = &rpc.provider;
    let account_signer = local_ed25519_signer(1)?;
    let pk2_signer = local_ed25519_signer(12)?;
    let pk3_signer = local_ed25519_signer(13)?;
    let owner = account_signer.address();

    let wallet = LocalWallet::new(account_signer);
    let provider = provider.with_wallet_filler(WalletFiller::new(wallet));

    let res = provider.claim_faucet_with_cooldown_remaining().await?;
    println!("claim_faucet result: {res:?}");

    let pks = vec![
        pk2_signer.public_key().clone(),
        pk3_signer.public_key().clone(),
    ];
    let res = provider.add_signers(pks, vec![2, 3], 2).await?;
    println!("add_signers tx_hash: {res}");
    time::sleep(Duration::from_secs(1)).await;

    let signers = provider.list_signers(owner).await?;
    println!("add_signers list_signers = {signers:?}");
    Ok(())
}

/*
before set_threshold list_signers = (Account { bitmap: Bitmap64(15), weight: 7, threshold: 2 },
[(AKnL4NNf3DGWZJS6cPknBuEGnVsV4A4m5tgebLHaRSZ9, 0, 1), (7v54NWdBtkjuAFJrLGsS2SXnuk8nKam81mZJeeYxVFi9, 1, 1),
(mBKqcnGotbsSb5vNrdyhzZ5EhqZdids9QYiTRckvi7v, 2, 2), (AoVsGaj8MSJ6xwKxfFxo9iZWH3enC8RRTXKH2fx2F8os, 3, 3)])
*/
async fn set_threshold(rpc: &DemoRpc) -> Result<(), Box<dyn Error>> {
    let provider = &rpc.provider;
    let account_signer = local_ed25519_signer(1)?;
    let pk2_signer = local_ed25519_signer(12)?;

    let owner = account_signer.address();

    let mut wallet = LocalWallet::new(account_signer);
    wallet.register_signer(pk2_signer)?;
    let provider = provider.with_wallet_filler(WalletFiller::new(wallet));

    let res = provider.claim_faucet_with_cooldown_remaining().await?;
    println!("claim_faucet result: {res:?}");

    let res = provider.set_threshold(5).await?;
    println!("set_threshold tx_hash: {res}");
    time::sleep(Duration::from_secs(1)).await;

    let signers = provider.list_signers(owner).await?;
    println!("set_threshold list_signers = {signers:?}");
    Ok(())
}

// 未跑通
async fn remove_signer(rpc: &DemoRpc) -> Result<(), Box<dyn Error>> {
    let provider = &rpc.provider;
    let account_signer = local_ed25519_signer(1)?;
    // let pk1_signer = local_ed25519_signer(11)?;
    // let pk1 = pk1_signer.public_key().clone();
    let owner = account_signer.address();

    let wallet = LocalWallet::new(account_signer);
    let provider = provider.with_wallet_filler(WalletFiller::new(wallet));

    let res = provider.claim_faucet_with_cooldown_remaining().await?;
    println!("claim_faucet result: {res:?}");

    let res = provider.remove_signer(1, 1).await?;
    println!("remove_signer tx_hash: {res}");
    time::sleep(Duration::from_secs(1)).await;

    let signers = provider.list_signers(owner).await?;
    println!("remove_signer list_signers = {signers:?}");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rpc = DemoRpc::connect(DEFAULT_HTTP_RPC_URL)?;

    // create_account(&rpc).await?;
    // create_multisig(&rpc).await?;
    // add_signer(&rpc).await?;
    // add_signers(&rpc).await?;
    set_threshold(&rpc).await?;

    // 未跑通
    // remove_signer(&rpc).await?;

    Ok(())
}
async fn transfer_from_with_ixs(rpc: &DemoRpc) -> Result<(), Box<dyn Error>> {
    let provider = &rpc.provider;
    let token = local_ed25519_signer(b'E')?;
    let token_addr = token.address();
    println!(">>>>>>token(EGL) address: {token_addr}");

    let holder_signer = local_ed25519_signer(1)?;
    let spender_signer = local_ed25519_signer(2)?;

    let holder = holder_signer.address();
    let spender = spender_signer.address();

    // balance
    let view_methods = vec![
        token::BalanceOf {
            token: token_addr,
            account: holder,
        },
        token::BalanceOf {
            token: token_addr,
            account: spender,
        },
    ];
    let balance_of = provider.multicall2(view_methods.clone()).await?;
    println!(">>>>>> before balance-of: {balance_of:?}");

    let instructions = vec![
        token::Approve {
            owner: ChainSigner::new(holder),
            token: token_addr,
            spender,
            amount: 202000000,
        }
        .pack()?,
        token::TransferFrom {
            spender: ChainSigner::new(spender),
            token: token_addr,
            from: holder,
            amount: 202000000,
        }
        .pack()?,
    ];

    let mut wallet = LocalWallet::new(holder_signer);
    wallet.register_signer(spender_signer)?;

    let plan = SigningPlan::new(holder)
        .authorize(AccountAuthorization::new(holder, vec![0]).with_payer())
        .authorize(AccountAuthorization::new(spender, vec![1]));
    let request = TransactionRequest::new(instructions)?.with_signing_plan(plan);

    let provider = provider.with_wallet_filler(WalletFiller::new(wallet));
    let tx_hash = provider.send_transaction(request).await?;
    println!(">>>>>>transfer_from_with_ixs>>>>>>>submit tx_hash: {tx_hash}");

    let raw: Vec<u8> = wait_for_get_txn(&provider, tx_hash).await?;

    let balance_of = provider.multicall2(view_methods).await?;
    println!(">>>>>> after balance-of: {balance_of:?}");

    let history = sdk::decode_transaction_history(&raw)?;
    print_transaction_history(&history);
    Ok(())
}

fn metadata() -> token::Metadata {
    token::Metadata {
        name: "Egal".to_owned(),
        symbol: "EGL".to_owned(),
        decimals: 9,
        icon: "https://milon.com/egl_icon.png".to_owned(),
    }
}
