use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(value: i32, children: Vec<Rc<Node>>) -> Node {
        Node {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(children),
        }
    }
}

fn main() {
    let leaf = Rc::new(Node::new(3, vec![]));
    let branch = Rc::new(Node::new(3, vec![Rc::clone(&leaf)]));

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("{:#?}", branch);
    println!("PARENT: {:#?}", leaf.parent.borrow().upgrade())
}
