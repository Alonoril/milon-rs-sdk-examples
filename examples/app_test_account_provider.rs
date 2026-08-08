use milon_client::{
    self as sdk, AccountProviderExt, TokenProviderExt, WalletFiller, account::VoteProposal, token,
};
use milon_crypto::hash32_hasher;
use milon_idl_core::{Bitmap64 as IdlBitmap64, Method, Signer as ChainSigner, WriteMethod};
use milon_local_wallet::{LocalWallet, MultisigSlot, Signer};
use milon_primitives::{AccountAuthorization, B256, ChainId, SigningPlan};
use milon_provider::{IdlProviderExt, TransactionRequest};
use only_sdk_examples::{
    DemoRpc, decode_print::print_transaction_history, local_ed25519_signer, wait_for_get_txn,
};
use std::{error::Error, time::Duration};
use tokio::time;

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
    let account_signer = local_ed25519_signer(3)?;
    let pk0 = account_signer.public_key().clone();

    let pk1_signer = local_ed25519_signer(31)?;
    let pk2_signer = local_ed25519_signer(32)?;
    let pk3_signer = local_ed25519_signer(33)?;
    let owner = account_signer.address();

    let wallet = LocalWallet::new(account_signer);
    let provider = provider.with_wallet_filler(WalletFiller::new(wallet));

    let res = provider.claim_faucet_with_cooldown_remaining().await?;
    println!("claim_faucet result: {res:?}");
    if res.is_some() {
        time::sleep(Duration::from_secs(1)).await;
    }

    let pks = vec![
        pk0,
        pk1_signer.public_key().clone(),
        pk2_signer.public_key().clone(),
        pk3_signer.public_key().clone(),
    ];
    let res = provider.create_multisig(pks, vec![2, 1, 2, 3], 3).await?;
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

    let slots = vec![
        MultisigSlot::new(0, account_signer),
        MultisigSlot::with_weight(2, 2, pk2_signer),
    ];

    // set_threshold(5) 是由当前 threshold 2 授权的，所以本地多签定义必须先使用 2。交易成功后，
    // 后续交易需要重新按链上 threshold 5 创建钱包，并准备足够权重的本地 signer。
    //
    // 如果 index 或 weight 不是固定值，应以 list_signers(owner) 返回结果为准，不能只按 signer 注册顺序推断。
    let wallet = LocalWallet::new_multisig(owner, slots, 1)?; // 当前链上 threshold，不是即将设置的 5
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

/*
before remove_signer list_signers = (Account { bitmap: Bitmap64(15), weight: 7, threshold: 5 },
[(AKnL4NNf3DGWZJS6cPknBuEGnVsV4A4m5tgebLHaRSZ9, 0, 1), (7v54NWdBtkjuAFJrLGsS2SXnuk8nKam81mZJeeYxVFi9, 1, 1),
(mBKqcnGotbsSb5vNrdyhzZ5EhqZdids9QYiTRckvi7v, 2, 2), (AoVsGaj8MSJ6xwKxfFxo9iZWH3enC8RRTXKH2fx2F8os, 3, 3)])

remove_signer tx_hash: D8gfx9jCE4UCnmd8Ub9JeynyzaYmez5iKu9grzQhT1pn
remove_signer list_signers = (Account { bitmap: Bitmap64(13), weight: 6, threshold: 1 },
[(AKnL4NNf3DGWZJS6cPknBuEGnVsV4A4m5tgebLHaRSZ9, 0, 1), (mBKqcnGotbsSb5vNrdyhzZ5EhqZdids9QYiTRckvi7v, 2, 2),
(AoVsGaj8MSJ6xwKxfFxo9iZWH3enC8RRTXKH2fx2F8os, 3, 3)])
 */
async fn remove_signer(rpc: &DemoRpc) -> Result<(), Box<dyn Error>> {
    let provider = &rpc.provider;
    let account_signer = local_ed25519_signer(1)?;
    let pk2_signer = local_ed25519_signer(12)?;
    let pk3_signer = local_ed25519_signer(13)?;
    let owner = account_signer.address();

    let slots = vec![
        MultisigSlot::new(0, account_signer),
        MultisigSlot::with_weight(2, 2, pk2_signer),
        MultisigSlot::with_weight(3, 3, pk3_signer),
    ];

    let wallet = LocalWallet::new_multisig(owner, slots, 5)?;
    let provider = provider.with_wallet_filler(WalletFiller::new(wallet));

    let res = provider.claim_faucet_with_cooldown_remaining().await?;
    println!("claim_faucet result: {res:?}");
    if res.is_some() {
        time::sleep(Duration::from_secs(1)).await;
    }

    let res = provider.remove_signer(1, 1).await?;
    println!("remove_signer tx_hash: {res}");
    time::sleep(Duration::from_secs(1)).await;

    let signers = provider.list_signers(owner).await?;
    println!("remove_signer list_signers = {signers:?}");
    Ok(())
}

