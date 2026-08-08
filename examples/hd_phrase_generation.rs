use milon_local_wallet::{Mnemonic, WordCount};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Milon mnemonic generation demo ===\n");

    for word_count in [
        WordCount::Words12,
        WordCount::Words15,
        WordCount::Words18,
        WordCount::Words21,
        WordCount::Words24,
    ] {
        let mnemonic = Mnemonic::generate(word_count)?;
        let phrase = mnemonic.phrase();
        let reparsed = Mnemonic::from_phrase(&phrase)?;
        println!(
            "{word_count:?}: {} words, valid: {}",
            phrase.split_whitespace().count(),
            reparsed.phrase().as_str() == phrase.as_str()
        );
        println!("  {}", phrase.as_str());
    }

    let default_mnemonic = Mnemonic::generate_default()?;
    println!(
        "\ndefault phrase word count: {}",
        default_mnemonic.phrase().split_whitespace().count()
    );
    Ok(())
}
