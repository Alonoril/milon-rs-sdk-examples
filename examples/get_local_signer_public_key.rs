use std::error::Error;
use milon_local_wallet::Signer;
use only_sdk_examples::local_ed25519_signer;

fn main()-> Result<(), Box<dyn Error>> {
    let signer_a = local_ed25519_signer(1)?;
    let pubkey = signer_a.public_key();
    println!("publickey-hex: {}", pubkey.to_hex());
    println!("publick-bs58: {}", pubkey.to_bs58());
    Ok(())
}
