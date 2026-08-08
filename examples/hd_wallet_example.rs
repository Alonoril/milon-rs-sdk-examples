use milon_local_wallet::{LocalSigner, LocalWallet, Mnemonic, SignatureAlgorithm, Signer};

const PHRASE: &str = "legal winner thank year wave sausage worth useful legal winner thank yellow";
const MESSAGE: &[u8] = b"Milon HD wallet signature demo";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mnemonic = Mnemonic::from_phrase(PHRASE)?;

    for algorithm in SignatureAlgorithm::ALL {
        let signer = LocalSigner::from_mnemonic(&mnemonic, "", 0, algorithm)?;
        let public_key = *signer.public_key();
        let signature = signer.sign_message(MESSAGE)?;
        signature.verify(MESSAGE, &public_key)?;

        let wallet = LocalWallet::new(signer);
        println!(
            "algorithm: {algorithm:?}\n  address: {}\n  signature bytes: {}\n  verified: true",
            wallet.default_account(),
            signature.as_bytes().len(),
        );
    }

    Ok(())
}
