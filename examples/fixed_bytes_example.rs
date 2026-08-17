use milon_primitives::FixedBytes;

fn main() {
    let tx_hash = FixedBytes::new([
        190, 218, 18, 83, 207, 98, 124, 218, 57, 128, 2, 155, 72, 209, 237, 218, 157, 213, 58, 55,
        16, 34, 70, 116, 195, 41, 246, 11, 171, 190, 79, 84,
    ]);

    println!("tx_hash: {tx_hash}");
}
