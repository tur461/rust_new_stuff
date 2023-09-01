use std::{collections::HashMap, io};

#[derive(Debug, Clone)]
struct TrieNode {
    is_word: bool,
    children: HashMap<char, Self>,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            is_word: false,
            children: HashMap::new(),
        }
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    fn insert(mut self, word: String) {
        let mut curr = self.root.clone();
        word.chars().into_iter().for_each(|c| {
            if !curr.children.contains_key(&c) {
                curr.children.insert(c, TrieNode::new());
            }
            curr = curr.children.get(&c).unwrap().to_owned();
        });
        curr.is_word = true;
    }

    fn search(mut self, word: String) -> bool {
        let mut curr = self.root;
        for c in word.chars().into_iter() {
            if !curr.children.contains_key(&c) {
                return false;
            }
            curr = curr.children.get(&c).unwrap().clone();
        }

        curr.is_word
    }
}

fn main() {
    println!("Hello, world!");
}