/*
set_signer_weight tx_hash: AkXwsM8mNE2qxiBVVQbqX9vcETcLYzRES9RsePpYtwYE
set_signer_weight list_signers = (Account { bitmap: Bitmap64(13), weight: 10, threshold: 1 },
[(AKnL4NNf3DGWZJS6cPknBuEGnVsV4A4m5tgebLHaRSZ9, 0, 5), (mBKqcnGotbsSb5vNrdyhzZ5EhqZdids9QYiTRckvi7v, 2, 2),
(AoVsGaj8MSJ6xwKxfFxo9iZWH3enC8RRTXKH2fx2F8os, 3, 3)])
 */
async fn set_signer_weight(rpc: &DemoRpc) -> Result<(), Box<dyn Error>> {
    let provider = &rpc.provider;
    let account_signer = local_ed25519_signer(1)?;
    let pk2_signer = local_ed25519_signer(12)?;
    let pk3_signer = local_ed25519_signer(13)?;
    let owner = account_signer.address();

    let slots = vec![
        MultisigSlot::new(0, account_signer),
        MultisigSlot::with_weight(2, 2, pk2_signer),
        MultisigSlot::with_weight(3, 3, pk3_signer),
    ];

    let wallet = LocalWallet::new_multisig(owner, slots, 5)?;
    let provider = provider.with_wallet_filler(WalletFiller::new(wallet));

    let res = provider.claim_faucet_with_cooldown_remaining().await?;
    println!("claim_faucet result: {res:?}");
    if res.is_some() {
        time::sleep(Duration::from_secs(1)).await;
    }

    let res = provider.set_signer_weight(0, 5).await?;
    println!("set_signer_weight tx_hash: {res}");
    time::sleep(Duration::from_secs(1)).await;

    let signers = provider.list_signers(owner).await?;
    println!("set_signer_weight list_signers = {signers:?}");
    Ok(())
}

async fn vote_init(
    rpc: &DemoRpc,
    wallet: &LocalWallet,
    proposal: VoteProposal,
    intent_hash: B256,
) -> Result<(), Box<dyn Error>> {
    let provider = &rpc.provider;
    // let account_signer = local_ed25519_signer(1)?;
    // let pk2_signer = local_ed25519_signer(12)?;
    // let pk3_signer = local_ed25519_signer(13)?;
    // let owner = account_signer.address();
    //
    // let slots = vec![
    //     MultisigSlot::new(0, account_signer),
    //     MultisigSlot::with_weight(2, 2, pk2_signer),
    //     MultisigSlot::with_weight(3, 3, pk3_signer),
    // ];
    //
    // let wallet = LocalWallet::new_multisig(owner, slots, 5)?;
    let (wallet, owner) = (wallet.clone(), wallet.default_account());
    let provider = provider.with_wallet_filler(WalletFiller::new(wallet));

    let res = provider.claim_faucet_with_cooldown_remaining().await?;
    println!("claim_faucet result: {res:?}");
    if res.is_some() {
        time::sleep(Duration::from_secs(1)).await;
    }

    let res = provider.vote_init(intent_hash, proposal).await?;
    println!("vote_init tx_hash: {res}");
    time::sleep(Duration::from_secs(1)).await;

    let vote_info = provider.get_vote(owner, intent_hash).await?;
    println!("vote_init vote_info = {vote_info:?}");

    Ok(())
}

async fn vote(
    rpc: &DemoRpc,
    wallet: &LocalWallet,
    intent_hash: B256,
) -> Result<(), Box<dyn Error>> {
    let provider = &rpc.provider;
    // let account_signer = local_ed25519_signer(1)?;
    // let pk2_signer = local_ed25519_signer(12)?;
    // let pk3_signer = local_ed25519_signer(13)?;
    // let owner = account_signer.address();
    //
    // let slots = vec![
    //     MultisigSlot::new(0, account_signer),
    //     MultisigSlot::with_weight(2, 2, pk2_signer),
    //     MultisigSlot::with_weight(3, 3, pk3_signer),
    // ];
    // let wallet = LocalWallet::new_multisig(owner, slots, 5)?;
    let (wallet, owner) = (wallet.clone(), wallet.default_account());
    let provider = provider.with_wallet_filler(WalletFiller::new(wallet));

    let res = provider.claim_faucet_with_cooldown_remaining().await?;
    println!("claim_faucet result: {res:?}");
    if res.is_some() {
        time::sleep(Duration::from_secs(1)).await;
    }

    let res = provider.vote(intent_hash).await?;
    println!("vote tx_hash: {res}");
    time::sleep(Duration::from_secs(1)).await;

    let vote_info = provider.get_vote(owner, intent_hash).await?;
    println!("vote vote_info = {vote_info:?}");
    Ok(())
}

