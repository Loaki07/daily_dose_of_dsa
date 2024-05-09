use std::collections::HashSet;

pub struct Solution;

impl Solution {
    // O(v + e) time v is vertices | O(v) space v is
    // visited
    pub fn find_circle_num(
        is_connected: Vec<Vec<i32>>,
    ) -> i32 {
        let mut visited: HashSet<i32> = HashSet::new();
        let mut provinces: i32 = 0;

        for node in 0..is_connected.len() {
            if !visited.contains(&(node as i32)) {
                Self::dfs(
                    &is_connected,
                    node as i32,
                    &mut visited,
                );
                provinces += 1;
            }
        }

        provinces
    }

    fn dfs(
        is_connected: &Vec<Vec<i32>>,
        node: i32,
        visited: &mut HashSet<i32>,
    ) {
        visited.insert(node);

        for (adj_node, &edge) in
            is_connected[node as usize].iter().enumerate()
        {
            if edge == 1 {
                if !visited.contains(&(adj_node as i32)) {
                    Self::dfs(
                        is_connected,
                        adj_node as i32,
                        visited,
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let is_connected = vec![
            vec![1, 1, 0],
            vec![1, 1, 0],
            vec![0, 0, 1],
        ];
        assert_eq!(
            Solution::find_circle_num(is_connected),
            2
        );
    }

    #[test]
    fn test_example2() {
        let is_connected = vec![
            vec![1, 0, 0],
            vec![0, 1, 0],
            vec![0, 0, 1],
        ];
        assert_eq!(
            Solution::find_circle_num(is_connected),
            3
        );
    }
}
