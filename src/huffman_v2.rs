use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};

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
    tokens.iter().fold(HashMap::new(), |mut freqs, t| {
        *freqs.entry(t.clone()).or_insert(0) += 1;
        freqs
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

pub fn construct_encoder<T: Clone + Eq + Ord + Hash>(tree: &Tree<T>) -> HashMap<T, BitVec> {
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

pub fn encode_bit<T: Clone + Eq + Ord + Hash>(
    encoder: &HashMap<T, BitVec>,
    source: &Vec<T>,
) -> BitVec {
    let mut output = BitVec::new();

    for token in source {
        output.extend_from_bitslice(encoder.get(token).unwrap().as_bitslice());
    }

    output
}

pub fn decode_bit<T: Clone + Eq + Ord + Hash>(tree: &Tree<T>, source: &BitVec) -> Vec<T> {
    let mut output = Vec::new();
    let mut current_tree = tree.clone();
    let mut i = 0;

    while i < source.len() {
        match current_tree {
            Tree::Leaf { token, .. } => {
                output.push(token.clone());
                current_tree = tree.clone();
            }
            Tree::Node { left, right, .. } => {
                if source[i] {
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
