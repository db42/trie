use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    charMap: Vec<Option<Rc<RefCell<Node>>>>,
}

fn emptyCharMap() {

}

fn main() {
    println!("Hello world");
    let words: [&str; 1] = ["a"];

    //create prefix tree
    let mut trie = Node { charMap: Vec::new()};    

    // trie.charMap.resize(26, None);

    // let child_node = Rc::new(RefCell::new(Node {charMap: Vec::new()}));
    // trie.charMap[0] = Some(child_node);

    // let word = words[0];
    // let substring = &word[1..];
    // println!("{}", substring);
    // iterate over arr
    for word in words.iter() {
        // add word to prefix trie
        addWord(&trie, word);
    }
}

fn addWord(node: &Node, word: &str) {
    for character in word.chars() {
        let index = (character as u32 - 97) as usize;

        if let Some(child_node_rc) = &node.charMap[index] {
            let child_node = child_node_rc.borrow();
            let substring = &word[1..];
            addWord(&child_node, &substring)
        } else {
            //insert
            node.charMap.resize(26, None);

            let new_node_rc = Rc::new(RefCell::new(Node {charMap: Vec::new()}));
            let new_node = new_node_rc.borrow();
            node.charMap[index] = Some(new_node);

            let substring = &word[1..];
            addWord(&new_node, &substring)
        }
    }
}
