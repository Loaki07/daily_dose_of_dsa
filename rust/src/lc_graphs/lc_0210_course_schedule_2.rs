pub struct Solution;

impl Solution {
    // O(n * E) time | O(n) space
    pub fn find_order(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let num = num_courses as usize;
        let mut graph = vec![vec![]; num];

        for prerequisite in prerequisites {
            let x = prerequisite[0] as usize;
            let y = prerequisite[1] as usize;
            graph[x].push(y);
        }

        let mut res = Vec::new();
        let mut visited = vec![0; num];
        for i in 0..num {
            if !Self::dfs(&graph, i, &mut visited, &mut res)
            {
                return vec![];
            }
        }
        res
    }

    fn dfs(
        graph: &[Vec<usize>],
        idx: usize,
        visited: &mut Vec<i32>,
        res: &mut Vec<i32>,
    ) -> bool {
        if visited[idx] == -1 {
            return false;
        }

        if visited[idx] == 1 {
            return true;
        }

        visited[idx] = -1;
        for &next_idx in &graph[idx] {
            if !Self::dfs(graph, next_idx, visited, res) {
                return false;
            }
        }
        res.push(idx as i32);
        visited[idx] = 1;

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let num_courses = 2;
        let prerequisites = vec![vec![1, 0]];
        let expected = vec![0, 1];
        assert_eq!(
            Solution::find_order(
                num_courses,
                prerequisites
            ),
            expected
        );
    }

    #[test]
    fn test_example2() {
        let num_courses = 4;
        let prerequisites = vec![
            vec![1, 0],
            vec![2, 0],
            vec![3, 1],
            vec![3, 2],
        ];

        let mut result = Solution::find_order(
            num_courses,
            prerequisites,
        );
        result.sort_unstable();
        let mut expected = vec![0, 2, 1, 3];
        expected.sort_unstable();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example3() {
        let num_courses = 1;
        let prerequisites = vec![];
        let expected = vec![0];
        assert_eq!(
            Solution::find_order(
                num_courses,
                prerequisites
            ),
            expected
        );
    }
}
