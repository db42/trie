use std::cell::RefCell;
use std::rc::Rc;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

struct Node {
    charMap: Vec<Option<Node>>,
    endOfWord: bool,
}

impl Node {
    fn new() -> Self {
    let mut node = Node { 
        charMap: Vec::new(),
        endOfWord: false
    };
    while node.charMap.len() < 26 {
        node.charMap.push(None);
    }
    node
        // Node { charMap: vec![None; 26] } // Pre-allocate with 26 None elements
    }
}

pub struct Indexer {
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
        // println!("Hello world");

        // let words = lines_from_file("/Users/dushyant.bansal/work/rprojects/helloworld-tonic/words.txt"); //sample
        let words = lines_from_file("/Users/dushyant.bansal/work/rprojects/helloworld-tonic/words_alpha.txt"); //all words
        // for line in &words {
        //     println!("{:?}", line);
        // }
        // let words: [&str; 3] = ["apple", "april", "mango"];

        // create trie
        let mut trie = Node::new();
        for word in words.iter() {
            // add word to prefix trie
            addWord(&mut trie, word);
        }

        let indexer = Indexer { trie: trie };
        indexer
    }

    pub fn prefixMatch(&self, word: &str) -> Vec<String>  {
        return prefixMatch(&self.trie, word);
    }
}

// fn main() {

//     //find a word in trie
//     // findWord(&trie, needle);
//     // findWord(&trie, "apl");
//     let matches = prefixMatch(&trie, "ap");
//     println!("{:?}", matches);

//     println!("{:?}",prefixMatch(&trie, "apple"));
//     println!("{:?}",prefixMatch(&trie, "aps"));
//     println!("{:?}",prefixMatch(&trie, "man"));
// }

fn addWord(node: &mut Node, word: &str) {
    if let Some(character) = word.chars().nth(0) {
        let index = (character as u32 - 97) as usize;
        // println!("character {} index {}", character, index);

        if let Some(child_node) = &mut node.charMap[index] {
            let substring = &word[1..];
            addWord(child_node, &substring)
        } else {
            let mut new_node = Node::new();
            node.charMap[index] = Some(new_node);
            
            let node_rc = node.charMap[index].as_mut().unwrap();

            let substring = &word[1..];
            addWord(node_rc, &substring)
        }
    } else {
        node.endOfWord = true;
        return
    }
}

fn findWord(trie: &Node, word: &str) -> bool {
    let mut node = trie;
    for character in word.chars() {
        let index = (character as u32 - 97) as usize;

        if let Some(child_node) = &node.charMap[index] {
            node = child_node;
            continue;
        } else {
            println!("word not found {}", word);
            return false;
        }
    }
    println!("word found {}", word);
    return true;
}

fn prefixMatch(trie: &Node, prefix: &str) -> Vec<String> {
    let mut node = trie;
    for character in prefix.chars() {
        let index = (character as u32 - 97) as usize;

        if let Some(child_node) = &node.charMap[index] {
            node = child_node;
            continue;
        } else {
            println!("prefix not found {}", prefix);
            // return matches;
            return Vec::new();
        }
    }


    //do a dfs traversal starting from node and print all prefixMatches
    let mut st = "";
    let mut matches = dfs(&node, &prefix);
    if (node.endOfWord == true) {
        matches.insert(0, prefix.to_string());
    }

    return matches;
}

fn dfs(trie: &Node, pre: &str) -> Vec<String> {
    let mut matches = Vec::new();
    let mut is_last_node = true;

    for index in 0..26 {
        if let Some(child_node) = &trie.charMap[index] {
            is_last_node = false;

            let ascii_code = (index + 97) as u32;
            let character = char::from_u32(ascii_code).unwrap();

            // println!("more character pre {} char {}",pre, character );
            let new_string = format!("{}{}", pre, character);
            let temp_matches = dfs(child_node, &new_string);
            matches.extend_from_slice(&temp_matches);
        } else {

        }
    }

    if is_last_node {
        // println!("completions {}", pre);
        matches.push(pre.to_string());
    }

    return matches;
}