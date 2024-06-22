use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    charMap: Vec<Option<Node>>,
}

impl Node {
    fn new() -> Self {
    let mut node = Node { charMap: Vec::new()};    
    while node.charMap.len() < 26 {
        node.charMap.push(None);
    }
    node
        // Node { charMap: vec![None; 26] } // Pre-allocate with 26 None elements
    }
}

fn emptyCharMap() {

}

fn main() {
    println!("Hello world");
    let words: [&str; 3] = ["apple", "banana", "mango"];

    //create prefix tree
    // let mut trie = Node { charMap: Vec::new()};    
    // let target_size = 26;
    // while trie.charMap.len() < target_size {
    //     trie.charMap.push(None);
    // }

    // trie.charMap.resize(26, None);

    // let child_node = Rc::new(RefCell::new(Node {charMap: Vec::new()}));
    // trie.charMap[0] = Some(child_node);

    // let word = words[0];
    // let substring = &word[1..];
    // println!("{}", substring);

    // create trie
    let mut trie = Node::new();
    for word in words.iter() {
        // add word to prefix trie
        addWord(&mut trie, word);
    }

    //find a word in trie
    let needle = "app";
    // findWord(&trie, needle);
    // findWord(&trie, "apl");
    prefixMatch(&trie, "ap");
}

fn addWord(node: &mut Node, word: &str) {
    if let Some(character) = word.chars().nth(0) {
        let index = (character as u32 - 97) as usize;
        println!("character {} index {}", character, index);

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

fn prefixMatch(trie: &Node, prefix: &str) -> Vec<&str> {
    let mut matches = Vec::new();
    let mut node = trie;
    for character in prefix.chars() {
        let index = (character as u32 - 97) as usize;

        if let Some(child_node) = &node.charMap[index] {
            node = child_node;
            continue;
        } else {
            println!("prefix not found {}", prefix);
            return matches;
        }
    }

    //do a dfs traversal starting from node
    let mut st = "";
    dfs(&node, &st, &matches);

    println!("prefix found {}", prefix);
    return matches;
}

fn dfs(trie: &Node, pre: &str, arr: &Vec<&str>) {
    // let mut matches = Vec::new();
    let is_last_node = false;
    for index in 0..26 {
        if let Some(child_node) = &trie.charMap[index] {
            is_last_node = true;

            let ascii_code = (index + 97) as u32;
            let character = char::from_u32(ascii_code);

            let new_string = pre + &character.to_string(); // Convert char to string first

            dfs(trie, &new_string, arr);


        } else {

        }
        // println!("Count: {}", i);
    }

    if is_last_node {
        arr.push(pre);
    }
}
