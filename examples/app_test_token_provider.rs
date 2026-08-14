use infra_core::map_err_logged;
use milon_client::{self as sdk, TokenProviderExt, WalletFiller, token};
use milon_crypto::Address;
use milon_idl_core::{Method, Signer as ChainSigner};
use milon_local_wallet::{LocalWallet, Signer};
use milon_primitives::{AccountAuthorization, SigningPlan};
use milon_provider::{IdlProviderExt, TransactionRequest};
use only_sdk_examples::{
    DemoRpc, decode_print::print_transaction_history, errors::ExmErr, init, local_ed25519_signer,
    wait_for_get_txn,
};
use std::{error::Error, time::Duration};
use tokio::{time, time::timeout};

// const DEFAULT_HTTP_RPC_URL: &str = "http://127.0.0.1:6280/milon/v1";
const DEFAULT_HTTP_RPC_URL: &str = "http://47.84.39.153:6280/milon/v1";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _gard = init()?;
    let rpc = DemoRpc::connect(DEFAULT_HTTP_RPC_URL)?;

    // balance_of(&rpc).await?;
    // create_token(&rpc).await?;
    // get_metadata(&rpc).await?;
    mint(&rpc).await?;
    // burn(&rpc).await?;
    // transfer(&rpc).await?;

    // freeze(&rpc).await?;

    // unfreeze(&rpc).await?;

    // approve_and_revoke(&rpc).await?;

    // transfer_from(&rpc).await?;

    // transfer_from_with_ixs(&rpc).await?;

    Ok(())
}

async fn get_metadata(rpc: &DemoRpc) -> Result<(), Box<dyn Error>> {
    let provider = rpc.provider.clone();

    let token = local_ed25519_signer(b'E')?;
    let token_addr = token.address();

    let metadata = timeout(Duration::from_secs(10), provider.metadata(token_addr))
        .await
        .map_err(map_err_logged!(ExmErr::MetadataErr))??;
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
        .await
        .map_err(map_err_logged!(ExmErr::BalanceOfErr))?;
    println!("balance: {}", balance);
    Ok(())
}

async fn create_token(rpc: &DemoRpc) -> Result<(), Box<dyn Error>> {
    let token = local_ed25519_signer(b'T')?;
    let token_addr = token.address();
    let wallet = LocalWallet::new(token);

    let provider = &rpc.provider;
    let provider = provider.with_wallet_filler(WalletFiller::new(wallet));

    // claim_faucet
    let hash = provider
        .claim_faucet()
        .await
        .map_err(map_err_logged!(ExmErr::ClaimFaucetErr))?;
    println!("claim_faucet tx_hash: {hash}");
    time::sleep(Duration::from_secs(3)).await;

    let account_signer = local_ed25519_signer(1)?;
    let owner = account_signer.address();

    let res = provider
        .create_token(owner, metadata())
        .await
        .map_err(map_err_logged!(ExmErr::CreateTokenErr))?;
    println!("Created token response: {}", res);
    time::sleep(Duration::from_secs(3)).await;

    let metadata = timeout(Duration::from_secs(10), provider.metadata(token_addr))
        .await
        .map_err(map_err_logged!(ExmErr::BalanceOfErr))??;
    println!("metadata = {metadata:?}");
    Ok(())
}

