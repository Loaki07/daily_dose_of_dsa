pub struct Solution;

const TRAVERSE_ALICE: i32 = 1;
const TRAVERSE_BOB: i32 = 2;
const TRAVERSE_BOTH: i32 = 3;

impl Solution {
    // Union find
    // O(edges.len) time | O(n) space
    pub fn max_num_edges_to_remove(
        n: i32,
        edges: Vec<Vec<i32>>,
    ) -> i32 {
        let n = n as usize;
        let mut uf_alice = UnionFind::new(n + 1);
        let mut uf_bob = UnionFind::new(n + 1);
        let mut removed = 0;

        for edge in &edges {
            let (type_, u, v) = (
                edge[0],
                edge[1] as usize,
                edge[2] as usize,
            );
            if type_ == TRAVERSE_BOTH {
                // Alice tests for redundant connections.
                let is_new_connection =
                    uf_alice.union(u, v);

                if is_new_connection {
                    // Connection is new, git it to Bob too.
                    uf_bob.union(u, v);
                } else {
                    // A redundant connection found. It can
                    // be removed.
                    removed += 1;
                }
            }
        }

        for edge in edges {
            let (type_, u, v) = (
                edge[0],
                edge[1] as usize,
                edge[2] as usize,
            );
            match type_ {
                TRAVERSE_ALICE => {
                    if !uf_alice.union(u, v) {
                        removed += 1; // Redundant
                    }
                }
                TRAVERSE_BOB => {
                    if !uf_bob.union(u, v) {
                        removed += 1 // Redundant
                    }
                }
                _ => (),
            }
        }

        // If both union find structures are fully
        // connected, the size of the cluster and
        // given vertex [1..n] isi on should be n.
        if uf_alice.size(1) == n && uf_bob.size(1) == n {
            removed
        } else {
            -1
        }
    }
}

struct UnionFind {
    link: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            link: (0..n).collect(),
            size: vec![1; n],
        }
    }

    /// Return the root of the cluster vertex v is
    /// on.
    fn find(&self, v: usize) -> usize {
        let mut v_root = v;
        while v_root != self.link[v_root] {
            v_root = self.link[v_root];
        }
        v_root
    }

    /// Return the size of the cluster that v is
    /// on.
    fn size(&self, v: usize) -> usize {
        let v_root = self.find(v);
        self.size[v_root]
    }

    /// Connect vertices u and v. Return true if
    /// this is a new connect or false
    /// otherwise.
    fn union(&mut self, u: usize, v: usize) -> bool {
        let u_root = self.find(u);
        let v_root = self.find(v);
        if u_root != v_root {
            if self.size[u_root] < self.size[v_root] {
                self.link[u_root] = v_root;
                self.size[v_root] += self.size[u_root];
            } else {
                self.link[v_root] = u_root;
                self.size[u_root] += self.size[v_root];
            }
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let n = 4;
        let edges = vec![
            vec![3, 1, 2],
            vec![3, 2, 3],
            vec![1, 1, 3],
            vec![1, 2, 4],
            vec![1, 1, 2],
            vec![2, 3, 4],
        ];
        assert_eq!(
            Solution::max_num_edges_to_remove(n, edges),
            2
        );
    }

    #[test]
    fn test_example_2() {
        let n = 4;
        let edges = vec![
            vec![3, 1, 2],
            vec![3, 2, 3],
            vec![1, 1, 4],
            vec![2, 1, 4],
        ];
        assert_eq!(
            Solution::max_num_edges_to_remove(n, edges),
            0
        );
    }

    #[test]
    fn test_example_3() {
        let n = 4;
        let edges = vec![
            vec![3, 2, 3],
            vec![1, 1, 2],
            vec![2, 3, 4],
        ];
        assert_eq!(
            Solution::max_num_edges_to_remove(n, edges),
            -1
        );
    }
}
