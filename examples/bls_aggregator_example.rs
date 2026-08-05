use blst::min_pk::AggregatePublicKey;
use milon_crypto::{PublicKey, secretkey::SecretKey};
use milon_local_wallet::{BlsSignatureAggregator, LocalSigner, SignatureAlgorithm, Signer};
use std::error::Error;

const MESSAGE: &[u8] = b"aggregate local BLS signatures";

fn bls_signer(seed: u8) -> Result<LocalSigner, Box<dyn Error>> {
    let secret_key = SecretKey::from_bytes(&[seed; 32])?;
    Ok(LocalSigner::from_secret_key(
        secret_key,
        SignatureAlgorithm::Bls12381,
    )?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let signers = [bls_signer(1)?, bls_signer(2)?, bls_signer(3)?];
    let mut aggregator = BlsSignatureAggregator::new();

    for (index, signer) in signers.iter().enumerate() {
        let signature = signer.sign_message(MESSAGE)?;
        aggregator.add_signature(&signature)?;
        println!(
            "signer {} added, signature bytes: {}",
            index + 1,
            signature.as_bytes().len()
        );
    }

    let aggregate_signature = aggregator.build()?;
    let public_keys = signers
        .iter()
        .map(|signer| signer.public_key().to_bls12381())
        .collect::<Result<Vec<_>, _>>()?;
    let public_key_refs = public_keys.iter().collect::<Vec<_>>();
    let aggregate_public_key = AggregatePublicKey::aggregate(&public_key_refs, true)
        .map_err(|error| {
            std::io::Error::other(format!("BLS public key aggregation failed: {error:?}"))
        })?
        .to_public_key();

    aggregate_signature.verify(MESSAGE, &PublicKey::from(&aggregate_public_key))?;

    println!("message: {:?}", MESSAGE);
    println!("aggregated signatures: {}", signers.len());
    println!(
        "aggregate signature bytes: {}",
        aggregate_signature.as_bytes().len()
    );
    println!("aggregate BLS signature verified: true");

    Ok(())
}
