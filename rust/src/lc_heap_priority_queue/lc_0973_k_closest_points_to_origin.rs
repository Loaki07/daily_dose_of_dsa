pub struct Solution;

impl Solution {
    pub fn k_closest(
        points: Vec<Vec<i32>>,
        k: i32,
    ) -> Vec<Vec<i32>> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut min_heap: BinaryHeap<
            Reverse<(i32, Vec<i32>)>,
        > = BinaryHeap::new();

        //  point: /[1,3]/ etc
        for coord in points {
            let dist = coord[0].pow(2) + coord[1].pow(2);
            min_heap.push(Reverse((dist, coord)));
        }

        // minheap: [Reverse((8, [-2, 2])), Reverse((10,
        // [1, 3]))]
        let mut res: Vec<Vec<i32>> = Vec::new();

        //  input inside of minheap:
        //  [Reverse((8, [-2, 2])), Reverse((10, [1, 3]))]

        while let Some(Reverse((_, coord))) = min_heap.pop()
        {
            //  output after popping the smallest value:
            //  [Reverse((10, [1, 3]))]

            //  Insert this into result vec
            res.push(coord);
            if res.len() == k as usize {
                break;
            }
        }

        res
    }

    // iterative approach using sort
    pub fn _k_closest(
        points: Vec<Vec<i32>>,
        k: i32,
    ) -> Vec<Vec<i32>> {
        let mut dist = points
            .into_iter()
            // .map(|p| {
            //     (
            //         p.clone(),
            //         ((p[0] * p[0] + p[1] * p[1]) as f64)
            //             .sqrt(),
            //     )
            // })
            // to optimize the time removing sqrt
            .map(|p| {
                (p.clone(), (p[0] * p[0] + p[1] * p[1]))
            })
            .collect::<Vec<(Vec<i32>, i32)>>();

        // need to use partial_cmp for float
        // dist.sort_by(|a, b|
        // a.1.partial_cmp(&b.1).unwrap());

        // can use normal cmp for i32
        dist.sort_by(|a, b| a.1.cmp(&b.1));

        dbg!(&dist);
        dist.iter()
            .take(k as usize)
            .map(|e| e.0.clone())
            .collect::<Vec<Vec<i32>>>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::k_closest(
                vec![vec![1, 3], vec![-2, 2]],
                1
            ),
            vec![vec![-2, 2]]
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::k_closest(
                vec![vec![3, 3], vec![5, -1], vec![-2, 4]],
                2
            ),
            vec![vec![3, 3], vec![-2, 4]]
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::k_closest(
                vec![vec![1, 3], vec![-2, 2], vec![2, -2]],
                2
            ),
            vec![vec![-2, 2], vec![2, -2]]
        );
    }
}