enum VoteKind {
    Init,
    Vote,
    Info,
    Submit,
}

async fn multisig_on_chain(rpc: &DemoRpc, kind: VoteKind) -> Result<(), Box<dyn Error>> {
    let account_signer = local_ed25519_signer(1)?;
    let pk2_signer = local_ed25519_signer(12)?;
    let pk3_signer = local_ed25519_signer(13)?;
    let owner = account_signer.address();

    let slots = vec![
        MultisigSlot::new(0, account_signer),
        MultisigSlot::with_weight(2, 2, pk2_signer),
        MultisigSlot::with_weight(3, 3, pk3_signer),
    ];

    let wallet = LocalWallet::new_multisig(owner, slots, 5)?;

    let ix = sdk::account::SetSignerWeight {
        owner: ChainSigner::new(owner),
        index: 2,
        weight: 4,
    };
    let (_packed_ix, proposal, intent_hash) = build_vote_proposal(ix.clone())?;

    match kind {
        VoteKind::Init => vote_init(rpc, &wallet, proposal, intent_hash).await,
        VoteKind::Vote => vote(rpc, &wallet, intent_hash).await,
        VoteKind::Submit => submit_after_multisig_on_chain(rpc, &wallet, ix).await,
        VoteKind::Info => {
            let vote_info = rpc.provider.get_vote(owner, intent_hash).await?;
            println!("vote_init vote_info = {vote_info:?}");
            Ok(())
        },
    }
}

async fn submit_after_multisig_on_chain<M: WriteMethod + Send + 'static>(
    rpc: &DemoRpc,
    wallet: &LocalWallet,
    ix: M,
) -> Result<(), Box<dyn Error>> {
    let owner = wallet.default_account();
    let account_signer = local_ed25519_signer(1)?;
    let pk2_signer = local_ed25519_signer(12)?;
    let pk3_signer = local_ed25519_signer(13)?;
    let relayer_signer = local_ed25519_signer(2)?;
    let slots = vec![
        MultisigSlot::new(0, account_signer),
        MultisigSlot::with_weight(2, 2, pk2_signer),
        MultisigSlot::with_weight(3, 3, pk3_signer),
    ];
    let payer = relayer_signer.address();
    let mut payer_wallet = LocalWallet::new(relayer_signer);
    payer_wallet.register_multisig(owner, 5, slots)?;
    let provider = rpc
        .provider
        .with_wallet_filler(WalletFiller::new(payer_wallet));

    let res = provider.claim_faucet_with_cooldown_remaining().await?;
    println!("claim_faucet result: {res:?}");
    if res.is_some() {
        time::sleep(Duration::from_secs(1)).await;
    }

    let plan = SigningPlan::new(payer)
        .authorize(AccountAuthorization::new(owner, vec![0]))
        .authorize(AccountAuthorization::new(payer, Vec::new()).with_payer());
    let request = TransactionRequest::from_method(ix)?
        .with_payer(payer)
        .with_signing_plan(plan);
    let tx_hash = provider.send_voted_transaction(request).await?;
    println!("submit_after_vote tx_hash: {tx_hash}");

    let raw: Vec<u8> = wait_for_get_txn(&provider, tx_hash).await?;
    let history = sdk::decode_transaction_history(&raw)?;
    print_transaction_history(&history);
    Ok(())
}

fn build_vote_proposal(
    // owner: Address,
    ix: sdk::account::SetSignerWeight,
) -> Result<(milon_primitives::PackedInstruction, VoteProposal, B256), Box<dyn Error>> {
    let proposal_instruction = ix.pack()?;
    let proposal = sdk::account::VoteProposal {
        instructions: vec![proposal_instruction.as_slice().to_vec()],
        auth_bit: IdlBitmap64::from_raw(1),
    };
    let intent_hash = compute_vote_intent_hash(&proposal_instruction);
    Ok((proposal_instruction, proposal, intent_hash))
}

fn compute_vote_intent_hash(proposal_instruction: &milon_primitives::PackedInstruction) -> B256 {
    let instruction_hash = proposal_instruction.ix_hash(ChainId::new(900_000_001));
    let mut hasher = hash32_hasher(b"milon.ix-auth.batch.v1");
    hasher.update(instruction_hash.as_ref());
    hasher.update(instruction_hash.as_ref());
    B256::new(*hasher.finalize().as_bytes())
}

