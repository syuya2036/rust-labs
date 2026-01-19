use std::{cell::RefCell, rc::Rc};

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> List<T> {
    fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    fn push_front(&mut self, elem: T) {}

    fn push_back(&mut self, elem: T) {
        unimplemented!()
    }

    fn pop_front(&mut self) -> Option<T> {
        unimplemented!()
    }

    fn pop_back(&mut self) -> Option<T> {
        unimplemented!()
    }

    fn is_empty(&self) -> bool {
        if self.head.is_some() {
            assert!(self.tail.is_some());
            return true;
        }
        assert!(self.tail.is_none());
        false
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

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn test_push_front() {
        let mut lst = List::new();
        let elems = vec![1, 1, 2, 3, 5, 8, 13];
        for e in &elems {
            lst.push_front(e);
        }

        assert_eq!(lst.len(), elems.len());
    }
}
