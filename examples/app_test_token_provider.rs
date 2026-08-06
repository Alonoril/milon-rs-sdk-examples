use milon_client::{self as sdk, TokenProviderExt, WalletFiller, token};
use milon_crypto::Address;
use milon_idl_core::{Method, Signer as ChainSigner};
use milon_local_wallet::{LocalWallet, Signer};
use milon_primitives::{AccountAuthorization, SigningPlan};
use milon_provider::{IdlProviderExt, TransactionRequest};
use only_sdk_examples::{
    DemoRpc, decode_print::print_transaction_history, local_ed25519_signer, wait_for_get_txn,
};
use std::{error::Error, time::Duration};
use tokio::{time, time::timeout};

// const DEFAULT_HTTP_RPC_URL: &str = "http://127.0.0.1:6280/milon/v1";
const DEFAULT_HTTP_RPC_URL: &str = "http://47.84.39.153:6280/milon/v1";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rpc = DemoRpc::connect(DEFAULT_HTTP_RPC_URL)?;

    // balance_of(&rpc).await?;
    // create_token(&rpc).await?;
    // get_metadata(&rpc).await?;
    // mint(&rpc).await?;
    // burn(&rpc).await?;
    // transfer(&rpc).await?;

    // freeze(&rpc).await?;

    // unfreeze(&rpc).await?;

    // approve_and_revoke(&rpc).await?;

    transfer_from(&rpc).await?;

    // transfer_from_with_ixs(&rpc).await?;

    Ok(())
}

async fn get_metadata(rpc: &DemoRpc) -> Result<(), Box<dyn Error>> {
    let provider = rpc.provider.clone();

    let token = local_ed25519_signer(b'E')?;
    let token_addr = token.address();

    let metadata = timeout(Duration::from_secs(10), provider.metadata(token_addr)).await??;
    println!("metadata = {metadata:?}");
    Ok(())
}

async fn balance_of(rpc: &DemoRpc) -> Result<(), Box<dyn Error>> {
    let token = local_ed25519_signer(b'E')?;
    let token_addr = token.address();

    let account_signer = local_ed25519_signer(2)?;

    let balance = rpc
        .provider
        .balance_of(token_addr, account_signer.address())
        .await?;
    println!("balance: {}", balance);
    Ok(())
}

async fn create_token(rpc: &DemoRpc) -> Result<(), Box<dyn Error>> {
    let token = local_ed25519_signer(b'E')?;
    let token_addr = token.address();
    let wallet = LocalWallet::new(token);

    let provider = &rpc.provider;
    let provider = provider.with_wallet_filler(WalletFiller::new(wallet));

    // claim_faucet
    let hash = provider.claim_faucet().await?;
    println!("claim_faucet tx_hash: {hash}");
    time::sleep(Duration::from_secs(3)).await;

    let account_signer = local_ed25519_signer(1)?;
    let owner = account_signer.address();

    let res = provider.create_token(owner, metadata()).await?;
    println!("Created token response: {}", res);
    time::sleep(Duration::from_secs(3)).await;

    let metadata = timeout(Duration::from_secs(10), provider.metadata(token_addr)).await??;
    println!("metadata = {metadata:?}");
    Ok(())
}

async fn mint(rpc: &DemoRpc) -> Result<(), Box<dyn Error>> {
    let token = local_ed25519_signer(b'E')?;
    let token_addr = token.address();
    println!(">>>>>>token(EGL) address: {token_addr}");

    let account_signer = local_ed25519_signer(1)?;
    let owner = account_signer.address();
    let wallet = LocalWallet::new(account_signer);

    let provider = &rpc.provider;
    let provider = provider.with_wallet_filler(WalletFiller::new(wallet));
    let res = provider.claim_faucet_with_cooldown_remaining().await?;
    println!(">>>claim_faucet_with_cooldown_remaining tx_hash: {res:?}");
    time::sleep(Duration::from_secs(1)).await;

    let res = provider.mint(token_addr, owner, 999999999000000).await?;
    println!(">>>>>>Mint token tx_hash: {res}");
    time::sleep(Duration::from_secs(3)).await;

    let balance = timeout(
        Duration::from_secs(10),
        provider.balance_of(token_addr, owner),
    )
    .await??;
    println!(">>>>>>balance_of = {balance}");
    Ok(())
}

