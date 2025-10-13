mod huffman_v1;
mod huffman_v2;

fn main() {
    let source: Vec<char> = "aaabbc".chars().collect();
    let freqs = huffman_v2::construct_freqs(&source);
    let tree = huffman_v2::construct_tree(&freqs);
    let encoder = huffman_v2::construct_encoder(&tree);

    let source_encoded = huffman_v2::encode_bit(&encoder, &source);
    let source_decoded = huffman_v2::decode_bit(&tree, &source_encoded);

    println!(
        "Source: {:?}\nTree: {:?}\nEncodedBit: {:?}\nDecodedBit: {:?}",
        source, encoder, source_encoded, source_decoded
    );
}
