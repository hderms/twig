use serde::Serialize;
use std::{collections::HashMap, sync::{Arc, RwLock}};
#[derive(Debug, Serialize)]
pub struct Node {
    children: HashMap<char, Node>,
    terminal: bool,
}
pub struct Umbrella {
    roots: HashMap<char, Arc<RwLock<Node>>>
}
impl Umbrella{
    pub fn new() -> Umbrella {
        let mut roots = HashMap::new();
        for char in 'a'..'z' {
            roots.insert(char, Arc::new(RwLock::new(Node::new())));


        }

        for char in 'A'..'Z' {
            roots.insert(char, Arc::new(RwLock::new(Node::new())));
        }
        Umbrella{
            roots
        }
    }
    pub fn get(&self, string: &str) -> &RwLock<Node> {
        self.roots.get(&string.chars().next().unwrap()).unwrap()
    }
}
impl Node {
    pub fn new() -> Node {
        let children = HashMap::with_capacity(256);
        let terminal = false;
        Node { children, terminal }
    }
    pub fn insert(&mut self, string: &str) {
        if string.is_empty() {
            self.terminal = true;
            return;
        }
        let head: char = string.chars().next().unwrap();
        let entry = self.children.entry(head).or_insert_with(Node::new);
        entry.insert(&string[1..]);
    }

    pub fn suggest<'a>(&'a self, string: &'a str, limit: usize) -> Option<Vec<String>> {
        let mut vec = Vec::with_capacity(limit);
        self.find(string).map(|found| {
            if found.terminal {
                vec.push(string.to_string());
            }
            found.collect_suggest(&mut vec, string, limit).to_owned()
        })
    }

    fn collect_suggest<'a>(
        &self,
        vec: &'a mut Vec<String>,
        base_str: &'a str,
        limit: usize,
    ) -> &'a Vec<String> {
        let mut level = base_str.to_string();
        for (char, child) in self.children.iter() {
            level.push(*char);
            if vec.len() == limit {
                break;
            }
            if child.terminal {
                vec.push(level.clone());
            }
            child.collect_suggest(vec, level.clone().as_str(), limit);
            level.pop();
        }
        vec
    }

    fn find(&self, string: &str) -> Option<&Node> {
        if string.is_empty() {
            return Some(self);
        }
        let head: char = string.chars().next().unwrap();
        let entry = self.children.get(&head)?;
        entry.find(&string[1..])
    }
}
