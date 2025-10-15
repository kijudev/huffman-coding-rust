use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};

use serde::{Deserialize, Serialize};

use bitvec::vec::BitVec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tree<T> {
    Leaf {
        freq: u64,
        token: T,
    },
    Node {
        freq: u64,
        left: Box<Tree<T>>,
        right: Box<Tree<T>>,
    },
}

impl<T: Clone> Tree<T> {
    pub fn freq(&self) -> u64 {
        match self {
            Tree::Leaf { freq, .. } => *freq,
            Tree::Node { freq, .. } => *freq,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncodedMessage<T: Clone + Hash + Eq> {
    freqs: HashMap<T, u64>,
    message: BitVec,
}

impl<T: Clone + Eq> Ord for Tree<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.freq().cmp(&other.freq())
    }
}

impl<T: Clone + Eq> PartialOrd for Tree<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn construct_freqs<T: Clone + Eq + Hash>(tokens: &Vec<T>) -> HashMap<T, u64> {
    tokens.iter().fold(HashMap::new(), |mut acc, token| {
        *acc.entry(token.clone()).or_insert(0) += 1;
        acc
    })
}

pub fn construct_tree<T: Clone + Eq + Ord>(freqs: &HashMap<T, u64>) -> Tree<T> {
    let mut heap = BinaryHeap::new();

    for (token, freq) in freqs {
        heap.push(Reverse(Tree::Leaf {
            freq: *freq,
            token: token.clone(),
        }));
    }

    while heap.len() > 1 {
        let (left, right) = (heap.pop().unwrap().0, heap.pop().unwrap().0);

        let parent = Tree::Node {
            freq: left.freq() + right.freq(),
            left: Box::new(left),
            right: Box::new(right),
        };

        heap.push(Reverse(parent));
    }

    heap.pop().unwrap().0
}

pub fn construct_encoder<T: Clone + Eq + Hash>(tree: &Tree<T>) -> HashMap<T, BitVec> {
    let mut stack: Vec<(&Tree<T>, BitVec)> = vec![(tree, BitVec::new())];
    let mut encoder = HashMap::new();

    while stack.len() > 0 {
        let (subtree, code) = stack.pop().unwrap();

        match subtree {
            Tree::Leaf { token, .. } => {
                encoder.insert(token.clone(), code);
            }
            Tree::Node { left, right, .. } => {
                let (mut code_left, mut code_right) = (code.clone(), code.clone());

                code_left.push(false);
                code_right.push(true);

                stack.push((left, code_left));
                stack.push((right, code_right));
            }
        }
    }

    encoder
}

pub fn encode_bit<T: Eq + Hash>(encoder: &HashMap<T, BitVec>, tokens: &Vec<T>) -> BitVec {
    tokens.iter().fold(BitVec::new(), |mut acc, token| {
        acc.extend_from_bitslice(encoder.get(token).unwrap().as_bitslice());
        acc
    })
}

pub fn decode_bit<T: Clone + Eq + Hash>(tree: &Tree<T>, encoded_message: &BitVec) -> Vec<T> {
    let mut output = Vec::new();
    let mut current_tree = tree.clone();
    let mut i = 0;

    while i < encoded_message.len() {
        match current_tree {
            Tree::Leaf { token, .. } => {
                output.push(token.clone());
                current_tree = tree.clone();
            }
            Tree::Node { left, right, .. } => {
                if encoded_message[i] {
                    current_tree = right.as_ref().clone();
                } else {
                    current_tree = left.as_ref().clone();
                }

                i += 1;
            }
        }
    }

    match current_tree {
        Tree::Leaf { token, .. } => output.push(token.clone()),
        Tree::Node { .. } => (),
    }

    output
}

pub fn encode<'a, T, TokenExtractor>(message: &'a String, extract_tokens: TokenExtractor) -> Vec<u8>
where
    T: Clone + Eq + Hash + Ord + Serialize,
    TokenExtractor: Fn(&'a str) -> Vec<T>,
{
    let tokens = extract_tokens(&message);
    let freqs = construct_freqs(&tokens);
    let tree = construct_tree(&freqs);
    let encoder = construct_encoder(&tree);

    let bits = encode_bit(&encoder, &tokens);
    let encoded_message = EncodedMessage {
        freqs: freqs.clone(),
        message: bits,
    };

    rmp_serde::encode::to_vec(&encoded_message).unwrap()
}

pub fn encode_with_freqs<'a, T, TokenExtractor>(
    message: &'a String,
    extract_tokens: TokenExtractor,
    freqs: &HashMap<T, u64>,
) -> Vec<u8>
where
    T: Clone + Eq + Hash + Ord + Serialize,
    TokenExtractor: Fn(&'a str) -> Vec<T>,
{
    let tokens = extract_tokens(&message);
    let tree = construct_tree(&freqs);
    let encoder = construct_encoder(&tree);

    let bits = encode_bit(&encoder, &tokens);

    rmp_serde::encode::to_vec(&bits).unwrap()
}

pub fn decode<'a, T, TokensToString>(
    encoded_message: &'a Vec<u8>,
    tokens_to_string: TokensToString,
) -> String
where
    T: Clone + Eq + Hash + Deserialize<'a> + Ord,
    TokensToString: Fn(Vec<T>) -> String,
{
    let EncodedMessage {
        freqs,
        message: bits,
    }: EncodedMessage<T> = rmp_serde::decode::from_slice(encoded_message).unwrap();

    let tree = construct_tree(&freqs);
    tokens_to_string(decode_bit(&tree, &bits))
}

pub fn decode_with_freqs<'a, T, TokensToString>(
    encoded_message: &'a Vec<u8>,
    tokens_to_string: TokensToString,
    freqs: &HashMap<T, u64>,
) -> String
where
    T: Clone + Eq + Hash + Deserialize<'a> + Ord,
    TokensToString: Fn(Vec<T>) -> String,
{
    let bits: BitVec = rmp_serde::decode::from_slice(encoded_message).unwrap();

    let tree = construct_tree(&freqs);
    tokens_to_string(decode_bit(&tree, &bits))
}
