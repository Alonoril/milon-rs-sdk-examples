use milon_client::{
    self as sdk, AccountProviderExt, TokenProviderExt, WalletFiller, account::VoteProposal, demo,
};
use milon_crypto::{Address, hash32_hasher, secretkey::SecretKey};
use milon_idl_core::{Bitmap64 as IdlBitmap64, Method, Signer as InstructionSigner};
use milon_local_wallet::{LocalWallet, MultisigSlot, Signer};
use milon_primitives::{AccountAuthorization, B256, ChainId, PackedInstruction, SigningPlan};
use milon_provider::TransactionRequest;
use only_sdk_examples::{
    DemoRpc, decode_print::print_transaction_history, local_ed25519_signer, wait_for_get_txn,
};
use std::{error::Error, time::Duration};
use tokio::time;

const DEFAULT_HTTP_RPC_URL: &str = "http://47.84.39.153:6280/milon/v1";
const OWNER_SIGNER_SEED: u8 = 100;
const IDX1_VOTE_SIGNER_SEED: u8 = 101;
const IDX2_VOTE_SIGNER_SEED: u8 = 102;
const IDX3_VOTE_SIGNER_SEED: u8 = 103;
const RELAYER_SIGNER_SEED: u8 = 200;

struct VoteIntent {
    packed_ix: PackedInstruction,
    proposal: VoteProposal,
    intent_hash: B256,
}

impl VoteIntent {
    fn new(ix: demo::InitPool) -> Result<Self, Box<dyn Error>> {
        let packed_ix = ix.pack()?;
        let proposal = VoteProposal {
            instructions: vec![packed_ix.as_slice().to_vec()],
            auth_bit: IdlBitmap64::from_raw(1),
        };
        let intent_hash = compute_vote_intent_hash(&packed_ix);
        Ok(Self {
            packed_ix,
            proposal,
            intent_hash,
        })
    }

    fn recompute_hash(&self) -> B256 {
        compute_vote_intent_hash(&self.packed_ix)
    }
}

fn vote_wallet(
    owner: Address,
    signer_seed: u8,
    index: u8,
    weight: u8,
) -> Result<LocalWallet, Box<dyn Error>> {
    let signer = local_ed25519_signer(signer_seed)?;
    Ok(LocalWallet::new_multisig(
        owner,
        vec![MultisigSlot::with_weight(index, weight, signer)],
        weight,
    )?)
}

async fn wait_after_transaction(tx_hash: B256) {
    println!("transaction tx_hash: {tx_hash}");
    time::sleep(Duration::from_secs(1)).await;
}

async fn vote_init(
    rpc: &DemoRpc,
    owner: milon_crypto::Address,
    wallet: LocalWallet,
    intent: &VoteIntent,
) -> Result<(), Box<dyn Error>> {
    let provider = rpc.provider.with_wallet_filler(WalletFiller::new(wallet));
    let faucet_result = provider.claim_faucet_with_cooldown_remaining().await?;
    println!("vote_init claim_faucet result: {faucet_result:?}");
    if faucet_result.is_some() {
        time::sleep(Duration::from_secs(1)).await;
    }

    let tx_hash = provider
        .vote_init(intent.intent_hash, intent.proposal.clone())
        .await?;
    wait_after_transaction(tx_hash).await;
    let vote_info = provider.vote_info(owner, intent.intent_hash).await?;
    println!("after vote_init vote_info: {vote_info:?}");
    Ok(())
}

async fn collect_vote(
    rpc: &DemoRpc,
    owner: milon_crypto::Address,
    wallet: LocalWallet,
    intent: &VoteIntent,
    label: &str,
) -> Result<bool, Box<dyn Error>> {
    let provider = rpc.provider.with_wallet_filler(WalletFiller::new(wallet));
    let faucet_result = provider.claim_faucet_with_cooldown_remaining().await?;
    println!("{label} claim_faucet result: {faucet_result:?}");
    if faucet_result.is_some() {
        time::sleep(Duration::from_secs(1)).await;
    }

    let tx_hash = provider.vote(intent.intent_hash).await?;
    wait_after_transaction(tx_hash).await;
    let vote_info = provider.vote_info(owner, intent.intent_hash).await?;
    println!("after {label} vote_info: {vote_info:?}");
    Ok(vote_info.2)
}

async fn submit(rpc: &DemoRpc, owner: Address, intent: &VoteIntent) -> Result<(), Box<dyn Error>> {
    let account_signer = local_ed25519_signer(OWNER_SIGNER_SEED)?;
    let idx2_vote_signer = local_ed25519_signer(IDX2_VOTE_SIGNER_SEED)?;
    let idx3_vote_signer = local_ed25519_signer(IDX3_VOTE_SIGNER_SEED)?;
    let relayer_signer = local_ed25519_signer(RELAYER_SIGNER_SEED)?;
    let payer = relayer_signer.address();
    let slots = vec![
        MultisigSlot::new(0, account_signer),
        MultisigSlot::with_weight(2, 2, idx2_vote_signer),
        MultisigSlot::with_weight(3, 3, idx3_vote_signer),
    ];
    let mut payer_wallet = LocalWallet::new(local_ed25519_signer(255)?);
    payer_wallet.register_multisig(owner, 5, slots)?;
    payer_wallet.register_signer(relayer_signer)?;
    let provider = rpc
        .provider
        .with_wallet_filler(WalletFiller::new(payer_wallet));

    let faucet_result = provider.claim_faucet_with_cooldown_remaining().await?;
    println!("submit claim_faucet result: {faucet_result:?}");
    if faucet_result.is_some() {
        time::sleep(Duration::from_secs(1)).await;
    }

    let plan =
        SigningPlan::new(payer).authorize(AccountAuthorization::new(payer, vec![0]).with_payer());
    let request = TransactionRequest::new(vec![intent.packed_ix.clone()])?
        .with_payer(payer)
        .with_signing_plan(plan);
    let tx_hash = provider.send_voted_transaction(request).await?;
    println!("submit tx_hash: {tx_hash}");

    let raw: Vec<u8> = wait_for_get_txn(&provider, tx_hash).await?;
    let history = sdk::decode_transaction_history(&raw)?;
    print_transaction_history(&history);
    Ok(())
}

