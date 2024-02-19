use std::{
    cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc,
};

pub struct LRUCache {
    map: HashMap<i32, Rc<RefCell<DListNode>>>,
    lru: DoubleDListNode,
    cap: usize,
}

#[derive(Debug)]
pub struct DListNode {
    key: i32,
    value: i32,
    prev: Option<Rc<RefCell<DListNode>>>,
    next: Option<Rc<RefCell<DListNode>>>,
}

impl DListNode {
    pub fn new(key: i32, value: i32) -> Self {
        Self {
            key,
            value,
            prev: None,
            next: None,
        }
    }
}

#[derive(Debug)]
pub struct DoubleDListNode {
    head: Option<Rc<RefCell<DListNode>>>,
    tail: Option<Rc<RefCell<DListNode>>>,
}

impl DoubleDListNode {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn get_head(
        &self,
    ) -> Option<Rc<RefCell<DListNode>>> {
        if self.head.is_none() {
            None
        } else {
            Some(self.head.as_ref().unwrap().clone())
        }
    }

    pub fn get_tail(
        &self,
    ) -> Option<Rc<RefCell<DListNode>>> {
        if self.tail.is_none() {
            None
        } else {
            Some(self.tail.as_ref().unwrap().clone())
        }
    }

    pub fn add_front(&mut self, key: i32, value: i32) {
        let node = Rc::new(RefCell::new(DListNode {
            key,
            value,
            prev: None,
            next: self.get_head(),
        }));
        self.head.replace(node);
    }

    pub fn add_back(&mut self, key: i32, value: i32) {
        let node = Rc::new(RefCell::new(DListNode {
            key,
            value,
            prev: self.get_tail(),
            next: None,
        }));
        self.tail.replace(node);
    }

    pub fn add_front_node(
        &mut self,
        node: Rc<RefCell<DListNode>>,
    ) {
        let head = self.get_head();

        if head.is_some() {
            head.as_ref().unwrap().borrow_mut().prev =
                Some(node.clone());
        }

        node.borrow_mut().prev = None;
        node.borrow_mut().next = head;

        self.head = Some(node);
    }

    pub fn add_back_node(
        &mut self,
        node: Rc<RefCell<DListNode>>,
    ) {
        let tail = self.get_tail();

        if tail.is_some() {
            tail.as_ref().unwrap().borrow_mut().next =
                Some(node.clone());
        }

        node.borrow_mut().prev = tail;
        node.borrow_mut().next = None;

        self.tail = Some(node);
    }

    pub fn remove(
        &mut self,
        target: Rc<RefCell<DListNode>>,
    ) {
        let prev = target.borrow().prev.clone();
        let next = target.borrow().next.clone();

        match (prev, next) {
            (Some(prev), Some(next)) => {
                prev.borrow_mut().next = Some(next.clone());
                next.borrow_mut().prev = Some(prev);
            }
            (Some(prev), None) => {
                // tail case
                prev.borrow_mut().next.take();
                self.tail.replace(prev);
            }
            (None, Some(next)) => {
                // head case
                next.borrow_mut().prev.take();
                self.head.replace(next);
            }
            (None, None) => {
                // single node case
                self.head.take();
                self.tail.take();
            }
        }
    }

    pub fn move_head(
        &mut self,
        target: Rc<RefCell<DListNode>>,
    ) {
        if !Rc::ptr_eq(
            self.get_head().as_ref().unwrap(),
            &target,
        ) {
            self.remove(target.clone());
            self.add_front_node(target);
        }
    }

    pub fn move_tail(
        &mut self,
        target: Rc<RefCell<DListNode>>,
    ) {
        if !Rc::ptr_eq(
            self.get_tail().as_ref().unwrap(),
            &target,
        ) {
            self.remove(target.clone());
            self.add_back_node(target);
        }
    }
}

/**
 * `&self` means the method takes an immutable
 * reference. If you need a mutable reference,
 * change it to `&mut self` instead.
 */
impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            map: HashMap::new(),
            lru: DoubleDListNode::new(),
            cap: capacity as usize,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if self.map.contains_key(&key) {
            let node = self.map.get(&key).unwrap();
            self.lru.move_head(node.clone());
            node.as_ref().borrow().value
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        let node = if self.map.contains_key(&key) {
            let node = self.map.get(&key).unwrap();
            node.borrow_mut().value = value;
            self.lru.remove(node.clone());
            self.lru.add_front_node(node.clone());
            node.clone()
        } else {
            let node = Rc::new(RefCell::new(
                DListNode::new(key, value),
            ));
            if self.map.len() == self.cap {
                let tail = self
                    .lru
                    .get_tail()
                    .as_ref()
                    .unwrap()
                    .clone();
                self.map
                    .remove(&tail.as_ref().borrow().key);
                self.lru.remove(tail);

                self.map.insert(key, node.clone());
                self.lru.add_front_node(node.clone());
            } else {
                self.map.insert(key, node.clone());
                self.lru.add_front_node(node.clone());
            }
            node
        };

        if self.lru.tail.is_none() {
            self.lru.add_back_node(node);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut lru_cache = LRUCache::new(2);
        lru_cache.put(1, 1); // cache is {1=1}
        lru_cache.put(2, 2); // cache is {1=1, 2=2}
        assert_eq!(lru_cache.get(1), 1); // returns 1
        lru_cache.put(3, 3); // evicts key 2, cache is {1=1, 3=3}
        assert_eq!(lru_cache.get(2), -1); // returns -1 (not found)
        lru_cache.put(4, 4); // evicts key 1, cache is {4=4, 3=3}
        assert_eq!(lru_cache.get(1), -1); // return -1 (not found)
        assert_eq!(lru_cache.get(3), 3); // return 3
        assert_eq!(lru_cache.get(4), 4); // return
                                         // 4
    }
}