async fn list_active_votes(rpc: &DemoRpc) -> Result<(), Box<dyn Error>> {
    let provider = &rpc.provider;
    let owner_signer = local_ed25519_signer(1)?;
    let owner = owner_signer.address();

    let infos = provider.list_active_votes(owner).await?;
    println!("active votes: {}", infos.len());
    for info in infos {
        println!(
            "vote info intent_hash: {} {} {} {}",
            info.0.intent_hash, info.0.source_tx_hash, info.2, info.0.expires_at_ms
        );
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rpc = DemoRpc::connect(DEFAULT_HTTP_RPC_URL)?;

    // list_signers(&rpc).await?;
    // create_account(&rpc).await?;
    // create_multisig(&rpc).await?;
    // add_signer(&rpc).await?;
    // add_signers(&rpc).await?;
    // set_threshold(&rpc).await?;
    // remove_signer(&rpc).await?;
    // set_signer_weight(&rpc).await?;

    // vote_init(&rpc).await?;
    // time::sleep(Duration::from_secs(6)).await;
    // vote(&rpc).await?;

    // submit_after_vote(&rpc).await?;

    // list_signers(&rpc).await?;
    // let res = multisig_on_chain(&rpc, VoteKind::Submit).await;
    // println!(">>>>>>multisig_on_chain res: {res:?}");
    // list_signers(&rpc).await?;

    list_active_votes(&rpc).await?;
    // list_signers(&rpc).await?;
    Ok(())
}

// set_signer_weight list_signers = (
// Account { bitmap: Bitmap64(13), weight: 10, threshold: 5 },
// [(AKnL4NNf3DGWZJS6cPknBuEGnVsV4A4m5tgebLHaRSZ9, 0, 3),
// (mBKqcnGotbsSb5vNrdyhzZ5EhqZdids9QYiTRckvi7v, 2, 4),
// (AoVsGaj8MSJ6xwKxfFxo9iZWH3enC8RRTXKH2fx2F8os, 3, 3)])
async fn list_signers(rpc: &DemoRpc) -> Result<(), Box<dyn Error>> {
    let provider = &rpc.provider;
    let owner_signer = local_ed25519_signer(1)?;
    let owner = owner_signer.address();

    let signers = provider.list_signers(owner).await?;
    println!("set_signer_weight list_signers = {signers:?}");

    Ok(())
}

// async fn multisig_on_chain(rpc: &DemoRpc) -> Result<(), Box<dyn Error>> {
//     let provider = &rpc.provider;
//     let account_signer = local_ed25519_signer(1)?;
//     let pk2_signer = local_ed25519_signer(12)?;
//     let pk3_signer = local_ed25519_signer(13)?;
//     let owner = account_signer.address();
//
//     let ix = sdk::account::SetSignerWeight {
//         owner: ChainSigner::new(owner),
//         index: 2,
//         weight: 4,
//     };
//     let (_packed_ix, proposal, intent_hash) = build_vote_proposal(ix.clone())?;
//
//     let slots = vec![
//         // MultisigSlot::new(0, account_signer),
//         MultisigSlot::with_weight(2, 2, pk2_signer),
//         MultisigSlot::with_weight(3, 3, pk3_signer),
//     ];
//
//     let wallet = LocalWallet::new_multisig(owner, slots, 5)?;
//     let provider = provider.with_wallet_filler(WalletFiller::new(wallet));
//
//     let res = provider.claim_faucet_with_cooldown_remaining().await?;
//     println!("claim_faucet result: {res:?}");
//     if res.is_some() {
//         time::sleep(Duration::from_secs(1)).await;
//     }
//
//     // // vote init
//     // let res = provider.vote_init(intent_hash, proposal).await?;
//     // println!("vote_init tx_hash: {res}");
//     // time::sleep(Duration::from_secs(1)).await;
//
//     let vote_info = provider.vote_info(owner, intent_hash).await?;
//     println!("vote vote_info = {vote_info:?}");
//     // if vote_info.2 {
//     //     submit_after_multisig_on_chain(rpc, ix.clone()).await?;
//     //     return Ok(());
//     // }
//
//     // // sleep 6's
//     // time::sleep(Duration::from_secs(6)).await;
//
//     // vote
//     let res = provider.vote(intent_hash).await?;
//     println!("vote tx_hash: {res}");
//     time::sleep(Duration::from_secs(5)).await;
//
//     let vote_info = provider.vote_info(owner, intent_hash).await?;
//     if vote_info.2 {
//         submit_after_multisig_on_chain(rpc, ix).await?;
//     }
//     Ok(())
// }
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