async fn multisig_on_chain(rpc: &DemoRpc) -> Result<(), Box<dyn Error>> {
    let owner_signer = local_ed25519_signer(OWNER_SIGNER_SEED)?;
    let owner = owner_signer.address();

    let pool_signer = local_ed25519_signer(RELAYER_SIGNER_SEED)?;
    let pool = pool_signer.address();
    let intent = VoteIntent::new(demo::InitPool {
        pool: InstructionSigner::new(pool),
        label: "idl app demo pool".to_owned(),
    })?;
    debug_assert_eq!(intent.intent_hash, intent.recompute_hash());

    // vote_init(
    //     rpc,
    //     owner,
    //     vote_wallet(owner, IDX1_VOTE_SIGNER_SEED, 1, 1)?,
    //     &intent,
    // )
    // .await?;
    //
    // let first_vote_ready = collect_vote(
    //     rpc,
    //     owner,
    //     vote_wallet(owner, IDX2_VOTE_SIGNER_SEED, 2, 2)?,
    //     &intent,
    //     "first vote",
    // )
    // .await?;
    // if first_vote_ready {
    //     return Err("vote_info became ready after only one additional vote".into());
    // }
    //
    // let final_vote_ready = collect_vote(
    //     rpc,
    //     owner,
    //     vote_wallet(owner, IDX3_VOTE_SIGNER_SEED, 3, 3)?,
    //     &intent,
    //     "second vote",
    // )
    // .await?;
    // if !final_vote_ready {
    //     return Err("vote_info.ready is false after collecting two votes".into());
    // }
    // println!("vote_info.ready = true; submitting voted transaction");
    submit(rpc, owner, &intent).await
}

fn compute_vote_intent_hash(proposal_instruction: &PackedInstruction) -> B256 {
    let instruction_hash = proposal_instruction.ix_hash(ChainId::new(900_000_001));
    let mut hasher = hash32_hasher(b"milon.ix-auth.batch.v1");
    hasher.update(instruction_hash.as_ref());
    hasher.update(instruction_hash.as_ref());
    B256::new(*hasher.finalize().as_bytes())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rpc = DemoRpc::connect(DEFAULT_HTTP_RPC_URL)?;
    // create_multisig(&rpc).await?;

    let result = multisig_on_chain(&rpc).await;
    println!(">>>>>>multisig_on_chain res: {result:?}");

    Ok(())
}

// multisig list_signers =
// (Account { bitmap: Bitmap64(15), weight: 7, threshold: 5 },
// [(3wpYnGqceZ8DzN3guiTd9rrYkWTwTHCChBSuo6cvkXTG, 0, 1),
// (FR5pWwinRBn35GNhg7bsvw8Q13kRept2pm561DwZCQzT, 1, 1),
// (4Yk9HoDSfJv9QcmJbLcXdWVgS7nfvdUqiVcvbSu8VBru, 2, 2),
// (2FmTRNa4NTmmswmafCReLTHRTEMVEMUmzgRdBLrDRk57, 3, 3)])
async fn create_multisig(rpc: &DemoRpc) -> Result<(), Box<dyn Error>> {
    let provider = &rpc.provider;
    let owner_signer = local_ed25519_signer(OWNER_SIGNER_SEED)?;
    let pk0 = owner_signer.public_key().clone();

    let pk1_signer = local_ed25519_signer(IDX1_VOTE_SIGNER_SEED)?;
    let pk2_signer = local_ed25519_signer(IDX2_VOTE_SIGNER_SEED)?;
    let pk3_signer = local_ed25519_signer(IDX3_VOTE_SIGNER_SEED)?;
    let owner = owner_signer.address();

    let wallet = LocalWallet::new(owner_signer);
    let provider = provider.with_wallet_filler(WalletFiller::new(wallet));

    let res = provider.claim_faucet_with_cooldown_remaining().await?;
    println!("{owner} claim_faucet result: {res:?}");
    if res.is_some() {
        time::sleep(Duration::from_secs(1)).await;
    }

    let pks = vec![
        pk0,
        pk1_signer.public_key().clone(),
        pk2_signer.public_key().clone(),
        pk3_signer.public_key().clone(),
    ];
    let res = provider.create_multisig(pks, vec![1, 1, 2, 3], 5).await?;
    println!("create_multisig tx_hash: {res}");
    time::sleep(Duration::from_secs(1)).await;

    let signers = provider.list_signers(owner).await?;
    println!("multisig list_signers = {signers:?}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::VoteIntent;
    use milon_client::demo;
    use milon_crypto::{Address, secretkey::SecretKey};
    use milon_idl_core::Signer as InstructionSigner;

    #[test]
    fn vote_intent_contains_one_reusable_proposal() {
        let pool_secret = SecretKey::new_pure();
        let pool = Address::from_public_key(&pool_secret.ed25519_public());
        let intent = VoteIntent::new(demo::InitPool {
            pool: InstructionSigner::new(pool),
            label: "idl app demo pool".to_owned(),
        })
        .unwrap();

        assert_eq!(intent.proposal.instructions.len(), 1);
        assert_eq!(intent.proposal.auth_bit.raw(), 1);
        assert_eq!(intent.intent_hash, intent.recompute_hash());
    }
}