async fn burn(rpc: &DemoRpc) -> Result<(), Box<dyn Error>> {
    let token = local_ed25519_signer(b'E')?;
    let token_addr = token.address();

    let account_signer = local_ed25519_signer(1)?;
    let holder = account_signer.address();
    let holder_wallet = LocalWallet::new(account_signer);

    let provider = &rpc.provider;
    let balance = timeout(
        Duration::from_secs(10),
        provider.balance_of(token_addr, holder),
    )
    .await??;
    println!(">>>>>>balance_of = {balance}");

    // wallet-provider
    let provider = provider.with_wallet_filler(WalletFiller::new(holder_wallet));

    let res = provider.burn(token_addr, 99000000).await?;
    println!(">>>>>>burn token tx_hash: {res}");
    time::sleep(Duration::from_secs(3)).await;

    let balance = timeout(
        Duration::from_secs(10),
        provider.balance_of(token_addr, holder),
    )
    .await??;
    println!(">>>>>>balance_of = {balance}");
    Ok(())
}

// transfer
async fn transfer(rpc: &DemoRpc) -> Result<(), Box<dyn Error>> {
    let token = local_ed25519_signer(b'E')?;
    let token_addr = token.address();

    let account_signer = local_ed25519_signer(1)?;
    let holder = account_signer.address();
    let holder_wallet = LocalWallet::new(account_signer);

    let provider = &rpc.provider;
    let balance = provider.balance_of(token_addr, holder).await?;
    println!(">>>>>>holder before balance_of = {balance}");

    // wallet-provider
    let provider = provider.with_wallet_filler(WalletFiller::new(holder_wallet));
    // let to = Address::from_bs58("JCSS25kd9r7ipG1xXeA4txEqT7m")?;
    let res = provider.claim_faucet_with_cooldown_remaining().await?;
    time::sleep(Duration::from_secs(1)).await;

    let to_signer = local_ed25519_signer(2)?;
    let to = to_signer.address();

    let res = provider.transfer(token_addr, to, 10023000000).await?;
    println!(">>>>>>transfer token tx_hash: {res}");
    time::sleep(Duration::from_secs(3)).await;

    let balance = provider.balance_of(token_addr, holder).await?;
    println!(">>>>>>holder transfer balance_of = {balance}");
    let balance = provider.balance_of(token_addr, to).await?;
    println!(">>>>>>to[{}] balance_of = {balance}", to.to_bs58());
    Ok(())
}

async fn freeze(rpc: &DemoRpc) -> Result<(), Box<dyn Error>> {
    let token = local_ed25519_signer(b'E')?;
    let token_addr = token.address();
    println!(">>>>>>token(EGL) address: {token_addr}");

    let account_signer = local_ed25519_signer(1)?;
    let wallet = LocalWallet::new(account_signer);

    let provider = &rpc.provider;
    let provider = provider.with_wallet_filler(WalletFiller::new(wallet));

    let holder = Address::from_bs58("JCSS25kd9r7ipG1xXeA4txEqT7m")?;
    let balance = provider.balance_of(token_addr, holder).await?;
    println!(">>>>>>holder[{}] balance_of = {balance}", holder.to_bs58());

    let res = provider.freeze(token_addr, holder, 23000000).await?;
    println!(">>>>>>freeze token tx_hash: {res}");
    time::sleep(Duration::from_secs(3)).await;

    let balance = provider.balance_of(token_addr, holder).await?;
    println!(">>>>>>holder[{}] balance_of = {balance}", holder.to_bs58());
    Ok(())
}

async fn unfreeze(rpc: &DemoRpc) -> Result<(), Box<dyn Error>> {
    let token = local_ed25519_signer(b'E')?;
    let token_addr = token.address();
    println!(">>>>>>token(EGL) address: {token_addr}");

    let account_signer = local_ed25519_signer(1)?;
    let wallet = LocalWallet::new(account_signer);

    let provider = &rpc.provider;
    let provider = provider.with_wallet_filler(WalletFiller::new(wallet));

    let holder = Address::from_bs58("JCSS25kd9r7ipG1xXeA4txEqT7m")?;
    let balance = provider.balance_of(token_addr, holder).await?;
    println!(">>>>>>holder[{}] balance_of = {balance}", holder.to_bs58());

    let res = provider.unfreeze(token_addr, holder, 1000000).await?;
    println!(">>>>>>unfreeze token tx_hash: {res}");
    time::sleep(Duration::from_secs(3)).await;

    let balance = provider.balance_of(token_addr, holder).await?;
    println!(">>>>>>holder[{}] balance_of = {balance}", holder.to_bs58());
    Ok(())
}

