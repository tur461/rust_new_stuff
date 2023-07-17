use sha2::{Digest, Sha256};

// number of levels (inlc root):
// Num of leaves = 2^numOfLevels
// you can use log10 on both sides instead of log2, for simplcity i use log2
// log2 (numOfLeaves) = numOfLevels x log2 (2)
// numOfLevels = ceil(log2 (numOfLeaves))
// ceil() because for odd number of leaves, in merkle tree, the last leaf is concatenated with
// itself to get hash

#[derive(Debug)]
struct MerkleTree {
    root: Option<Vec<u8>>,
    levels: Vec<Vec<String>>,
}

impl MerkleTree {
    fn new(transactions: &[Vec<u8>]) -> Self {
        let mut tree = MerkleTree {
            root: None,
            levels: Vec::new(),
        };
        tree.root = Some(tree.build_tree(transactions));
        tree
    }

    fn build_tree(&mut self, transactions: &[Vec<u8>]) -> Vec<u8> {
        let mut hashes: Vec<Vec<u8>> = transactions.into_iter().map(|tx| self.hash(tx)).collect();
        let mut i = 0;
        while hashes.len() > 1 {
            let mut next_level: Vec<Vec<u8>> = Vec::new();
            let mut level: Vec<String> = Vec::new();
            for pair in hashes.chunks_exact(2) {
                let mut hash_input = Vec::new();
                //println!("pair: {:?}", pair);
                hash_input.extend_from_slice(&pair[0]);
                hash_input.extend_from_slice(&pair[1]);
                let hashed_pair = self.hash(&hash_input);
                //println!("[{}, {}] hashed pair {:?}", i, j, hashed_pair);
                next_level.push(hashed_pair.clone());
                level.push(Self::to_hex(hashed_pair));
            }
            if hashes.len() % 2 != 0 {
                let last_hash = hashes.last().unwrap().clone();
                let hashed_pair = self.hash(&[&last_hash[..], &last_hash[..]].concat());
                next_level.push(hashed_pair.clone());
                level.push(Self::to_hex(hashed_pair));
            }
            hashes = next_level;
            self.levels.push(level);
        }

        hashes.pop().unwrap()
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
    fn to_hex(bytes: Vec<u8>) -> String {
        bytes
            .iter()
            .map(|b| format!("{:02x}", b).to_string())
            .collect::<Vec<String>>()
            .join("")
    }
}

fn main() {
    let transactions = vec![
        // here you can have transaction bytes (serialized)
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 16],
        vec![17, 18, 19, 20],
        vec![21, 22, 23, 24],
        vec![25, 26, 27, 28],
        //vec![29, 30, 31, 32], // for even number of txns, uncomment
    ];

    let merkle_tree = MerkleTree::new(&transactions);
    println!(
        "Merkle Root: {:?}",
        MerkleTree::to_hex(merkle_tree.root.unwrap())
    );
    println!("\nAll levels:");

    for (i, level) in merkle_tree.levels.iter().rev().enumerate() {
        println!("\n[L: {}] {:?}", i, level);
    }
}
