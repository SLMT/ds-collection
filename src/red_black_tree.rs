
use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;

use Set;

enum Color {
    RED,
    BLACK
}

// Rc provides a way to maintain multiple references (children and parent pointers) and
// shared ownership in the same time. However, an object referenced by Rc is immutable.
// RefCell is another good stuff to solve the problem. RefCell allows us to mutablly borrow
// an object through immutable reference, which could be a Rc.
type NodeLink = Option<Rc<RefCell<Node>>>;

struct Node {
    left: NodeLink,
    right: NodeLink,
    parent: NodeLink,
    value: i32,
    color: Color,
    size: usize // to speed up Select and Rank
}

impl Node {
    fn new(val: i32) -> Node {
        Node {
            left: None,
            right: None,
            parent: None,
            value: val,
            color: Color::RED,
            size: 1
        }
    }

    fn to_string(&self) -> String {
        let left = match self.left {
            Some(ref node) => node.borrow().to_string(),
            None => String::from("NIL")
        };
        let right = match self.right {
            Some(ref node) => node.borrow().to_string(),
            None => String::from("NIL")
        };

        match self.color {
            Color::RED => format!("{{ {} Red ({}) {} }}", left, self.value, right),
            Color::BLACK => format!("{{ {} Red ({}) {} }}", left, self.value, right)
        }
    }
}

pub struct RedBlackTree {
    root: NodeLink
}

impl RedBlackTree {
    pub fn new_test() -> RedBlackTree {
        let node_1 = Rc::new(RefCell::new(Node::new(1)));
        let node_2 = Rc::new(RefCell::new(Node::new(2)));
        let node_3 = Rc::new(RefCell::new(Node::new(3)));
        let node_4 = Rc::new(RefCell::new(Node::new(4)));
        let node_5 = Rc::new(RefCell::new(Node::new(5)));
        let node_6 = Rc::new(RefCell::new(Node::new(6)));
        let node_7 = Rc::new(RefCell::new(Node::new(7)));

        node_1.borrow_mut().color = Color::BLACK;
        node_2.borrow_mut().color = Color::BLACK;
        node_3.borrow_mut().color = Color::BLACK;
        node_4.borrow_mut().color = Color::BLACK;
        node_5.borrow_mut().color = Color::BLACK;
        node_6.borrow_mut().color = Color::BLACK;
        node_7.borrow_mut().color = Color::BLACK;

        node_1.borrow_mut().parent = Some(node_2.clone());
        node_2.borrow_mut().parent = Some(node_4.clone());
        node_3.borrow_mut().parent = Some(node_2.clone());
        node_5.borrow_mut().parent = Some(node_6.clone());
        node_6.borrow_mut().parent = Some(node_4.clone());
        node_7.borrow_mut().parent = Some(node_6.clone());

        node_2.borrow_mut().left = Some(node_1);
        node_2.borrow_mut().right = Some(node_3);
        node_6.borrow_mut().left = Some(node_5);
        node_6.borrow_mut().right = Some(node_7);
        node_4.borrow_mut().left = Some(node_2);
        node_4.borrow_mut().right = Some(node_6);

        RedBlackTree {
            root: Some(node_4)
        }
    }

    fn to_string(&self) {

    }
}

impl fmt::Display for RedBlackTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self.root {
            Some(ref node) => node.borrow().to_string(),
            None => String::from("NIL")
        })
    }
}

impl Set for RedBlackTree {
    // create a set
    fn new() -> Self {
        RedBlackTree {
            root: None
        }
    }

    // is x in the set
    fn member(&self, x: i32) -> bool {
        // TODO
        false
    }

    // the integer in the set that is just smaller than x
    fn predecessor(&self, x: i32) -> Option<i32> {
        // TODO
        None
    }

    // # of integers in the set smaller than or equal to x
    fn rank(&self, x: i32) -> usize {
        // TODO
        0
    }

    // the j-th smallest integer in the set
    fn select(&self, j: usize) -> Option<i32> {
        // TODO
        None
    }

    // insert x into the set
    fn insert(&mut self, x: i32) {
        let mut parent: NodeLink = None;
        let mut current: NodeLink = self.root.clone();

        // Find a proper position
        while current.is_some() {
            current = match current {
                Some(ref current_node) => {
                    parent = Some(current_node.clone());

                    if x < current_node.borrow().value {
                        current_node.borrow().left.clone()
                    } else {
                        current_node.borrow().right.clone()
                    }
                },
                None => None
            }
        }

        // Create a new node
        let new_node = Rc::new(RefCell::new(Node::new(x)));
        new_node.borrow_mut().parent = parent.clone();

        match parent {
            Some(ref parent_node) => {
                let parent_val = parent_node.borrow().value;

                if x < parent_val {
                    parent_node.borrow_mut().left = Some(new_node);
                } else {
                    parent_node.borrow_mut().right = Some(new_node);
                }
            },
            None => {
                self.root = Some(new_node);
            }
        }
    }

    // delete x in the set
    fn delete(&mut self, x: i32) {
        // TODO
    }
}
