use milon_crypto::secretkey::SecretKey;
use milon_local_wallet::{LocalSigner, SignatureAlgorithm, Signer};
use std::error::Error;

pub fn local_ed25519_signer(seed: u8) -> Result<LocalSigner, Box<dyn Error>> {
    let signer = LocalSigner::from_secret_key(
        SecretKey::from_bytes(&[seed; 32])?,
        SignatureAlgorithm::Ed25519,
    )?;

    let public_key = signer.public_key().clone();
    let account_address = signer.address();

    println!("seed[{seed}]bytes->public_key: {:?}", public_key.as_bytes());
    println!("seed[{seed}]hex->public_key: {}", public_key.to_hex());
    println!("seed[{seed}]bs58->public_key: {}", public_key.to_bs58());
    println!("seed[{seed}]bs58->address: {account_address}");

    Ok(signer)
}
