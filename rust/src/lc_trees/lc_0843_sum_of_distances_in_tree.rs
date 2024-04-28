pub struct Solution;

impl Solution {
    // O(n) time | O(n) space
    pub fn sum_of_distances_in_tree(
        n: i32,
        edges: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n = n as usize;
        let mut edge_map = vec![Vec::new(); n];

        for p in &edges {
            let (a, b) = (p[0] as usize, p[1] as usize);
            edge_map[a].push(b);
            edge_map[b].push(a);
        }

        let mut below_list = vec![(0, 0); n];
        Self::below(0, n, &edge_map, &mut below_list);

        let mut res = vec![0; n];
        Self::above(
            0,
            n,
            &edge_map,
            &mut res,
            (0, 0),
            &below_list,
        );
        res
    }

    pub fn below(
        node: usize,
        from: usize,
        edges: &Vec<Vec<usize>>,
        below: &mut Vec<(i32, i32)>,
    ) -> (i32, i32) {
        let mut res = (1, 0);

        for &nxt in &edges[node] {
            if nxt == from {
                continue;
            }
            let tmp = Self::below(nxt, node, edges, below);
            res.0 += tmp.0;
            res.1 += tmp.1 + tmp.0;
        }

        below[node] = res;
        res
    }

    pub fn above(
        node: usize,
        from: usize,
        edges: &Vec<Vec<usize>>,
        res: &mut Vec<i32>,
        abv: (i32, i32),
        below: &Vec<(i32, i32)>,
    ) {
        let abv = (abv.0, abv.0 + abv.1);
        res[node] = abv.1 + below[node].1;

        for &nxt in &edges[node] {
            if nxt == from {
                continue;
            }
            let msg = (
                abv.0 + below[node].0 - below[nxt].0,
                abv.1 + below[node].1
                    - below[nxt].0
                    - below[nxt].1,
            );
            Self::above(nxt, node, edges, res, msg, below);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_distances_in_tree_case_1() {
        let n = 6;
        let edges = vec![
            vec![0, 1],
            vec![0, 2],
            vec![2, 3],
            vec![2, 4],
            vec![2, 5],
        ];
        let expected = vec![8, 12, 6, 10, 10, 10];
        assert_eq!(
            Solution::sum_of_distances_in_tree(n, edges),
            expected
        );
    }

    #[test]
    fn test_sum_of_distances_in_tree_case_2() {
        let n = 1;
        let edges = vec![];
        let expected = vec![0];
        assert_eq!(
            Solution::sum_of_distances_in_tree(n, edges),
            expected
        );
    }

    #[test]
    fn test_sum_of_distances_in_tree_case_3() {
        let n = 2;
        let edges = vec![vec![1, 0]];
        let expected = vec![1, 1];
        assert_eq!(
            Solution::sum_of_distances_in_tree(n, edges),
            expected
        );
    }

    // Additional tests for complex scenarios
    #[test]
    fn test_sum_of_distances_in_tree_linear_tree() {
        let n = 4;
        let edges =
            vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        let expected = vec![6, 4, 4, 6]; // Linear tree
        assert_eq!(
            Solution::sum_of_distances_in_tree(n, edges),
            expected
        );
    }
}
