use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    // double bfs
    // O(n) time | O(n) space
    pub fn find_min_height_trees(
        n: i32,
        edges: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        if n == 1 {
            return vec![0];
        }

        let NOT_DEFINED: usize = 20001;
        let mut adjacent_to = vec![Vec::new(); n as usize];

        let start = edges[0][0] as usize;

        for edge in edges {
            adjacent_to[edge[0] as usize]
                .push(edge[1] as usize);
            adjacent_to[edge[1] as usize]
                .push(edge[0] as usize);
        }

        let mut q: VecDeque<(usize, usize, usize)> =
            VecDeque::from([(
                start,
                NOT_DEFINED,
                NOT_DEFINED,
            )]);
        let mut not_yet_visited = vec![true; n as usize];
        let mut furthest: usize = NOT_DEFINED;

        // first BFS
        while let Some((curr, _, _)) = q.pop_front() {
            if not_yet_visited[curr] {
                not_yet_visited[curr] = false;
                furthest = curr;
                for &nieghbour in adjacent_to[curr].iter() {
                    q.push_back((
                        nieghbour,
                        NOT_DEFINED,
                        NOT_DEFINED,
                    ));
                }
            }
        }

        for i in 0..n {
            not_yet_visited[i as usize] = true;
        }

        q.push_back((furthest, 0, NOT_DEFINED));
        let mut previous: Vec<usize> =
            vec![NOT_DEFINED; n as usize];
        let mut diameter: usize = 0;

        // second bfs
        while let Some((curr, dist, prev)) = q.pop_front() {
            if not_yet_visited[curr] {
                not_yet_visited[curr] = false;
                previous[curr] = prev;
                furthest = curr;
                diameter = dist;

                for &neighbour in adjacent_to[curr].iter() {
                    q.push_back((
                        neighbour,
                        dist + 1,
                        curr,
                    ));
                }
            }
        }

        // move towards the center
        let number_of_steps = diameter / 2;
        for i in 0..number_of_steps {
            furthest = previous[furthest];
        }

        // even, one center point
        if diameter % 2 == 0 {
            vec![furthest as i32]
        }
        // odd, two center points
        else {
            vec![furthest as i32, previous[furthest] as i32]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min_height_trees_case_1() {
        let n = 4;
        let edges =
            vec![vec![1, 0], vec![1, 2], vec![1, 3]];
        let expected = vec![1];
        assert_eq!(
            Solution::find_min_height_trees(n, edges),
            expected
        );
    }

    #[test]
    fn test_find_min_height_trees_case_2() {
        let n = 6;
        let edges = vec![
            vec![3, 0],
            vec![3, 1],
            vec![3, 2],
            vec![3, 4],
            vec![5, 4],
        ];
        let mut expected = vec![3, 4];
        // Note: The output order is not specified, so
        // sort the output if the function implementation
        // doesn't guarantee order
        let mut result =
            Solution::find_min_height_trees(n, edges);
        result.sort();
        expected.sort();
        assert_eq!(result, expected);
    }

    // Additional test to check behavior with no edges
    // (single node)
    #[test]
    fn test_find_min_height_trees_single_node() {
        let n = 1;
        let edges = vec![];
        let expected = vec![0]; // Assuming single node labeled as 0
        assert_eq!(
            Solution::find_min_height_trees(n, edges),
            expected
        );
    }

    // Test with a linear tree to check handling of
    // line-like structures
    #[test]
    fn test_find_min_height_trees_linear_tree() {
        let n = 5;
        let edges = vec![
            vec![0, 1],
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
        ];
        let expected = vec![2]; // The middle node in a line tree
        assert_eq!(
            Solution::find_min_height_trees(n, edges),
            expected
        );
    }
}
