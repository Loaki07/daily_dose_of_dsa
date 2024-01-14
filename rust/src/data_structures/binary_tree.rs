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

#[derive(Debug)]
pub struct LevelTraversal<'a> {
    node: Option<&'a Box<Node>>,
    queue: VecDeque<&'a Box<Node>>,
}

impl<'a> LevelTraversal<'a> {
    fn new(node: Option<&'a Box<Node>>) -> Self {
        Self {
            node,
            queue: VecDeque::<&'a Box<Node>>::new(),
        }
    }
}

impl<'a> Iterator for LevelTraversal<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.node, &mut self.queue) {
            (None, q) if q.is_empty() => None,
            (None, q) => {
                self.node = q.pop_front();
                self.next()
            }
            (Some(node), q) => {
                if let Some(ref left) = node.left {
                    q.push_back(left);
                }
                if let Some(ref right) = node.right {
                    q.push_back(right);
                }
                self.node = None;
                Some(node.value)
            }
        }
    }
}

#[derive(Debug)]
pub struct InorderTraversal<'a> {
    node: Option<&'a Box<Node>>,
    queue: Vec<&'a Box<Node>>,
}

impl<'a> InorderTraversal<'a> {
    fn new(node: Option<&'a Box<Node>>) -> Self {
        Self {
            node,
            queue: Vec::new(),
        }
    }
}

impl<'a> Iterator for InorderTraversal<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.node, &mut self.queue) {
            (None, q) if q.is_empty() => None,
            (None, q) => {
                let node = q.pop().unwrap();
                self.node = node.right.as_ref();
                Some(node.value)
            }
            (Some(node), q) => {
                q.push(node);
                self.node = node.left.as_ref();
                self.next()
            }
        }
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

    fn level_iter(&self) -> LevelTraversal {
        LevelTraversal::new(self.root.as_ref())
    }

    fn inorder_iter(&self) -> InorderTraversal {
        InorderTraversal::new(self.root.as_ref())
    }

    // DFS explores as far as possible along each
    // branch before backtracking. It dives deep into
    // the tree structure, visiting children nodes
    // before visiting sibling nodes. DFS can take
    // three forms in a binary tree, each differing in
    // the order they visit the nodes:

    // 1. Pre-order Traversal: Visits the root, then
    //    the
    // left subtree, and finally the right subtree.
    // 2. In-order Traversal: Visits the left subtree,
    // then the root, and finally the right subtree.
    // This is the one we're discussing.
    // 3. Post-order Traversal: Visits the left
    //    subtree,
    // then the right subtree, and finally the root.
    fn traverse_inorder_recursive(
        values: &mut Vec<i32>,
        node: &Box<Node>,
    ) {
        if let Some(ref left) = node.left {
            Tree::traverse_inorder_recursive(values, left);
        }
        values.push(node.value);

        if let Some(ref right) = node.right {
            Tree::traverse_inorder_recursive(values, right);
        }
    }

    fn traverse_inorder_iterative(&self) -> Vec<i32> {
        if self.root.is_none() {
            return Vec::new();
        }

        let mut result: Vec<i32> = Vec::new();
        let mut q: Vec<&Box<Node>> = Vec::new();
        let mut current = self.root.as_ref();

        while !q.is_empty() || current.is_some() {
            while let Some(node) = current {
                q.push(node);
                current = node.left.as_ref();
            }

            if let Some(node) = q.pop() {
                result.push(node.value);
                current = node.right.as_ref();
            }
        }

        result
    }

    fn inorder(&self) -> Vec<i32> {
        if self.root.is_none() {
            return Vec::new();
        }

        let mut results: Vec<i32> = Vec::new();
        if let Some(ref root) = self.root {
            Tree::traverse_inorder_recursive(
                &mut results,
                root,
            );
        }
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

    #[test]
    fn works_builds_level_traversal_iter_tree() {
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

        for v in tree.level_iter() {
            // dbg!(&v);
        }

        let values: Vec<i32> = tree.level_iter().collect();
        assert_eq!(
            values,
            vec![8, 3, 10, 1, 6, 14, 4, 7, 13]
        );
    }

    #[test]
    fn works_builds_inorder_traversal_iter_tree() {
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

        for v in tree.inorder_iter() {
            // dbg!(&v);
        }

        let values: Vec<i32> =
            tree.inorder_iter().collect();
        assert_eq!(
            values,
            vec![1, 3, 4, 6, 7, 8, 10, 13, 14]
        );
    }

    #[test]
    fn works_builds_tree_inorder() {
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
            tree.inorder(),
            vec![1, 3, 4, 6, 7, 8, 10, 13, 14]
        );
    }

    #[test]
    fn works_builds_tree_iterative() {
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
            tree.traverse_inorder_iterative(),
            vec![1, 3, 4, 6, 7, 8, 10, 13, 14]
        );
    }
}
