mod huffman_v1;

fn main() {
    let msg = String::from(
        "I've seen things you people wouldn't believe. Attack ships on fire off the shoulder of Orion. I watched C-beams glitter in the dark near the Tannhäuser Gate. All those moments will be lost in time, like tears in rain. Time to die.",
    );
    let freqs = huffman_v1::construct_freqs(&msg);
    let tree = huffman_v1::construct_huffman_tree(&freqs);
    let encoder = huffman_v1::construct_encoder(&tree);

    let msg_encoded = huffman_v1::encode(&encoder, &msg);
    let msg_decoded = huffman_v1::decode(&tree, &msg_encoded);

    println!("msg_encoded: {:?}", msg_encoded);
    println!("msg_decoded: {:?}", msg_decoded);
    println!("---------------");
    println!(
        "(in bits) msg len: {:?}, encoded len: {:?}",
        msg.len() * 8,
        msg_encoded.len()
    );
}
