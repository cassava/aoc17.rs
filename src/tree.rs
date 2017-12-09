extern crate regex;
#[macro_use] extern crate lazy_static;

use std::cell::RefCell;
use regex::Regex;

// A Tree contains the root node.
#[derive(Debug)]
pub struct Tree(Node);

impl Tree {
    pub fn from_iter<'a, I>(it: I) -> Self
    where I: Iterator<Item = &'a str> {

    }

}

struct ParsedNodeLine {
    name: &str,
    weight: u32,
    children: Vec<&str>,
}

impl ParsedNodeLine {
    // pbga (66)
    // fwft (72) -> ktlj, cntj, xhth
    fn from_str(s: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^([[:alpha:]]+) \((\d+)\)(?: -> (?([[:alpha:]]+), )+)$"
            );
        }

        // continue here with parsing

    }
}

#[derive(Debug, Clone)]
pub struct Node {
    name: String,
    weight: u32,
    parent: Weak<RefCell<Node>>,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Node{
            name: String::new(name),
            weight: 0,
            parent: Weak::new(),
            children: Vec::new(),
        }
    }
}
