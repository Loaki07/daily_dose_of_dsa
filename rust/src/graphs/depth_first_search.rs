#[derive(Debug)]
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

    pub fn add_child(&mut self, name: &str) {
        self.children.push(Box::new(Node::new(name)));
    }

    pub fn depth_first_search(
        &self,
        array: &mut Vec<String>,
    ) {
        array.push(self.name.clone());
        for child in self.children.iter() {
            child.depth_first_search(array)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_depth_first_search() {
        // Create the root node and its children
        let mut root = Node::new("A");
        root.add_child("B");
        root.add_child("C");
        root.children[0].add_child("D"); // Add child D to B
        root.children[0].add_child("E"); // Add child E to B
        root.children[1].add_child("F"); // Add child F to C

        // Expected order of nodes after a depth-first
        // search
        let expected_order =
            vec!["A", "B", "D", "E", "C", "F"];

        // Perform the depth-first search
        let mut result = Vec::new();
        root.depth_first_search(&mut result);

        // Assert that the result matches the expected
        // order
        assert_eq!(result, expected_order, "Depth first search did not match the expected order.");
    }
}
