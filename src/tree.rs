use Set;

#[derive(Debug)]
struct Node {
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

    fn member(&self, x: i32) -> bool {
        if x == self.value {
            true
        } else if x < self.value {
            match self.left {
                Some(node) => node.member(x),
                None => false
            }
        } else {
            match self.right {
                Some(node) => node.member(x),
                None => false
            }
        }
    }

    fn predecessor(&self, x: i32) -> Option<i32> {
        // if the value >= x, it means that the predecessor must be
        // in the left sub-tree.
        if x <= self.value {
            match self.left {
                Some(node) => node.predecessor(x),
                None => None
            }
        } else {
            // if the value < x, the predecessor might be the value
            // , or the predecessor is in the right sub-tree.
            let right_result = match self.right {
                Some(node) => node.predecessor(x),
                None => None
            };

            match right_result {
                Some(val) => Some(val),
                None => Some(self.value)
            }
        }
    }

    fn rank(&self, x: i32) -> usize {
        let left_size = match self.left {
            Some(node) => node.size,
            None => 0
        };

        if x < self.value {
            left_size
        } else if x == self.value {
            left_size + 1
        } else {
            left_size + 1 + (match self.right {
                Some(node) => node.size,
                None => 0
            })
        }
    }

    fn select(&self, j: usize) -> Option<i32> {
        match self.left {
            Some(left_node) => {
                if j < left_node.size {
                    left_node.select(j)
                } else if j == left_node.size {
                    Some(self.value)
                } else {
                    match self.right {
                        Some(right_node) => right_node.select(j - left_node.size - 1),
                        None => None
                    }
                }
            },
            None => None
        }
    }

    fn insert(&mut self, x: i32) {
        if x < self.value {
            match self.left {
                Some(node) => node.insert(x),
                None => self.left = Node::new(x)
            };
            self.size += 1;
        } else {
            match self.right {
                Some(node) => node.insert(x),
                None => self.right = Node::new(x)
            };
            self.size += 1;
        }
    }

    fn delete_rightmost(self) -> Option<Box<Node>> {
        match self.right {
            Some(node) => {
                self.right = node.delete_rightmost();
                self.size -= 1;
                Some(Box::new(self))
            },
            None => match self.left {
                Some(node) => Some(node),
                None => None
            }
        }
    }

    // it returns the new root
    fn delete(self, x: i32) -> Option<Box<Node>> {
        if x < self.value {
            match self.left {
                Some(node) => {
                    self.left = node.delete(x);
                },
                None => {}
            }
            self.size -= 1;
            Some(Box::new(self))
        } else if x > self.value {
            match self.right {
                Some(node) => {
                    self.right = node.delete(x);
                },
                None => {}
            }
            self.size -= 1;
            Some(Box::new(self))
        } else {
            // Case 1: No left nor right => return None
            // Case 2: Has left but not right => return left
            // Case 3: Has right but not left => return right
            // Case 4: Has left and right => make the predecessor has new root

            if let Some(left) = self.left {
                if let Some(right) = self.right {
                    // Case 4
                    let mut new_root = left.delete_rightmost().unwrap();
                    new_root.size = left.size + right.size + 1;
                    new_root.left = Some(left);
                    new_root.right = Some(right);
                    Some(new_root)
                } else {
                    // Case 2
                    Some(left)
                }
            } else {
                if let Some(right) = self.right {
                    // Case 3
                    Some(right)
                } else {
                    // Case 1
                    None
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct BasicBinaryTree {
    root: Option<Box<Node>>
}

impl Set for BasicBinaryTree {
    // create a set
    fn new() -> Self {
        BasicBinaryTree {
            root: None
        }
    }

    // is x in the set
    fn member(&self, x: i32) -> bool {
        match self.root {
            Some(node) => node.member(x),
            None => false
        }
    }

    // the integer in the set that is just smaller than x
    fn predecessor(&self, x: i32) -> Option<i32> {
        match self.root {
            Some(node) => node.predecessor(x),
            None => None
        }
    }

    // # of integers in the set smaller than or equal to x
    fn rank(&self, x: i32) -> usize {
        match self.root {
            Some(node) => node.rank(x),
            None => 0
        }
    }

    // the j-th smallest integer in the set
    fn select(&self, j: usize) -> Option<i32> {
        match self.root {
            Some(node) => node.select(j),
            None => None
        }
    }

    // insert x into the set
    fn insert(&mut self, x: i32) {
        match self.root {
            Some(node) => node.insert(x),
            None => {
                self.root = Node::new(x);
            }
        }
    }

    // delete x in the set
    fn delete(&mut self, x: i32) {
        if let Some(root_node) = self.root {
            self.root = root_node.delete(x);
        }
    }
}
