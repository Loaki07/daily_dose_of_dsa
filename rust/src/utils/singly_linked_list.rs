#[derive(Debug)]
pub struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

impl<T: Ord> LinkedList<T> {
    pub fn new() -> Self {
        Self(None)
    }

    fn push_front(&mut self, data: T) {
        let t = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(t))));
    }

    fn _push_back(&mut self, data: T) {
        match self.0 {
            Some((_, ref mut child)) => {
                child._push_back(data)
            }
            None => self.push_front(data),
        }
    }

    pub fn insert(&mut self, data: T) {
        match self.0 {
            Some((ref mut current, ref mut child)) => {
                if data < *current {
                    self.push_front(data);
                } else {
                    child.insert(data);
                }
            }
            None => {
                self.push_front(data);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let mut ll = LinkedList::new();
        ll.push_front(3);
        ll._push_back(12);
        ll.push_front(1);
        dbg!(&ll);
        assert!(true);
    }
}
