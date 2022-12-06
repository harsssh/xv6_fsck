use std::cell::{RefCell};
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub parent: RefCell<Weak<Node<T>>>,
    pub children: RefCell<Vec<Rc<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(Vec::new()),
        }
    }

    pub fn set_parent(parent: &Rc<Node<T>>, child: &Rc<Node<T>>) {
        child.parent.replace(Rc::downgrade(parent));
    }

    pub fn add_child(parent: &Rc<Node<T>>, child: &Rc<Node<T>>) {
        parent.children.borrow_mut().push(Rc::clone(child));
    }

    pub fn set_relation(parent: &Rc<Node<T>>, child: &Rc<Node<T>>) {
        Node::set_parent(parent, child);
        Node::add_child(parent, child);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let root = Rc::new(Node::new(1));
        let child1 = Rc::new(Node::new(2));
        let child2 = Rc::new(Node::new(3));
        Node::set_relation(&root, &child1);
        Node::set_relation(&root, &child2);

        assert_eq!(root.children.borrow().len(), 2);
        assert_eq!(child1.parent.borrow().upgrade().unwrap().value, 1);
        assert_eq!(child2.parent.borrow().upgrade().unwrap().value, 1);
    }
}
