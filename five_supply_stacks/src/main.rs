use std::{cell::RefCell, rc::{Rc, Weak}};

type NodePtr = Rc<RefCell<Node>>;

#[derive(Debug)]
pub struct Node {
    pub value: char,
    pub next: Option<NodePtr>,
    pub prev: Option<Weak<RefCell<Node>>>,
}

impl Node {
    fn new(value: char) -> Self {
        Self { value, next: None, prev: None }
    }
}

impl From<Node> for Option<Rc<RefCell<Node>>> {
    fn from(node: Node) -> Self {
        Some(Rc::new(RefCell::new(node)))
    }
}

#[derive(Debug)]
pub struct List {
    pub head: Option<NodePtr>,
    pub tail: Option<NodePtr>,
}

impl List {
    fn new() -> Self {
        Self { head: None, tail: None }
    }

    fn push_back(&mut self, value: char) {
        let mut node = Node::new(value);

        match self.tail.take() {
            None => {
                self.head = node.into();
                self.tail = self.head.clone();
            },
            Some(current_tail) => {
                node.prev = Some(Rc::downgrade(&current_tail));
                self.tail = node.into();

                (*current_tail).borrow_mut().next = self.tail.clone();
            }
        }
    }

    fn pop(&mut self) -> Option<char> {
        match self.tail.take() {
            None => None,
            Some(tail) => {
                let tail_value = (*tail).borrow().value;
                let prev_to_tail = (*tail).borrow_mut().prev.take();

                match prev_to_tail {
                    None => {
                        self.head.take();
                    },
                    Some(prev) => {
                        let prev = prev.upgrade();

                        if let Some(prev) = prev {
                            (*prev).borrow_mut().next = None;
                            self.tail = Some(prev);
                        }
                    }
                }

                Some(tail_value)
            },
        }
    }

    fn shift(&mut self) -> Option<char> {
        match self.head.take() {
            None => None,
            Some(head) => {
                let head_value = (*head).borrow().value;
                let after_head = (*head).borrow_mut().next.take();

                match after_head {
                    None => {
                        self.tail.take();
                    },
                    Some(after_head) => { 
                        (*after_head).borrow_mut().prev = None;
                        self.head = Some(after_head);
                    },
                }

                Some(head_value)
            },
        }
    }
}

fn main() {
    let mut list = List::new();

    list.push_back('A');
    list.push_back('B');
    list.push_back('C');
    list.push_back('D');

    println!("{:?}", list);

    let res = list.shift();
    println!("{:?}", res);
    let res = list.shift();
    println!("{:?}", res);
    let res = list.shift();
    println!("{:?}", res);
    let res = list.shift();
    println!("{:?}", res);
    let res = list.shift();
    println!("{:?}", res);
    println!("{:?}", list);
}
