//mod huffman_v1;
mod huffman_v2;

fn main() {
    let message = String::from("aaabbc");
    let compressed = huffman_v2::encode(&message, |m| m.chars().collect());
    let decompressed = huffman_v2::decode(&compressed, |cs: Vec<char>| cs.into_iter().collect());

    println!(
        "Message: {:?}\nCompressed: {:?}\n Decompressed: {:?}",
        message, compressed, decompressed
    );
}
