use milon_crypto::{Address, PublicKey};
use milon_primitives::{B160, B256};
use only_sdk_examples::local_ed25519_signer;

fn main() {
    local_ed25519_signer(2).expect("Failed to create signer");

    // parse_puk();
}

fn parse_puk() {
    let pub_key =
        PublicKey::from_str_relaxed("AKnL4NNf3DGWZJS6cPknBuEGnVsV4A4m5tgebLHaRSZ9").unwrap();
    println!("{}", pub_key.to_string());
    let addr = Address::from_public_key(&pub_key);
    println!("{}", addr.to_string());
    println!("{}", addr.to_string());

    let res = B256::new([
        201, 201, 8, 231, 96, 29, 12, 250, 15, 40, 134, 143, 198, 57, 248, 124, 188, 3, 54, 84,
        235, 82, 174, 209, 105, 238, 57, 18, 34, 44, 142, 137,
    ]);
    println!("hash: {res}")
}