async fn mint(rpc: &DemoRpc) -> Result<(), Box<dyn Error>> {
    let token = local_ed25519_signer(b'T')?;
    let token_addr = token.address();
    tracing::info!(">>>>>>token(EGL) address: {token_addr}");

    let owner_signer = local_ed25519_signer(1)?;
    let signer_2 = local_ed25519_signer(2)?;
    let signer_3 = local_ed25519_signer(3)?;
    let owner = owner_signer.address();
    // wallet注册的signers，都会对 ix=0 进行签名，包括default-signer
    let mut wallet = LocalWallet::new(owner_signer);
    wallet.register_signer(signer_2)?;
    wallet.register_signer(signer_3)?;

    let provider = &rpc.provider;
    let provider = provider.with_wallet_filler(WalletFiller::new(wallet));
    let res = provider
        .claim_faucet_with_cooldown_remaining()
        .await
        .map_err(map_err_logged!(ExmErr::ClaimFaucetWithCooldownRemaining))?;
    tracing::info!("claim_faucet_with_cooldown_remaining tx_hash: {res:?}");
    if res.is_some() {
        time::sleep(Duration::from_secs(1)).await;
    }

    // let owner = Address::from_bs58("pFjhQSFxva13nsrMmXrLZRJDkMK").unwrap();
    // 这里的 owner_signer、signer_2 和 signer_3 都会对 `token::mint` 指令签名
    let res = provider
        .mint(token_addr, owner, 9999000000)
        .await
        .map_err(map_err_logged!(ExmErr::TokenMintErr))?;
    tracing::info!(">>>>>>Mint token tx_hash: {res}");
    time::sleep(Duration::from_secs(3)).await;

    let balance = provider
        .balance_of(token_addr, owner)
        .await
        .map_err(map_err_logged!(ExmErr::BalanceOfErr))?;
    tracing::info!(">>>>>>balance_of = {balance}");
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
    .await
    .map_err(map_err_logged!(ExmErr::BalanceOfErr))??;
    println!(">>>>>>balance_of = {balance}");

    // wallet-provider
    let provider = provider.with_wallet_filler(WalletFiller::new(holder_wallet));

    let res = provider
        .burn(token_addr, 99000000)
        .await
        .map_err(map_err_logged!(ExmErr::BurnErr))?;
    println!(">>>>>>burn token tx_hash: {res}");
    time::sleep(Duration::from_secs(3)).await;

    let balance = timeout(
        Duration::from_secs(10),
        provider.balance_of(token_addr, holder),
    )
    .await
    .map_err(map_err_logged!(ExmErr::BalanceOfErr))??;
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
    let balance = provider
        .balance_of(token_addr, holder)
        .await
        .map_err(map_err_logged!(ExmErr::BalanceOfErr))?;
    println!(">>>>>>holder before balance_of = {balance}");

    // wallet-provider
    let provider = provider.with_wallet_filler(WalletFiller::new(holder_wallet));
    // let to = Address::from_bs58("JCSS25kd9r7ipG1xXeA4txEqT7m")?;
    let res = provider
        .claim_faucet_with_cooldown_remaining()
        .await
        .map_err(map_err_logged!(ExmErr::ClaimFaucetWithCooldownRemaining))?;
    time::sleep(Duration::from_secs(1)).await;

    let to_signer = local_ed25519_signer(2)?;
    let to = to_signer.address();

    let res = provider
        .transfer(token_addr, to, 10023000000)
        .await
        .map_err(map_err_logged!(ExmErr::TransferErr))?;
    println!(">>>>>>transfer token tx_hash: {res}");
    time::sleep(Duration::from_secs(3)).await;

    let balance = provider
        .balance_of(token_addr, holder)
        .await
        .map_err(map_err_logged!(ExmErr::BalanceOfErr))?;
    println!(">>>>>>holder transfer balance_of = {balance}");
    let balance = provider
        .balance_of(token_addr, to)
        .await
        .map_err(map_err_logged!(ExmErr::BalanceOfErr))?;
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
    let balance = provider
        .balance_of(token_addr, holder)
        .await
        .map_err(map_err_logged!(ExmErr::BalanceOfErr))?;
    println!(">>>>>>holder[{}] balance_of = {balance}", holder.to_bs58());

    let res = provider
        .freeze(token_addr, holder, 23000000)
        .await
        .map_err(map_err_logged!(ExmErr::FreezeErr))?;
    println!(">>>>>>freeze token tx_hash: {res}");
    time::sleep(Duration::from_secs(3)).await;

    let balance = provider
        .balance_of(token_addr, holder)
        .await
        .map_err(map_err_logged!(ExmErr::BalanceOfErr))?;
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
    let balance = provider
        .balance_of(token_addr, holder)
        .await
        .map_err(map_err_logged!(ExmErr::BalanceOfErr))?;
    println!(">>>>>>holder[{}] balance_of = {balance}", holder.to_bs58());

    let res = provider
        .unfreeze(token_addr, holder, 1000000)
        .await
        .map_err(map_err_logged!(ExmErr::UnfreezeErr))?;
    println!(">>>>>>unfreeze token tx_hash: {res}");
    time::sleep(Duration::from_secs(3)).await;

    let balance = provider
        .balance_of(token_addr, holder)
        .await
        .map_err(map_err_logged!(ExmErr::BalanceOfErr))?;
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

    let balance = provider
        .balance_of(token_addr, owner)
        .await
        .map_err(map_err_logged!(ExmErr::BalanceOfErr))?;
    println!(">>>>>>owner[{}] balance_of = {balance}", owner.to_bs58());

    let wallet = LocalWallet::new(holder);
    let provider = provider.with_wallet_filler(WalletFiller::new(wallet));

    let spender = Address::from_bs58("JCSS25kd9r7ipG1xXeA4txEqT7m")?;
    let approval = provider
        .approval_of(token_addr, owner, spender)
        .await
        .map_err(map_err_logged!(ExmErr::ApprovalOfErr));
    println!(">>>>>>approve before approval_of = {approval:?}",);

    let res = provider
        .approve(token_addr, spender, 1000000)
        .await
        .map_err(map_err_logged!(ExmErr::ApproveErr))?;
    println!(">>>>>>approve token tx_hash: {res}");
    time::sleep(Duration::from_secs(3)).await;

    let approval = provider
        .approval_of(token_addr, owner, spender)
        .await
        .map_err(map_err_logged!(ExmErr::ApprovalOfErr))?;
    println!(">>>>>>approve after approval_of = {approval}",);
    let balance = provider
        .balance_of(token_addr, owner)
        .await
        .map_err(map_err_logged!(ExmErr::BalanceOfErr))?;
    println!(">>>>>>owner[{}] balance_of = {balance}", owner.to_bs58());

    let res = provider
        .revoke(token_addr, spender)
        .await
        .map_err(map_err_logged!(ExmErr::RevokeErr))?;
    println!(">>>>>>revoke token tx_hash: {res}");
    time::sleep(Duration::from_secs(3)).await;

    let approval = provider
        .approval_of(token_addr, owner, spender)
        .await
        .map_err(map_err_logged!(ExmErr::ApprovalOfErr));
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

    let balance = provider
        .balance_of(token_addr, owner)
        .await
        .map_err(map_err_logged!(ExmErr::BalanceOfErr))?;
    println!(">>>>>>owner[{}] balance_of = {balance}", owner.to_bs58());

    let holder_wallet = LocalWallet::new(holder);
    let holder_provider = provider.with_wallet_filler(WalletFiller::new(holder_wallet));

    let spender_signer = local_ed25519_signer(2)?;
    let spender = spender_signer.address();
    let balance = provider
        .balance_of(token_addr, spender)
        .await
        .map_err(map_err_logged!(ExmErr::BalanceOfErr))?;
    println!(
        ">>>>>>spender[{}] balance_of = {balance}",
        spender.to_bs58()
    );

    let res = holder_provider
        .approve(token_addr, spender, 101000000)
        .await
        .map_err(map_err_logged!(ExmErr::ApproveErr))?;
    println!(">>>>>>approve token tx_hash: {res}");
    time::sleep(Duration::from_secs(2)).await;

    let approval = provider
        .approval_of(token_addr, owner, spender)
        .await
        .map_err(map_err_logged!(ExmErr::ApprovalOfErr))?;
    println!(">>>>>>approve after approval_of = {approval}",);

    let spender_wallet = LocalWallet::new(spender_signer);
    let spender_provider = provider.with_wallet_filler(WalletFiller::new(spender_wallet));
    spender_provider
        .claim_faucet()
        .await
        .map_err(map_err_logged!(ExmErr::ClaimFaucetErr))?;
    time::sleep(Duration::from_secs(2)).await;

    let res = spender_provider
        .transfer_from(token_addr, owner, 100000000)
        .await
        .map_err(map_err_logged!(ExmErr::TransferFromErr))?;
    println!(">>>>>>transfer_from token tx_hash: {res}");
    time::sleep(Duration::from_secs(2)).await;

    let approval = provider
        .approval_of(token_addr, owner, spender)
        .await
        .map_err(map_err_logged!(ExmErr::ApprovalOfErr))?;
    println!(">>>>>>transfer_from after approval_of = {approval}",);

    let balance = provider
        .balance_of(token_addr, spender)
        .await
        .map_err(map_err_logged!(ExmErr::BalanceOfErr))?;
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
    let balance_of = provider
        .typed_multicall(view_methods.clone())
        .await
        .map_err(map_err_logged!(ExmErr::Multicall2Err))?;
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
    let tx_hash = provider
        .send_transaction(request)
        .await
        .map_err(map_err_logged!(ExmErr::SendTransactionErr))?;
    println!(">>>>>>transfer_from_with_ixs>>>>>>>submit tx_hash: {tx_hash}");

    let raw: Vec<u8> = wait_for_get_txn(&provider, tx_hash).await?;

    let balance_of = provider
        .typed_multicall(view_methods)
        .await
        .map_err(map_err_logged!(ExmErr::Multicall2Err))?;
    println!(">>>>>> after balance-of: {balance_of:?}");

    let history = sdk::decode_transaction_history(&raw)?;
    print_transaction_history(&history);
    Ok(())
}

fn metadata() -> token::Metadata {
    token::Metadata {
        name: "Test".to_owned(),
        symbol: "Test".to_owned(),
        decimals: 6,
        icon: "https://milon.com/test_icon.png".to_owned(),
    }
}
