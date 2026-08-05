use milon_client::{self as sdk, AccountProviderExt, demo, token};
use milon_crypto::{Address, secretkey::SecretKey};
use milon_idl_core::{Method, Signer as InstructionSigner};
use milon_local_wallet::{
    AccountAuthorization, LocalSigner, LocalWallet, SignatureAlgorithm, Signer as WalletSigner,
    SigningPlan,
};
use milon_primitives::{ChainId, Transaction};
use milon_provider::Provider;
use only_sdk_examples::{
    DemoRpc, claim_faucet,
    decode_print::{print_decoded_instructions, print_simulate_receipt},
    local_ed25519_signer, mil_token_address,
};
use serde::{Deserialize, Serialize};
use std::{
    env,
    error::Error,
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

// const DEFAULT_HTTP_RPC_URL: &str = "http://127.0.0.1:6380/milon/v1";
const DEFAULT_HTTP_RPC_URL: &str = "http://47.84.39.153:6280/milon/v1";
const DEFAULT_CHAIN_ID: u64 = 900_000_001;
const TX_STAMP_LEAD_MS: u64 = 30_000;
const TRANSFER_AMOUNT: u64 = 1;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let input = load_json_input()?;
    let rpc = DemoRpc::connect(&input.rpc_url)?;
    let request =
        build_transfer_request(input.chain_id, &rpc, input.stamp.unwrap_or_else(next_stamp))
            .await?;
    println!("local tx_hash: {}", request.tx_hash());

    print_decoded_instructions(request.instructions());

    let response = rpc.provider.simulate_transaction(request.clone()).await?;
    let receipt = sdk::decode_transaction_response(&response)?;
    print_simulate_receipt(&receipt);
    println!(
        "------------------------------------------------------------------------------------------------"
    );

    // submit tx
    let tx_hash = rpc.provider.submit_transaction(request).await?;
    println!("submit txn tx_hash: {tx_hash}");

    Ok(())
}

async fn build_transfer_request(
    chain_id: ChainId,
    rpc: &DemoRpc,
    stamp: u64,
) -> Result<Transaction, Box<dyn Error>> {
    let signer_a = local_ed25519_signer(11)?;
    let signer_b = local_ed25519_signer(2)?;
    let signer_c = local_ed25519_signer(3)?;
    let signer_d = local_ed25519_signer(4)?;

    let account_a = signer_a.address();
    let account_b = signer_b.address();
    let account_c = signer_c.address();
    let account_d = signer_d.address();

    let token_address = mil_token_address();
    let demo_secret = SecretKey::new_pure();
    let demo_pool = Address::from_public_key(&demo_secret.ed25519_public());
    let demo_recipient = Address::from_bytes(&[11_u8; 20])?;

    let init_pool = demo::InitPool {
        pool: InstructionSigner::new(demo_pool),
        label: "simulate batch credit".to_owned(),
    };
    let batch_credit = demo::BatchCredit {
        pool: demo_pool,
        recipients: vec![demo_recipient],
        amount: 42,
    };
    let demo_signer = LocalSigner::from_secret_key(demo_secret, SignatureAlgorithm::Ed25519)?;

    let instructions = vec![
        claim_faucet(account_a)?,
        transfer(account_a, token_address, account_b)?,
        transfer(account_b, token_address, account_c)?,
        init_pool.pack()?,
        batch_credit.pack()?,
        transfer(account_c, token_address, account_d)?,
        transfer(account_d, token_address, account_a)?,
    ];

    let mut wallet = LocalWallet::new(signer_a);
    wallet.register_signer(signer_b)?;
    wallet.register_signer(signer_c)?;
    wallet.register_signer(signer_d)?;
    wallet.register_signer(demo_signer)?;

    let plan = SigningPlan::new(account_a)
        .authorize(AccountAuthorization::new(account_a, vec![0, 1]).with_payer())
        .authorize(AccountAuthorization::new(account_b, vec![2]))
        .authorize(AccountAuthorization::new(demo_pool, vec![3, 4]))
        .authorize(AccountAuthorization::new(account_c, vec![5]))
        .authorize(AccountAuthorization::new(account_d, vec![6]));

    // let plan = rpc.provider.prepare_signing_plan(&wallet, plan).await?;
    let mut transaction =
        Transaction::new_with_stamp(chain_id, stamp, Some(plan.payer()), instructions);
    wallet.sign_transaction_with_plan(&mut transaction, &plan)?;
    Ok(transaction)
}

fn transfer(
    from: Address,
    token_address: Address,
    to: Address,
) -> Result<sdk::PackedInstruction, sdk::idl_core::Error> {
    token::Transfer {
        from: InstructionSigner::new(from),
        token: token_address,
        to,
        amount: TRANSFER_AMOUNT,
    }
    .pack()
}

// fn claim_faucet(claimer: Address) -> Result<sdk::PackedInstruction, sdk::idl_core::Error> {
//     token::ClaimFaucet {
//         claimer: InstructionSigner::new(claimer),
//     }
//     .pack()
// }

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
struct SubmitJson {
    #[serde(default = "default_rpc_url")]
    rpc_url: String,
    #[serde(default = "default_chain_id")]
    chain_id: ChainId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    stamp: Option<u64>,
}

fn load_json_input() -> Result<SubmitJson, Box<dyn Error>> {
    if let Ok(path) = env::var("MILON_SUBMIT_TX_JSON_FILE") {
        return parse_json_input(&fs::read_to_string(path)?);
    }
    if let Ok(raw) = env::var("MILON_SUBMIT_TX_JSON") {
        return parse_json_input(&raw);
    }
    Ok(SubmitJson::default_for_demo())
}

fn parse_json_input(raw: &str) -> Result<SubmitJson, Box<dyn Error>> {
    Ok(serde_json::from_str(raw)?)
}

fn next_stamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before unix epoch")
        .as_millis() as u64
        + TX_STAMP_LEAD_MS
}

fn default_rpc_url() -> String {
    DEFAULT_HTTP_RPC_URL.to_owned()
}

const fn default_chain_id() -> ChainId {
    ChainId::new(DEFAULT_CHAIN_ID)
}

impl SubmitJson {
    fn default_for_demo() -> Self {
        Self {
            rpc_url: default_rpc_url(),
            chain_id: default_chain_id(),
            stamp: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::build_transfer_request;
    use milon_primitives::ChainId;

    // #[test]
    // fn transfer_ring_includes_demo_batch_credit() {
    //     let transaction = build_transfer_request(ChainId::new(900_000_001), 1_700_000_000)
    //         .expect("build transfer request")
    //         .into_transaction()
    //         .expect("sign transfer request");
    //     let decoded = transaction
    //         .instructions()
    //         .iter()
    //         .map(milon_client::decode_instruction)
    //         .collect::<Result<Vec<_>, _>>()
    //         .expect("decode transfer instructions");
    //
    //     assert_eq!(decoded.len(), 7);
    //     assert_eq!(decoded[0].instruction_name, "ClaimFaucet");
    //     assert_eq!(decoded[3].instruction_name, "InitPool");
    //     assert_eq!(decoded[4].instruction_name, "BatchCredit");
    //     assert!(
    //         [1, 2, 5, 6]
    //             .iter()
    //             .all(|&index| decoded[index].instruction_name == "Transfer")
    //     );
    // }
}
