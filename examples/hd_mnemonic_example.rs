use milon_local_wallet::{LocalWallet, SignatureAlgorithm};
use std::env;

const DEFAULT_PHRASE: &str = "alter twenty leader lock siege join rare radio debris helmet enable alcohol ecology evidence coconut ring vocal topple virus husband able field banner lake";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut arguments = env::args().skip(1);
    let phrase = arguments
        .next()
        .unwrap_or_else(|| DEFAULT_PHRASE.to_owned());
    let passphrase = arguments.next().unwrap_or_default();

    println!("phrase: {phrase}");
    println!("passphrase supplied: {}", !passphrase.is_empty());

    println!("\n-- single wallet --");
    let first_wallet =
        LocalWallet::from_phrase(&phrase, &passphrase, 0, SignatureAlgorithm::FnDsa512)?;
    print_wallet(0, &first_wallet);

    println!("\n-- batch wallets from the same phrase --");
    for account in 1..4 {
        let wallet =
            LocalWallet::from_phrase(&phrase, &passphrase, account, SignatureAlgorithm::FnDsa512)?;
        print_wallet(account, &wallet);
    }

    Ok(())
}

fn print_wallet(account: u32, wallet: &LocalWallet) {
    println!(
        "account {account:>3} | address {}",
        wallet.default_account()
    );
}
