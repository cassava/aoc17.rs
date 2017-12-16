use std::rc::{Rc, Weak};
use std::cell::{RefCell};

pub struct Node<T> {
    link: Rc<RefCell<Link<T>>>,
}

struct Link<T> {
    parent: Option<Weak<RefCell<Link<T>>>>,
    children: Vec<Node<T>>,
    data: T,
}

impl<T> Node<T> {
    fn from_link(link: Rc<RefCell<Link<T>>>) -> Self {
        Self { link: link }
    }

    pub fn has_parent(&self) -> bool {
        self.link.borrow().parent.is_some()
    }

    pub fn parent(&self) -> Option<Node<T>> {
        self.link.borrow()
            .parent
            .clone()
            .and_then(|p| p.upgrade())
            .map(|p| Node::from_link(p))
    }

    pub fn add_child(&mut self, data: T) -> Node<T> {
    }

    pub fn children(&self) -> Children<T> {
        // return an iterator
    }

    pub fn siblings(&self) -> Option<Siblings<T>> {
        // return an iterator
        // how to disambiguate between myself and my siblings?
        self.parent().map(|p| p.children())
    }
}
