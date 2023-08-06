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

    fn push(&mut self, value: char) {
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

    fn unshift(&mut self, value: char) {
        let mut new_node = Node::new(value);

        match self.head.take() {
            None => {
                self.head = new_node.into();
                self.tail = self.head.clone();
            },
            Some(current_head) => {
                new_node.next = Some(current_head.clone());
                self.head = new_node.into();

                if let Some(h) = &self.head {
                    (*current_head).borrow_mut().prev = Some(Rc::downgrade(h));
                }
            },
        };
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

    fn iter(&self) -> Iter {
        Iter { current: self.head.clone(), current_back: self.tail.clone() }
    }
}

pub struct Iter {
    current: Option<NodePtr>,
    current_back: Option<NodePtr>,
}

impl Iterator for Iter {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current.take() {
            None => None,
            Some(current) => {
                let node = (*current).borrow();
                self.current = node.next.clone();

                Some(node.value)
            },
        }
    }
}

impl DoubleEndedIterator for Iter {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.current_back.take() {
            None => None,
            Some(current_back) => {
                let node = (*current_back).borrow();

                if let Some(prev) = &node.prev {
                    let prev = prev.upgrade();
                    
                    self.current_back = prev;
                }

                Some(node.value)
            },
        }
    }
}

fn main() {
    let mut list = List::new();

    list.unshift('4');
    list.unshift('3');
    list.unshift('2');
    list.unshift('1');

    println!("list -> {:?}", list);

    let mut iter = list.iter();

    println!("next {:?}", iter.next());
    println!("next {:?}", iter.next());
    println!("next {:?}", iter.next());
    
    println!("back {:?}", iter.next_back());
    println!("back {:?}", iter.next_back());
    println!("back {:?}", iter.next_back());
    println!("back {:?}", iter.next_back());
    println!("back {:?}", iter.next_back());
                    
    println!("next {:?}", iter.next());
    println!("next {:?}", iter.next());

    println!("list -> {:?}", list);
}
