
use std::rc::Rc;

use Set;

#[derive(Debug)]
enum Color {
    RED,
    BLACK
}

#[derive(Debug)]
struct Node {
    left: Option<Rc<Node>>,
    right: Option<Rc<Node>>,
    parent: Option<Rc<Node>>,
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
}

#[derive(Debug)]
pub struct RedBlackTree {
    root: Option<Rc<Node>>
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
        // TODO
    }

    // delete x in the set
    fn delete(&mut self, x: i32) {
        // TODO
    }
}
