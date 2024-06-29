use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    // bfs
    // O(v(v+e)) time || O(v^2) space
    pub fn get_ancestors(
        n: i32,
        edges: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut graph = vec![None; n];
        let mut result = vec![vec![]; n];
        let mut seen = vec![false; n];
        let mut queue = VecDeque::new();

        for edge in edges {
            let from = edge[0] as usize;
            let to = edge[1] as usize;
            graph[from]
                .get_or_insert_with(Vec::new)
                .push(to);
        }

        for v in 0..n {
            queue.push_back(v);
            while let Some(u) = queue.pop_front() {
                if let Some(adj) = &graph[u] {
                    for &u in adj {
                        if !seen[u] {
                            seen[u] = true;
                            result[u].push(v as i32);
                            queue.push_back(u);
                        }
                    }
                }
            }
            seen.fill(false);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let n = 8;
        let edges = vec![
            vec![0, 3],
            vec![0, 4],
            vec![1, 3],
            vec![2, 4],
            vec![2, 7],
            vec![3, 5],
            vec![3, 6],
            vec![3, 7],
            vec![4, 6],
        ];

        let expected = vec![
            vec![],
            vec![],
            vec![],
            vec![0, 1],
            vec![0, 2],
            vec![0, 1, 3],
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 2, 3],
        ];

        let result = Solution::get_ancestors(n, edges);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example2() {
        let n = 5;
        let edges = vec![
            vec![0, 1],
            vec![0, 2],
            vec![0, 3],
            vec![0, 4],
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 3],
            vec![2, 4],
            vec![3, 4],
        ];

        let expected = vec![
            vec![],
            vec![0],
            vec![0, 1],
            vec![0, 1, 2],
            vec![0, 1, 2, 3],
        ];

        let result = Solution::get_ancestors(n, edges);
        assert_eq!(result, expected);
    }
}
