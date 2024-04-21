use std::collections::HashSet;

// Time complexity: O(Nα(N)+E)O(N \alpha(N) +
// E)O(Nα(N)+E), where N is the number of
// vertices, alpha(N) is the inverse Ackermann
// function (which grows very slowly), and E is
// the number of edges. The time complexity arises
// from performing union and find operations.
//
// Space complexity: O(N)O(N)O(N), where N is the
// number of vertices. We use additional space to
// store the parent array and rank array for each
// vertex in the UnionFind data structure.
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        let mut parent = vec![0; n];

        for i in 0..n {
            parent[i] = i;
        }

        Self {
            parent,
            rank: vec![0; n],
        }
    }

    pub fn find(&mut self, mut u: usize) -> usize {
        if self.parent[u] != u {
            self.parent[u] = self.find(self.parent[u]);
        }

        self.parent[u]
    }

    pub fn union_by_rank(&mut self, u: usize, v: usize) {
        let mut i = self.find(u);
        let mut j = self.find(v);

        if i == j {
            return;
        }

        if self.rank[i] < self.rank[j] {
            self.parent[i] = j;
        } else if self.rank[i] > self.rank[j] {
            self.parent[j] = i;
        } else {
            self.parent[i] = j;
            self.rank[j] += 1;
        }
    }
}

pub struct Solution;

impl Solution {
    // dfs
    // Time Complexity:    O(V + E) ~ O(`n` +
    // `_len_es`) Space Complexity:   O(V + E) ~
    // O(`n` + `_len_es`)
    pub fn valid_path(
        n: i32,
        edges: Vec<Vec<i32>>,
        source: i32,
        destination: i32,
    ) -> bool {
        use std::collections::HashSet;

        let _len_es = edges.len();
        let graph: Vec<HashSet<usize>> = {
            let mut graph: Vec<HashSet<usize>> =
                vec![HashSet::new(); n as usize];

            for edge in edges.iter() {
                let u = edge[0] as usize;
                let v = edge[1] as usize;

                graph[u].insert(v);
                graph[v].insert(u);
            }
            graph
        };

        let mut seen: HashSet<usize> =
            HashSet::with_capacity(n as usize);
        seen.insert(source as usize);
        Self::dfs(
            &mut seen,
            source as usize,
            destination as usize,
            &graph,
        )
    }

    fn dfs(
        seen: &mut HashSet<usize>,
        cur: usize,
        end: usize,
        graph: &Vec<HashSet<usize>>,
    ) -> bool {
        if cur == end {
            return true;
        }

        for &next in &graph[cur] {
            if seen.insert(next) {
                if Self::dfs(seen, next, end, graph) {
                    return true;
                }
            }
        }

        false
    }

    // bfs
    // Time Complexity:    O(V + E) ~ O(`n` +
    // `n_edges`) Space Complexity:   O(V + E) ~
    // O(`n` + `n_edges`)
    pub fn __valid_path(
        n: i32,
        edges: Vec<Vec<i32>>,
        source: i32,
        destination: i32,
    ) -> bool {
        use std::collections::{HashSet, VecDeque};

        let graph: Vec<HashSet<usize>> = {
            let mut graph: Vec<HashSet<usize>> =
                vec![HashSet::new(); n as usize];

            for edge in edges.into_iter() {
                let u = edge[0] as usize;
                let v = edge[1] as usize;

                graph[u].insert(v);
                graph[v].insert(u);
            }

            graph
        };

        let mut queue: VecDeque<usize> =
            VecDeque::with_capacity(n as usize);
        queue.push_back(source as usize);

        let mut seen: HashSet<usize> =
            HashSet::with_capacity(n as usize);
        seen.insert(source as usize);

        while !queue.is_empty() {
            let size = queue.len();

            for _ in 0..size {
                if let Some(cur) = queue.pop_front() {
                    if cur == destination as usize {
                        return true;
                    }

                    for &next in graph[cur].iter() {
                        if seen.insert(next) {
                            queue.push_back(next);
                        }
                    }
                }
            }
        }
        false
    }

    // Union Find
    pub fn _valid_path(
        n: i32,
        edges: Vec<Vec<i32>>,
        source: i32,
        destination: i32,
    ) -> bool {
        let n = n as usize;

        let mut uf = UnionFind::new(n);

        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;

            uf.union_by_rank(u, v);
        }

        uf.find(source as usize)
            == uf.find(destination as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_path_case_1() {
        let n = 3;
        let edges =
            vec![vec![0, 1], vec![1, 2], vec![2, 0]];
        let source = 0;
        let destination = 2;
        assert_eq!(
            Solution::valid_path(
                n,
                edges,
                source,
                destination
            ),
            true
        );
    }

    #[test]
    fn test_valid_path_case_2() {
        let n = 6;
        let edges = vec![
            vec![0, 1],
            vec![0, 2],
            vec![3, 5],
            vec![5, 4],
            vec![4, 3],
        ];
        let source = 0;
        let destination = 5;
        assert_eq!(
            Solution::valid_path(
                n,
                edges,
                source,
                destination
            ),
            false
        );
    }

    // Additional test to ensure handling of a graph
    // with no edges
    #[test]
    fn test_valid_path_no_edges() {
        let n = 3;
        let edges = vec![];
        let source = 0;
        let destination = 2;
        assert_eq!(
            Solution::valid_path(
                n,
                edges,
                source,
                destination
            ),
            false
        );
    }

    // Additional test to ensure handling when source
    // and destination are the same
    #[test]
    fn test_valid_path_same_source_and_destination() {
        let n = 1;
        let edges = vec![];
        let source = 0;
        let destination = 0;
        assert_eq!(
            Solution::valid_path(
                n,
                edges,
                source,
                destination
            ),
            true
        );
    }
}
