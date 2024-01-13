// An LRU (Least Recently Used) cache is a data
// structure that holds a fixed number of items,
// and when it reaches its capacity, it evicts the
// least recently used item before adding a new
// item. It's particularly useful in scenarios
// where you want to access data quickly without
// searching through a large dataset, such as
// caching pages in memory for a web server.

// Here's how an LRU cache typically works:

// Access: When you access an item, if the item is
// in the cache, it is considered a "hit", and it
// needs to be moved to the "most recently used"
// position. Update: If you update an item, it is
// also moved to the "most recently used"
// position. Insertion: When you insert a new item
// and the cache is not full, you add the item to
// the "most recently used" position. Eviction: If
// the cache is full and you insert a new item,
// you remove the item from the "least recently
// used" position and then insert the new item in
// the "most recently used" position.
// A doubly linked list is often used in the
// implementation of an LRU cache for several
// reasons:

// Efficiently Moving Nodes: The cache needs to
// frequently move nodes to the head (for most
// recently used) or remove the last node (for
// least recently used). A doubly linked list
// allows us to move nodes without a full
// traversal of the list. Constant Time
// Operations: The doubly linked list allows
// addition to the head or removal from the tail
// in constant time, which is crucial for
// high-performance caching. Easy Node Removal: If
// a node is accessed and it's already in the
// cache, it should be moved to the front (to mark
// it as the most recently used). In a doubly
// linked list, a node can easily remove itself
// without needing to traverse the list since it
// has a pointer to its previous node.

use std::{
    cell::RefCell, collections::HashMap, hash::Hash,
    rc::Weak,
};

use crate::doubly_linked_list::{List, Node};

pub struct LRU<K: Copy + Eq + Hash, T: Copy> {
    pub list: List<T>,
    pub map: HashMap<K, Weak<RefCell<Node<T>>>>,
    pub capacity: usize,
}

impl<K: Copy + Eq + Hash, T: Copy> LRU<K, T> {
    pub fn new() -> Self {
        LRU::with_capacity(10)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            list: List::new(),
            map: HashMap::new(),
            capacity,
        }
    }

    pub fn get(&mut self, k: K) -> Option<T> {
        let ptr = self.map.get_mut(&k);
        if ptr.is_none() {
            return None;
        }

        let ptr = ptr.unwrap();
        let ptr = ptr.upgrade();

        match ptr {
            None => None,
            Some(node) => {
                let value = node.borrow().value;
                self.list.move_node_to_back(node);
                Some(value)
            }
        }
    }

    pub fn put(&mut self, k: K, v: T) {
        let ptr = self.map.get_mut(&k);
        let ptr = if ptr.is_some() {
            ptr.unwrap().upgrade()
        } else {
            None
        };

        match ptr {
            None => {
                self.list.push_back(v);
                if let Some(tail) =
                    self.list.get_weak_tail()
                {
                    self.map.insert(k, tail);
                }

                if self.list.len() > self.capacity {
                    self.list.pop_front();
                }
            }
            Some(node) => {
                node.borrow_mut().value = v;
                self.list.move_node_to_back(node);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_builds_lru() {
        let mut lru = LRU::new();
        lru.put(1, "foo");
        lru.put(2, "bar");
        lru.put(3, "fizz");
        lru.put(4, "buzz");
        lru.put(5, "bazz");

        assert_eq!(lru.get(3), Some("fizz"));
        assert_eq!(lru.get(2), Some("bar"));

        let mut iter = lru.list.iter();
        assert_eq!(iter.next_back(), Some("bar"));
        assert_eq!(iter.next_back(), Some("fizz"));
        assert_eq!(iter.next_back(), Some("bazz"));
        assert_eq!(iter.next_back(), Some("buzz"));
        assert_eq!(iter.next_back(), Some("foo"));
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn works_builds_lru_capacity() {
        let mut lru = LRU::with_capacity(3);
        lru.put(1, "foo");
        lru.put(2, "bar");
        lru.put(3, "fizz");
        lru.put(4, "buzz");
        lru.put(5, "bazz");

        assert_eq!(lru.get(3), Some("fizz"));
        assert_eq!(lru.get(4), Some("buzz"));

        let mut iter = lru.list.iter();
        assert_eq!(iter.next_back(), Some("buzz"));
        assert_eq!(iter.next_back(), Some("fizz"));
        assert_eq!(iter.next_back(), Some("bazz"));
        assert_eq!(iter.next_back(), None);
    }
}
