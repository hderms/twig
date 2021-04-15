use std::collections::HashMap;


#[derive(Debug)]
struct Node {
    children: HashMap<char, Node>,
    terminal: bool
}
impl Node {
    fn new() -> Node {
        let children = HashMap::new();
        let terminal = false;
        Node{children, terminal}
    }
    fn insert(&mut self, string: &str) {
        if string.is_empty() {
            self.terminal = true;
            return
         } 
        let head: char = string.chars().next().unwrap();
        let entry = self.children.entry(head).or_insert_with(|| Node::new());
        entry.insert(&string[1..]);
    }
}
fn main() {
    let mut root = Node::new();
    root.insert("foo");
    println!("root is: {:?}", root);

}
