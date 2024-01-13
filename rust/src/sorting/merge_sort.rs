use std::fmt::Debug;

/// Best Case: O(n log n)
/// Worst Case: O(n log n)
/// Average Case: O(n log n)


// sort the left half, O(n log n)
// sort the right half, O(n log n)
// bring the sorted halves together, O(n)
pub fn merge_sort<T: PartialOrd + Debug + Clone + Copy>(
    v: &mut [T],
) -> Vec<T> {
    if v.len() <= 1 {
        return v.to_vec();
    }

    let mut res = Vec::with_capacity(v.len());
    let (left, right) = v.split_at_mut(v.len() / 2);
    let right = merge_sort(right);
    let left = merge_sort(left);

    // bright them together again add
    // whichever is lowest if front of left or front
    // of right
    let mut left_it = left.into_iter();
    let mut right_it = right.into_iter();
    let mut left_peek = left_it.next();
    let mut right_peek = right_it.next();

    loop {
        match left_peek {
            Some(ref left_val) => match right_peek {
                Some(ref right_val) => {
                    if right_val < left_val {
                        res.push(
                            right_peek.take().unwrap(),
                        );
                        right_peek = right_it.next();
                    } else {
                        res.push(left_peek.take().unwrap());
                        left_peek = left_it.next();
                    }
                }
                None => {
                    res.push(left_peek.take().unwrap());
                    res.extend(left_it);
                    return res;
                }
            },
            None => {
                if let Some(right_val) = right_peek {
                    res.push(right_val);
                }
                res.extend(right_it.clone());
                return res;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        let v = merge_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13]);
    }
}
