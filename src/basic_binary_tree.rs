
use std::mem;

use Set;

#[derive(Debug)]
pub struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    value: i32,
    size: usize // to speed up Select and Rank
}

impl Node {
    fn new(x: i32) -> Option<Box<Node>> {
        Some(Box::new(Node {
            left: None,
            right:None,
            value: x,
            size: 1
        }))
    }

    fn predecessor(&self, x: i32) -> Option<i32> {
        // if the value >= x, it means that the predecessor must be
        // in the left sub-tree.
        if x <= self.value {
            match self.left {
                Some(ref node) => node.predecessor(x),
                None => None
            }
        } else {
            // if the value < x, the predecessor might be the value
            // , or the predecessor is in the right sub-tree.
            let right_result = match self.right {
                Some(ref node) => node.predecessor(x),
                None => None
            };

            match right_result {
                Some(val) => Some(val),
                None => Some(self.value)
            }
        }
    }

    fn rank(&self, x: i32) -> usize {
        if x < self.value {
            if let Some(ref left_node) = self.left {
                left_node.rank(x)
            } else {
                0
            }
        } else {
            let left_size = match self.left {
                Some(ref node) => node.size,
                None => 0
            };

            if x == self.value {
                left_size + 1
            } else {
                left_size + 1 + (match self.right {
                    Some(ref node) => node.rank(x),
                    None => 0
                })
            }
        }
    }

    fn select(&self, j: usize) -> Option<i32> {
        match self.left {
            Some(ref left_node) => {
                if j < left_node.size {
                    left_node.select(j)
                } else if j == left_node.size {
                    Some(self.value)
                } else {
                    match self.right {
                        Some(ref right_node) => right_node.select(j - left_node.size - 1),
                        None => None
                    }
                }
            },
            None => {
                if j == 0 {
                    Some(self.value)
                } else {
                    match self.right {
                        Some(ref right_node) => right_node.select(j - 1),
                        None => None
                    }
                }
            }
        }
    }
}

fn size(node_opt: &Option<Box<Node>>) -> usize {
    match *node_opt {
        Some(ref node) => node.size,
        None => 0
    }
}

fn delete_rightmost(node_opt: &mut Option<Box<Node>>) -> Option<Box<Node>> {
    if let Some(ref mut node) = *node_opt {
        if node.right.is_some() {
            node.size -= 1;
            return delete_rightmost(&mut node.right);
        } else if node.left.is_some() {
            node.size -= 1;
            return delete_rightmost(&mut node.left);
        }
    }

    if node_opt.is_some() {
        return node_opt.take();
    }

    return None;
}

pub type BasicBinaryTree = Option<Box<Node>>;

impl Set for BasicBinaryTree {
    // create a set
    fn new() -> Self {
        None
    }

    // is x in the set
    fn member(&self, x: i32) -> bool {
        match *self {
            Some(ref node) => {
                if x == node.value {
                    true
                } else if x < node.value {
                    node.left.member(x)
                } else {
                    node.right.member(x)
                }
            },
            None => false
        }
    }

    // the integer in the set that is just smaller than x
    fn predecessor(&self, x: i32) -> Option<i32> {
        match *self {
            Some(ref node) => node.predecessor(x),
            None => None
        }
    }

    // # of integers in the set smaller than or equal to x
    fn rank(&self, x: i32) -> usize {
        match *self {
            Some(ref node) => node.rank(x),
            None => 0
        }
    }

    // the j-th smallest integer in the set
    fn select(&self, j: usize) -> Option<i32> {
        match *self {
            Some(ref node) => node.select(j),
            None => None
        }
    }

    // insert x into the set
    fn insert(&mut self, x: i32) {
        match *self {
            Some(ref mut node) => {
                if x < node.value {
                    node.left.insert(x);
                    node.size += 1;
                } else {
                    node.right.insert(x);
                    node.size += 1;
                }
            },
            None => {
                mem::replace(self, Node::new(x));
            }
        }
    }

    // delete x in the set
    fn delete(&mut self, x: i32) {
        // Search the node
        if let Some(ref mut node) = *self {
            if x < node.value {
                node.left.delete(x);
                node.size -= 1;
                return;
            } else if x > node.value {
                node.right.delete(x);
                node.size -= 1;
                return;
            }
        } else {
            return;
        }

        // Found the node
        if let Some(mut node) = self.take() {
            // Case 1: No left nor right => directly delete itself
            // Case 2: Has left but not right => replace with left
            // Case 3: Has right but not left => replace with right
            // Case 4: Has left and right => make the predecessor has new root

            if node.left.is_some() {
                if node.right.is_some() {
                    // Case 4
                    let mut rightmost = delete_rightmost(&mut node.left).unwrap();
                    mem::replace(&mut rightmost.left, node.left.take());
                    mem::replace(&mut rightmost.right, node.right.take());
                    rightmost.size = size(&rightmost.left) + size(&rightmost.right) + 1;
                    mem::replace(self, Some(rightmost));
                } else {
                    // Case 2
                    mem::replace(self, node.left.take());
                }
            } else {
                if node.right.is_some() {
                    // Case 3
                    mem::replace(self, node.right.take());
                } else {
                    // Case 1
                    mem::replace(self, None);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use Set;
    use basic_binary_tree::BasicBinaryTree;

    #[test]
    fn test_member() {
        let set = create_testing_data();

        assert_eq!(set.member(3), true);
        assert_eq!(set.member(6), false);
    }

    #[test]
    fn test_predecessor() {
        let set = create_testing_data();

        assert_eq!(set.predecessor(1), None);
        assert_eq!(set.predecessor(5), Some(4));
    }

    #[test]
    fn test_rank() {
        let set = create_testing_data();

        assert_eq!(set.rank(1), 1);
        assert_eq!(set.rank(3), 3);
        assert_eq!(set.rank(6), 5);
    }

    #[test]
    fn test_select() {
        let set = create_testing_data();

        assert_eq!(set.select(0), Some(1));
        assert_eq!(set.select(3), Some(4));
    }

    #[test]
    fn test_delete1() {
        let mut set = create_testing_data();

        assert_eq!(set.select(2), Some(3));
        set.delete(3);
        assert_eq!(set.select(2), Some(4));
    }

    #[test]
    fn test_delete2() {
        let mut set = create_testing_data();

        assert_eq!(set.select(4), Some(5));
        set.delete(6);
        assert_eq!(set.select(4), Some(5));
    }

    fn create_testing_data() -> BasicBinaryTree {
        let mut set = BasicBinaryTree::new();

        // insert data
        set.insert(1);
        set.insert(5);
        set.insert(2);
        set.insert(4);
        set.insert(3);

        set
    }
}
