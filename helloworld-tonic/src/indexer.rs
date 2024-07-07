use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    collections::HashMap,
};


#[path = "./trie.rs"]
mod trie;

use trie::Node;


pub struct Indexer {
    // tenantTrieMap: HashMap<String, Node>,
    //keep tenant trie map
    trie: Node
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

impl Indexer {
    pub fn new() -> Self {
        let mut trie = Node::new();
        let indexer = Indexer { trie: trie };
        indexer
    }

    pub fn indexFile(&mut self, tenantID: &str, filePath: &str) {
        // println!("Hello world");

        // let words = lines_from_file("/Users/dushyant.bansal/work/rprojects/helloworld-tonic/words.txt"); //sample
        let words = lines_from_file("/Users/dushyant.bansal/work/rprojects/helloworld-tonic/words_alpha.txt"); //all words
        // for line in &words {
        //     println!("{:?}", line);
        // }
        // let words: [&str; 3] = ["apple", "april", "mango"];

        //fill trie
        for word in words.iter() {
            // add word to prefix trie
            let mut trie_ref = &mut self.trie;
            trie::addWord(trie_ref, word);
        }
    }

    pub fn prefixMatch(&self, word: &str) -> Vec<String>  {
        return trie::prefixMatch(&self.trie, word);
    }
}