// A binary tree is a fundamental data structure
// in computer science and programming, which
// consists of nodes, each having up to two
// children. Here are the key features and
// concepts related to binary trees:

// 1. **Node Structure**: Each node in a binary
//    tree typically contains three components:
//    - **Data**: The value or the payload that
//      the node stores.
//    - **Left Child**: A reference or pointer to
//      the left child node.
//    - **Right Child**: A reference or pointer to
//      the right child node.

// 2. **Properties**:
//    - **Root**: The topmost node in a binary
//      tree is called the root.
//    - **Children**: Nodes linked directly under
//      another node are its children.
//    - **Parent**: The converse notion of a
//      child. Every node (except the root) has
//      one parent.
//    - **Leaf Nodes**: Nodes that do not have any
//      children are known as leaves or leaf
//      nodes.
//    - **Depth**: The length of the path from the
//      root to the node.
//    - **Height**: The length of the longest path
//      from the node to a leaf. The height of the
//      tree is the height of the root.

// 3. **Types of Binary Trees**:
//    - **Full Binary Tree**: Every node other
//      than the leaves has two children.
//    - **Complete Binary Tree**: All levels are
//      completely filled except possibly the last
//      level, which is filled from left to right.
//    - **Perfect Binary Tree**: A binary tree in
//      which all interior nodes have two children
//      and all leaves have the same depth or same
//      level.
//    - **Balanced Binary Tree**: A binary tree
//      where the depth of all leaf nodes or nodes
//      with two children differ by no more than
//      1. AVL trees and Red-Black trees are
//      examples of balanced binary trees.
//    - **Binary Search Tree (BST)**: A special
//      kind of binary tree where for each node,
//      the left children are less than the node
//      and the right children are greater.

// 4. **Traversal Methods**:
//    - **In-Order Traversal**: Visit left
//      subtree, current node, then right subtree.
//    - **Pre-Order Traversal**: Visit current
//      node, then left subtree, then right
//      subtree.
//    - **Post-Order Traversal**: Visit left

// subtree, right subtree, then current node.
//    - **Level-Order Traversal (or Breadth-First
//      Search)**: Visit nodes level by level from
//      root.

// 5. **Applications**:
//    - Binary Trees are used in many areas of
//      computer science, including expression
//      parsing, search algorithms (like binary
//      search trees), sorting algorithms,
//      representing hierarchical data, and more.
//    - They are the basis for more complex tree
//      structures like AVL trees, Red-Black
//      trees, Heaps, Syntax Trees, etc.

// 6. **Efficiency**:
//    - The efficiency of operations like
//      searching, insertion, and deletion in a
//      binary tree depends significantly on its
//      height. In the worst case (like a linear
//      chain), these operations can take O(n)
//      time, where n is the number of nodes. For
//      a balanced tree, they can be performed in
//      O(log n) time.

// Binary trees are fundamental because they
// provide a basis for more advanced tree-based
// data structures and algorithms, offering
// efficient ways to store and retrieve data. The
// concept of branching in two directions at each
// node makes binary trees an extremely versatile
// and widely used data structure.

use std::collections::VecDeque;

#[derive(Debug)]
pub struct Tree {
    root: Option<Box<Node>>,
}

#[derive(Debug)]
pub struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}

impl From<Node> for Option<Box<Node>> {
    fn from(node: Node) -> Self {
        Some(Box::new(node))
    }
}

impl Tree {
    fn new() -> Self {
        Self { root: None }
    }

    // bfs - iterative
    fn traverse_level(&self) -> Vec<i32> {
        if self.root.is_none() {
            return Vec::new();
        }

        let mut results: Vec<i32> = Vec::new();
        let mut q: VecDeque<&Box<Node>> = VecDeque::new();
        let root = self.root.as_ref().unwrap();
        results.push(root.value);
        q.push_back(root);

        let mut height = 0;
        while !q.is_empty() {
            for _ in 0..q.len() {
                if let Some(node) = q.pop_front() {
                    if let Some(ref left) = node.left {
                        results.push(left.value);
                        q.push_back(left);
                    }
                    if let Some(ref right) = node.right {
                        results.push(right.value);
                        q.push_back(right);
                    }
                }
            }
            height += 1;
        }
        // dbg!(&height);
        results
    }

    fn insert(&mut self, value: i32) {
        // use when using insert_recursive
        // match &mut self.root {
        //     None => {
        //         self.root = Node::new(value).into();
        //     }
        //     Some(node) => {
        //         Tree::insert_recursive(node, value);
        //     }
        // }
        self.insert_iterative(value);
    }

    fn insert_iterative(&mut self, value: i32) {
        if self.root.is_none() {
            self.root = Node::new(value).into();
            return;
        }

        let mut q: Vec<&mut Box<Node>> = Vec::new();
        let root = self.root.as_mut().unwrap();
        q.push(root);

        while let Some(node) = q.pop() {
            if value > node.value {
                match node.right {
                    ref mut right @ None => {
                        *right = Node::new(value).into();
                    }
                    Some(ref mut n) => {
                        q.push(n);
                    }
                }
            } else if value < node.value {
                let left = &mut node.left;
                match left {
                    None => {
                        *left = Node::new(value).into();
                    }
                    Some(n) => {
                        q.push(n);
                    }
                }
            }
        }
    }

    fn insert_recursive(node: &mut Box<Node>, value: i32) {
        if value > node.value {
            match &mut node.right {
                None => {
                    node.right = Node::new(value).into();
                }
                Some(n) => {
                    Tree::insert_recursive(n, value);
                }
            }
        } else if value < node.value {
            match &mut node.left {
                None => {
                    node.left = Node::new(value).into();
                }
                Some(n) => {
                    Tree::insert_recursive(n, value);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_builds_tree() {
        let mut tree = Tree::new();
        tree.insert(8);
        tree.insert(10);
        tree.insert(3);
        tree.insert(1);
        tree.insert(6);
        tree.insert(4);

        // dbg!(&tree);
        assert_eq!(tree.root.is_some(), true);
    }

    #[test]
    fn works_builds_level_traversal_tree() {
        let mut tree = Tree::new();
        tree.insert(8);
        tree.insert(10);
        tree.insert(3);
        tree.insert(1);
        tree.insert(6);
        tree.insert(4);
        tree.insert(7);
        tree.insert(14);
        tree.insert(13);

        assert_eq!(
            tree.traverse_level(),
            vec![8, 3, 10, 1, 6, 14, 4, 7, 13]
        );
    }
}
