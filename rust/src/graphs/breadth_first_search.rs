use std::collections::VecDeque;

pub struct Node {
    name: String,
    children: Vec<Box<Node>>,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, name: &str) -> &mut Self {
        self.children.push(Box::new(Node::new(name)));
        self
    }

    pub fn breadth_first_search(&self) -> Vec<String> {
        let mut queue = VecDeque::new();
        let mut array = Vec::new();

        queue.push_back(self);

        while let Some(current) = queue.pop_front() {
            array.push(current.name.clone());

            for child in &current.children {
                queue.push_back(child);
            }
        }

        array
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let mut root = Node::new("A");
        root.add_child("B").add_child("C");
        root.children[0].add_child("D");
        root.children[0].add_child("E");
        root.children[1].add_child("F");

        let expected_order =
            vec!["A", "B", "C", "D", "E", "F"];

        let result = root.breadth_first_search();

        assert_eq!(result, expected_order);
    }
}
