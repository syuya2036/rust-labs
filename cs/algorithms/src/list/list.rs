use std::{cell::RefCell, default, rc::Rc};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

pub struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Node {
            elem,
            next: None,
            prev: None,
        }
    }

    fn new_shared(elem: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node::new(elem)))
    }
}

impl<T> List<T> {
    fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    fn push_front(&mut self, elem: T) {
        let new_node = Node::new_shared(elem);
        if let Some(head) = self.head.take() {
            head.borrow_mut().prev = Some(new_node.clone());
            new_node.borrow_mut().next = Some(head);
            self.head = Some(new_node);
        } else {
            // headがない場合
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
        }
    }

    fn push_back(&mut self, elem: T) {
        let new_node = Node::new_shared(elem);
        if let Some(tail) = self.tail.take() {
            tail.borrow_mut().next = Some(new_node.clone());
            new_node.borrow_mut().prev = Some(tail);
            self.tail = Some(new_node);
        } else {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
        }
    }

    fn pop_front(&mut self) -> Option<T> {
        if let Some(head) = self.head.take() {
            if let Some(new_head) = head.borrow_mut().next.take() {
                new_head.borrow_mut().prev = None;
                self.head = Some(new_head);
            } else {
                self.tail.take();
            }
            Some(Rc::try_unwrap(head).ok().unwrap().into_inner().elem)
        } else {
            None
        }
    }

    fn pop_back(&mut self) -> Option<T> {
        if let Some(tail) = self.tail.take() {
            if let Some(new_tail) = tail.borrow_mut().prev.take() {
                new_tail.borrow_mut().next = None;
                self.tail = Some(new_tail);
            } else {
                // 要素1個
                self.head.take();
            }
            Some(Rc::try_unwrap(tail).ok().unwrap().into_inner().elem)
        } else {
            None
        }
    }

    fn is_empty(&self) -> bool {
        if self.head.is_some() {
            assert!(self.tail.is_some());
            return false;
        } else {
            assert!(self.tail.is_none());
            return true;
        }
    }

    fn len(&self) -> usize {
        let mut now_node = self.head.clone();
        let mut len = 0;
        while let Some(node) = now_node {
            len += 1;
            now_node = node.borrow().next.clone();
        }
        len
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn test_front() {
        let mut lst = List::new();
        let elems = vec![1, 1, 2, 3, 5, 8, 13];
        for e in elems.clone() {
            lst.push_front(e);
        }

        for i in 0..lst.len() {
            assert_eq!(lst.pop_front().unwrap(), elems[elems.len() - i - 1])
        }
    }

    #[test]
    fn test_back() {
        let mut lst = List::new();
        let elems = vec![1, 1, 2, 3, 5, 8, 13];
        for e in elems.clone() {
            lst.push_back(e);
        }

        for i in 0..elems.len() {
            assert_eq!(lst.pop_back().unwrap(), elems[elems.len() - i - 1]);
        }
    }

    #[test]
    fn test_mixed_ops() {
        let mut lst = List::new();

        lst.push_front(2); // [2]
        lst.push_front(1); // [1,2]
        assert_eq!(lst.len(), 2);
        lst.push_back(3); // [1,2,3]
        lst.push_back(4); // [1,2,3,4]
        assert_eq!(lst.len(), 4);
        assert!(!lst.is_empty());

        assert_eq!(lst.pop_front(), Some(1)); // [2,3,4]
        assert_eq!(lst.pop_back(), Some(4)); // [2,3]
        assert_eq!(lst.len(), 2);
        assert_eq!(lst.pop_back(), Some(3)); // [2]
        assert_eq!(lst.pop_front(), Some(2)); // []

        assert!(lst.is_empty());
        assert_eq!(lst.pop_front(), None);
        assert_eq!(lst.pop_back(), None);
    }
}
