// Immutable outside, but can mutate interior.
use std::cell::RefCell;
// Reference counting pointer
// using weak reference here, to avoid cyclic
// condition where a node has a reference to
// itself (a circle loop where child is pointing
// to parent, and parent is pointing to child) https://doc.rust-lang.org/std/rc/struct.Weak.html
// we can drop the weak in such condtiions
use std::rc::{Rc, Weak};

type NodePtr<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug)]
pub struct Node<T: Copy> {
    pub value: T,
    pub next: Option<Rc<RefCell<Node<T>>>>,
    pub prev: Option<Weak<RefCell<Node<T>>>>,
}

impl<T: Copy> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            next: None,
            prev: None,
        }
    }
}

impl<T: Copy> From<Node<T>>
    for Option<Rc<RefCell<Node<T>>>>
{
    fn from(node: Node<T>) -> Self {
        Some(Rc::new(RefCell::new(node)))
    }
}

#[derive(Debug)]
pub struct List<T: Copy> {
    head: Option<NodePtr<T>>,
    tail: Option<NodePtr<T>>,
    count: usize,
}

impl<T: Copy> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            count: 0,
        }
    }

    pub fn push_front(&mut self, value: T) {
        let mut node = Node::new(value);

        match &mut self.head.take() {
            None => {
                self.head = node.into();
                self.tail = self.head.clone();
            }
            Some(current_head) => {
                node.next = Some(current_head.clone());
                self.head = node.into();
                if let Some(h) = &self.head {
                    current_head.borrow_mut().prev =
                        Some(Rc::downgrade(&h));
                }
            }
        }

        self.count += 1;
    }

    pub fn push_back(&mut self, value: T) {
        let mut node = Node::new(value);

        match &mut self.tail.take() {
            Some(current_tail) => {
                node.prev =
                    Some(Rc::downgrade(&current_tail));
                self.tail = node.into();
                current_tail.borrow_mut().next =
                    self.tail.clone();
            }
            None => {
                self.head = node.into();
                self.tail = self.head.clone();
            }
        }

        self.count += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match &mut self.head.take() {
            None => None,
            Some(head) => {
                let mut head = head.borrow_mut();
                let next = head.next.take();

                match next {
                    None => {
                        self.tail.take();
                    }
                    Some(next) => {
                        next.borrow_mut().prev = None;
                        self.head = Some(next);
                    }
                }

                self.count -= 1;
                Some(head.value)
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        match &mut self.tail.take() {
            Some(tail) => {
                let mut tail = tail.borrow_mut();
                let prev = tail.prev.take();

                match prev {
                    Some(prev) => {
                        let prev = prev.upgrade();
                        if let Some(prev) = prev {
                            prev.borrow_mut().next = None;
                            self.tail = Some(prev);
                        }
                    }
                    None => {
                        self.head.take();
                    }
                };

                self.count -= 1;
                Some(tail.value)
            }
            None => None,
        }
    }

    pub fn iter(&self) -> ListIterator<T> {
        ListIterator {
            current: self.head.clone(),
            current_back: self.tail.clone(),
        }
    }

    pub fn remove_node(&mut self, node: &mut NodePtr<T>) {
        // borrow mut on node
        // take prev and next
        // match prev, next conditions
        // (set head tail)
        // (route prev.next -> next)
        // (route next.prev -> prev)

        let (prev, next) = {
            let mut node = node.borrow_mut();
            let prev = match node.prev.take() {
                None => None,
                Some(prev) => prev.upgrade(),
            };

            let next = node.next.take();
            (prev, next)
        };

        match (prev, next) {
            (None, None) => {
                self.head = None;
                self.tail = None;
            }
            // we are at the head
            (None, Some(next)) => {
                next.borrow_mut().prev = None;
                self.head.replace(next);
            }
            // we are at the tail
            (Some(prev), None) => {
                prev.borrow_mut().next = None;
                self.tail.replace(prev);
            }
            (Some(prev), Some(next)) => {
                next.borrow_mut()
                    .prev
                    .replace(Rc::downgrade(&prev));
                prev.borrow_mut().next.replace(next);
            }
        }
    }

    pub fn move_node_to_back(
        &mut self,
        mut node: NodePtr<T>,
    ) {
        // remove node first
        // push node to back
        self.remove_node(&mut node);
        self.push_node_back(node);
    }

    pub fn push_node_back(&mut self, node: NodePtr<T>) {
        // take existing tail
        // assign head if needed if tail already none
        // route existing-tail.next -> node
        // node.prev -> existing-tail
        // se tail to node
        match self.tail.take() {
            // we are at the head
            None => {
                self.head.replace(node);
                self.tail = self.head.clone();
            }
            Some(current_tail) => {
                node.borrow_mut()
                    .prev
                    .replace(Rc::downgrade(&current_tail));
                self.tail.replace(node);
                current_tail.borrow_mut().next =
                    self.tail.clone();
            }
        }
    }

    pub fn get_weak_tail(
        &self,
    ) -> Option<Weak<RefCell<Node<T>>>> {
        match &self.tail {
            None => None,
            Some(tail) => Some(Rc::downgrade(tail)),
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }
}

impl<T: Copy> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_back() {}
    }
}

pub struct ListIterator<T: Copy> {
    current: Option<NodePtr<T>>,
    current_back: Option<NodePtr<T>>,
}

impl<T: Copy> Iterator for ListIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.take();
        if current.is_none() {
            return None;
        }

        let current = current.unwrap();
        let current = current.borrow();
        self.current = current.next.clone();
        Some(current.value)
    }
}

impl<T: Copy> DoubleEndedIterator for ListIterator<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let current = self.current_back.take();
        if current.is_none() {
            return None;
        }

        let current = current.unwrap();
        let current = current.borrow();

        match &current.prev {
            Some(prev) => {
                self.current_back = prev.upgrade();
                Some(current.value)
            }
            None => Some(current.value),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn doubly_linked_list_construction_back() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        // dbg!(&list);

        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn doubly_linked_list_construction_front() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_front(4);
        // dbg!(&list);

        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn works_buuld_list_iter() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_front(4);

        for (i, j) in list.iter().zip(list.iter().rev()) {
            // dbg!(i, j);
        }

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(2));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), Some(3));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next_back(), Some(4));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }
}
