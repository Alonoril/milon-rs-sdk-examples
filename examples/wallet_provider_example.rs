use milon_client::{self as sdk, WalletFiller, demo, token};
use milon_crypto::{Address, secretkey::SecretKey};
use milon_idl_core::{Method, Signer as InstructionSigner};
use milon_local_wallet::{
    AccountAuthorization, LocalSigner, LocalWallet, SignatureAlgorithm, Signer, SigningPlan,
};
use milon_primitives::PackedInstruction;
use milon_provider::{Provider, SendableTransaction, TransactionRequest};
use only_sdk_examples::{
    LocalProvider, build_provider,
    decode_print::{print_decoded_instructions, print_simulate_receipt, print_transaction_history},
    local_ed25519_signer, mil_token_address, wait_for_get_txn,
};
use std::{env, error::Error};

// const DEFAULT_RPC_URL: &str = "http://127.0.0.1:6380/milon/v1";
const DEFAULT_RPC_URL: &str = "http://47.84.39.153:6280/milon/v1";

type WalletProvider = milon_provider::FillProvider<
    milon_provider::RecommendedFillers,
    milon_client::WalletFiller,
    milon_provider::RootProvider,
>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rpc_url = env::var("MILON_RPC_URL").unwrap_or_else(|_| DEFAULT_RPC_URL.to_owned());
    let rpc_provider = build_provider(&rpc_url)?;
    let (provider, request) = build_demo_provider(rpc_provider)?;

    // `fill` runs ChainIdFiller, StampFiller, then WalletFiller without sending.
    let sendable = provider.fill(request.clone()).await?;
    let transaction = match sendable {
        SendableTransaction::Transaction(transaction) => transaction,
        SendableTransaction::Request(_) => {
            return Err("wallet filler returned an incomplete request".into());
        },
    };
    println!("local tx_hash: {}", transaction.tx_hash());
    print_decoded_instructions(transaction.instructions());

    let response = provider.simulate_transaction(transaction).await?;
    let receipt = sdk::decode_transaction_response(&response)?;
    print_simulate_receipt(&receipt);

    // Re-run the same filler pipeline and submit the resulting signed transaction.
    let tx_hash = provider.send_transaction(request).await?;
    println!(">>>>>>>>>>>>>submit tx_hash: {tx_hash}");

    let raw: Vec<u8> = wait_for_get_txn(&provider, tx_hash).await?;
    let history = sdk::decode_transaction_history(&raw)?;
    print_transaction_history(&history);
    Ok(())
}

fn build_demo_provider(
    provider: LocalProvider,
) -> Result<(WalletProvider, TransactionRequest), Box<dyn Error>> {
    let signer_a = local_ed25519_signer(12)?;
    let signer_b = local_ed25519_signer(2)?;
    let signer_c = local_ed25519_signer(3)?;
    let signer_d = local_ed25519_signer(4)?;

    let account_a = signer_a.address();
    let account_b = signer_b.address();
    let account_c = signer_c.address();
    let account_d = signer_d.address();

    let demo_secret = SecretKey::new_pure();
    let demo_pool = Address::from_public_key(&demo_secret.ed25519_public());
    let demo_recipient = Address::from_bytes(&[11; 20])?;

    let instructions = vec![
        transfer(account_a, mil_token_address(), account_b)?,
        transfer(account_b, mil_token_address(), account_c)?,
        demo::InitPool {
            pool: InstructionSigner::new(demo_pool),
            label: "provider filler demo".to_owned(),
        }
        .pack()?,
        demo::BatchCredit {
            pool: demo_pool,
            recipients: vec![demo_recipient],
            amount: 42,
        }
        .pack()?,
        transfer(account_c, mil_token_address(), account_d)?,
        transfer(account_d, mil_token_address(), account_a)?,
    ];

    let mut wallet = LocalWallet::new(signer_a);
    wallet.register_signer(signer_b)?;
    wallet.register_signer(signer_c)?;
    wallet.register_signer(signer_d)?;
    wallet.register_signer(LocalSigner::from_secret_key(
        demo_secret,
        SignatureAlgorithm::Ed25519,
    )?)?;

    let plan = SigningPlan::new(account_a)
        .authorize(AccountAuthorization::new(account_a, vec![0]))
        .authorize(AccountAuthorization::new(account_b, vec![1]))
        .authorize(AccountAuthorization::new(demo_pool, vec![2, 3]))
        .authorize(AccountAuthorization::new(account_c, vec![4]))
        .authorize(AccountAuthorization::new(account_d, vec![5]));
    let request = TransactionRequest::new(instructions)?.with_signing_plan(plan);

    // let transport = HttpInvokeTransport::new(Url::parse(rpc_url)?);
    // let client = RpcClient::builder().transport(reliable_transport(transport), false);
    // let provider = ProviderBuilder::new().wallet(wallet).connect_client(client);
    let provider = provider.with_wallet_filler(WalletFiller::new(wallet));
    Ok((provider, request))
}

fn transfer(
    from: Address,
    token_address: Address,
    to: Address,
) -> Result<PackedInstruction, sdk::idl_core::Error> {
    token::Transfer {
        from: InstructionSigner::new(from),
        token: token_address,
        to,
        amount: 1,
    }
    .pack()
}
