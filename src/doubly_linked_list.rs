
use std::rc::Rc;
use std::cell::RefCell;

struct Node<T: Copy> {
    data: T,
    previous: Option<Rc<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>
}

impl<T: Copy> Node<T> {
    fn new(data: T) -> Node<T> {
        Node {
            data: data,
            previous: None,
            next: None
        }
    }
}

pub struct DoublyLinkedList<T: Copy> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>
}

impl<T: Copy> DoublyLinkedList<T> {
    pub fn new() -> DoublyLinkedList<T> {
        DoublyLinkedList {
            head: None,
            tail: None
        }
    }

    pub fn add_at_head(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node::new(data)));
        match self.head.take() {
            Some(node) => {
                node.borrow_mut().previous = Some(new_node.clone());
                new_node.borrow_mut().next = Some(node.clone());
            },
            None => {
                self.tail = Some(new_node.clone());
            }
        }
        self.head = Some(new_node);
    }

    pub fn add_at_tail(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node::new(data)));
        match self.tail.take() {
            Some(node) => {
                node.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().previous = Some(node.clone());
            },
            None => {
                self.head = Some(new_node.clone());
            }
        }
        self.tail = Some(new_node);
    }

    pub fn remove_head(&mut self) -> Option<T> {
        let mut removed_data = None;
        let new_head = match self.head.take() {
            Some(node) => {
                removed_data = Some(node.borrow().data);
                node.borrow_mut().next.take()
            },
            None => None
        };
        self.head = new_head;
        removed_data
    }

    pub fn remove_tail(&mut self) -> Option<T> {
        let mut removed_data = None;
        let new_tail = match self.tail.take() {
            Some(node) => {
                removed_data = Some(node.borrow().data);
                node.borrow_mut().previous.take()
            },
            None => None
        };
        self.tail = new_tail;
        removed_data
    }

    pub fn head(&self) -> Option<T> {
        self.head.as_ref().map(|node| node.borrow().data)
    }

    pub fn tail(&self) -> Option<T> {
        self.tail.as_ref().map(|node| node.borrow().data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_at_head() {
        let mut list = DoublyLinkedList::new();
        list.add_at_head(1);
        assert_eq!(Some(1), list.head());
    }

    #[test]
    fn test_add_at_tail() {
        let mut list = DoublyLinkedList::new();
        list.add_at_tail(1);
        assert_eq!(Some(1), list.tail());
    }

    #[test]
    fn test_add_at_head_multiple() {
        let mut list = DoublyLinkedList::new();
        list.add_at_head(1);
        list.add_at_head(2);
        list.add_at_head(3);
        assert_eq!(Some(3), list.head());
    }

    #[test]
    fn test_add_at_tail_multiple() {
        let mut list = DoublyLinkedList::new();
        list.add_at_tail(1);
        list.add_at_tail(2);
        list.add_at_tail(3);
        assert_eq!(Some(3), list.tail());
    }

    #[test]
    fn test_add_at_head_and_tail() {
        let mut list = DoublyLinkedList::new();
        list.add_at_head(1);
        list.add_at_head(2);
        list.add_at_head(3);
        list.add_at_tail(4);
        list.add_at_tail(5);
        list.add_at_tail(6);
        assert_eq!(Some(3), list.head());
        assert_eq!(Some(6), list.tail());
    }

    #[test]
    fn test_remove_head() {
        let mut list = DoublyLinkedList::new();
        list.add_at_head(1);
        list.add_at_head(2);
        list.add_at_head(3);
        assert_eq!(Some(3), list.remove_head());
        assert_eq!(Some(2), list.head());
    }

    #[test]
    fn test_remove_tail() {
        let mut list = DoublyLinkedList::new();
        list.add_at_tail(1);
        list.add_at_tail(2);
        list.add_at_tail(3);
        assert_eq!(Some(3), list.remove_tail());
        assert_eq!(Some(2), list.tail());
    }

    #[test]
    fn test_remove_head_multiple() {
        let mut list = DoublyLinkedList::new();
        list.add_at_head(1);
        list.add_at_head(2);
        list.add_at_head(3);
        assert_eq!(Some(3), list.remove_head());
        assert_eq!(Some(2), list.remove_head());
        assert_eq!(Some(1), list.head());
    }

    #[test]
    fn test_remove_tail_multiple() {
        let mut list = DoublyLinkedList::new();
        list.add_at_tail(1);
        list.add_at_tail(2);
        list.add_at_tail(3);
        assert_eq!(Some(3), list.remove_tail());
        assert_eq!(Some(2), list.remove_tail());
        assert_eq!(Some(1), list.tail());
    }

    #[test]
    fn test_remove_head_and_tail() {
        let mut list = DoublyLinkedList::new();
        list.add_at_head(1);
        list.add_at_head(2);
        list.add_at_head(3);
        list.add_at_tail(4);
        list.add_at_tail(5);
        list.add_at_tail(6);
        assert_eq!(Some(3), list.remove_head());
        assert_eq!(Some(6), list.remove_tail());
        assert_eq!(Some(2), list.head());
        assert_eq!(Some(5), list.tail());
    }
}