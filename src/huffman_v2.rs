use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};

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

fn construct_freqs<T: Clone + Eq + Hash>(tokens: &Vec<T>) -> HashMap<T, u64> {
    tokens.iter().fold(HashMap::new(), |mut freqs, t| {
        *freqs.entry(t.clone()).or_insert(0) += 1;
        freqs
    })
}

fn construct_tree<T: Clone + Eq + Ord>(freqs: HashMap<T, u64>) -> Tree<T> {
    let mut heap = BinaryHeap::new();

    for (token, freq) in freqs {
        heap.push(Reverse(Tree::Leaf {
            freq: freq,
            token: token,
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
