use milon_local_wallet::{LocalWallet, Mnemonic, SignatureAlgorithm};

// const PHRASE: &str = "legal winner thank year wave sausage worth useful legal winner thank yellow";
const PHRASE: &str = "alter twenty leader lock siege join rare radio debris helmet enable alcohol ecology evidence coconut ring vocal topple virus husband able field banner lake";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mnemonic = Mnemonic::from_phrase(PHRASE)?;
    let phrase = mnemonic.phrase();
    println!("phrase: {}", phrase.as_str());

    println!("\n-- derive wallets from the same phrase --");
    for account in 0..5 {
        let wallet = wallet_from_mnemonic(&mnemonic, "", account)?;
        println!("account {account}: {}", wallet.default_account());
    }

    println!("\n-- deterministic derivation --");
    let first = wallet_from_mnemonic(&mnemonic, "", 0)?;
    let second = wallet_from_mnemonic(&mnemonic, "", 0)?;
    println!(
        "same address: {}",
        first.default_account() == second.default_account()
    );

    println!("\n-- passphrase changes the derived wallet --");
    let protected = wallet_from_mnemonic(&mnemonic, "TREZOR", 0)?;
    println!("empty passphrase: {}", first.default_account());
    println!("TREZOR passphrase: {}", protected.default_account());
    Ok(())
}

fn wallet_from_mnemonic(
    mnemonic: &Mnemonic,
    passphrase: &str,
    account: u32,
) -> Result<LocalWallet, Box<dyn std::error::Error>> {
    Ok(LocalWallet::from_mnemonic(
        mnemonic,
        passphrase,
        account,
        SignatureAlgorithm::FnDsa512,
    )?)
}
