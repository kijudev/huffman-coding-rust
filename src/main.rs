use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
    fs,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tree {
    Leaf {
        freq: u64,
        token: char,
    },
    Node {
        freq: u64,
        left: Box<Tree>,
        right: Box<Tree>,
    },
}

impl Tree {
    pub fn freq(&self) -> u64 {
        match self {
            Tree::Leaf { freq, .. } => *freq,
            Tree::Node { freq, .. } => *freq,
        }
    }
}

impl Ord for Tree {
    fn cmp(&self, other: &Self) -> Ordering {
        self.freq().cmp(&other.freq())
    }
}

impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn construct_huffman_tree(freqs: &HashMap<char, u64>) -> Tree {
    let mut heap = BinaryHeap::new();

    for (token, freq) in freqs {
        heap.push(Reverse(Tree::Leaf {
            freq: *freq,
            token: *token,
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

pub fn construct_freqs(data: &String) -> HashMap<char, u64> {
    data.chars().fold(HashMap::new(), |mut freqs, c| {
        *freqs.entry(c).or_insert(0) += 1;
        freqs
    })
}

pub fn construct_encoder(tree: &Tree) -> HashMap<char, Vec<bool>> {
    let mut stack = vec![(tree, Vec::<bool>::new())];
    let mut encoder = HashMap::new();

    while !stack.is_empty() {
        let (subtree, code) = stack.pop().unwrap();

        match subtree {
            Tree::Leaf { token, .. } => {
                encoder.insert(*token, code);
            }
            Tree::Node { left, right, .. } => {
                let mut left_code = code.clone();
                let mut right_code = code.clone();

                left_code.push(false);
                right_code.push(true);

                stack.push((left, left_code));
                stack.push((right, right_code));
            }
        }
    }

    encoder
}

pub fn string_from_code(code: &Vec<bool>) -> str {}

fn main() {
    let data = fs::read_to_string("./src/data.txt").unwrap();
    let freqs = construct_freqs(&data);
    let tree = construct_huffman_tree(&freqs);
    let encoder = construct_encoder(&tree);

    println!("{:?}", encoder);
}
