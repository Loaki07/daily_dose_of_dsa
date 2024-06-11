use std::collections::HashMap;
use std::iter::repeat;

pub struct Solution;

impl Solution {
    pub fn relative_sort_array(
        arr1: Vec<i32>,
        arr2: Vec<i32>,
    ) -> Vec<i32> {
        let value_map: HashMap<i32, usize> = arr2
            .iter()
            .enumerate()
            .map(|(i, &num)| (num, i))
            .collect();

        let (counts, mut left_overs) = arr1.iter().fold(
            (vec![0; value_map.len()], Vec::new()),
            |(mut counts, mut left_overs), &num| {
                match value_map.get(&num) {
                    Some(&value) => counts[value] += 1usize,
                    None => left_overs.push(num),
                }

                (counts, left_overs)
            },
        );
        left_overs.sort_unstable();
        counts
            .into_iter()
            .zip(arr2)
            .flat_map(|(count, value)| {
                repeat(value).take(count)
            })
            .chain(left_overs)
            .collect()
    }

    pub fn _relative_sort_array(
        mut arr1: Vec<i32>,
        arr2: Vec<i32>,
    ) -> Vec<i32> {
        let hm: HashMap<i32, usize> = arr2
            .iter()
            .enumerate()
            .map(|(i, &x)| (x, i))
            .collect();
        arr1.sort_unstable_by_key(|&x| {
            hm.get(&x).map_or((arr2.len(), x), |&i| (i, x))
        });
        arr1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example1() {
        let arr1 = vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19];
        let arr2 = vec![2, 1, 4, 3, 9, 6];
        let result =
            Solution::relative_sort_array(arr1, arr2);
        assert_eq!(
            result,
            vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19]
        );
    }

    #[test]
    fn test_example2() {
        let arr1 = vec![28, 6, 22, 8, 44, 17];
        let arr2 = vec![22, 28, 8, 6];
        let result =
            Solution::relative_sort_array(arr1, arr2);
        assert_eq!(result, vec![22, 28, 8, 6, 17, 44]);
    }
}