async fn approve_and_revoke(rpc: &DemoRpc) -> Result<(), Box<dyn Error>> {
    let token = local_ed25519_signer(b'E')?;
    let token_addr = token.address();
    println!(">>>>>>token(EGL) address: {token_addr}");

    let provider = &rpc.provider;
    let holder = local_ed25519_signer(1)?;
    let owner = holder.address();

    let balance = provider.balance_of(token_addr, owner).await?;
    println!(">>>>>>owner[{}] balance_of = {balance}", owner.to_bs58());

    let wallet = LocalWallet::new(holder);
    let provider = provider.with_wallet_filler(WalletFiller::new(wallet));

    let spender = Address::from_bs58("JCSS25kd9r7ipG1xXeA4txEqT7m")?;
    let approval = provider.approval_of(token_addr, owner, spender).await;
    println!(">>>>>>approve before approval_of = {approval:?}",);

    let res = provider.approve(token_addr, spender, 1000000).await?;
    println!(">>>>>>approve token tx_hash: {res}");
    time::sleep(Duration::from_secs(3)).await;

    let approval = provider.approval_of(token_addr, owner, spender).await?;
    println!(">>>>>>approve after approval_of = {approval}",);
    let balance = provider.balance_of(token_addr, owner).await?;
    println!(">>>>>>owner[{}] balance_of = {balance}", owner.to_bs58());

    let res = provider.revoke(token_addr, spender).await?;
    println!(">>>>>>revoke token tx_hash: {res}");
    time::sleep(Duration::from_secs(3)).await;

    let approval = provider.approval_of(token_addr, owner, spender).await;
    println!(">>>>>>revoke after approval_of = {approval:?}",);

    Ok(())
}

async fn transfer_from(rpc: &DemoRpc) -> Result<(), Box<dyn Error>> {
    let token = local_ed25519_signer(b'E')?;
    let token_addr = token.address();
    println!(">>>>>>token(EGL) address: {token_addr}");

    let provider = &rpc.provider;
    let holder = local_ed25519_signer(1)?;
    let owner = holder.address();

    let balance = provider.balance_of(token_addr, owner).await?;
    println!(">>>>>>owner[{}] balance_of = {balance}", owner.to_bs58());

    let holder_wallet = LocalWallet::new(holder);
    let holder_provider = provider
        .clone()
        .with_wallet_filler(WalletFiller::new(holder_wallet));

    let spender_signer = local_ed25519_signer(2)?;
    let spender = spender_signer.address();
    let balance = provider.balance_of(token_addr, spender).await?;
    println!(
        ">>>>>>spender[{}] balance_of = {balance}",
        spender.to_bs58()
    );

    let res = holder_provider
        .approve(token_addr, spender, 101000000)
        .await?;
    println!(">>>>>>approve token tx_hash: {res}");
    time::sleep(Duration::from_secs(2)).await;

    let approval = provider.approval_of(token_addr, owner, spender).await?;
    println!(">>>>>>approve after approval_of = {approval}",);

    let spender_wallet = LocalWallet::new(spender_signer);
    let spender_provider = provider.with_wallet_filler(WalletFiller::new(spender_wallet));
    spender_provider.claim_faucet().await?;
    time::sleep(Duration::from_secs(2)).await;

    let res = spender_provider
        .transfer_from(token_addr, owner, 100000000)
        .await?;
    println!(">>>>>>transfer_from token tx_hash: {res}");
    time::sleep(Duration::from_secs(2)).await;

    let approval = provider.approval_of(token_addr, owner, spender).await?;
    println!(">>>>>>transfer_from after approval_of = {approval}",);

    let balance = provider.balance_of(token_addr, spender).await?;
    println!(
        ">>>>>>spender[{}] transfer_from after balance_of = {balance}",
        spender.to_bs58()
    );

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
