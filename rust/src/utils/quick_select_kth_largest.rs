fn partition(
    list: &mut [i32],
    left: usize,
    right: usize,
    pivot_index: usize,
) -> usize {
    let pivot_value = list[pivot_index];
    list.swap(pivot_index, right);
    let mut store_index = left;

    for i in left..right {
        if list[i] < pivot_value {
            list.swap(store_index, i);
            store_index += 1;
        }
    }
    list.swap(right, store_index);
    store_index
}

fn quick_select(
    list: &mut [i32],
    left: usize,
    right: usize,
    k: usize,
) -> i32 {
    if left == right {
        return list[left];
    }

    let mut pivot_index = left + (right - left) / 2;
    pivot_index = partition(list, left, right, pivot_index);
    match k {
        x if x == pivot_index => list[k],
        x if x < pivot_index => {
            quick_select(list, left, pivot_index - 1, k)
        }

        _ => quick_select(list, pivot_index + 1, right, k),
    }
}

pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    let len = nums.len();
    quick_select(&mut nums, 0, len - 1, len - k as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_kth_largest_ex1() {
        let nums = vec![3, 2, 1, 5, 6, 4];
        let k = 2;
        assert_eq!(find_kth_largest(nums, k), 5);
    }

    #[test]
    fn test_for_kth_largest_ex2() {
        let nums = vec![3, 2, 3, 1, 2, 4, 5, 5, 6];
        let k = 4;
        assert_eq!(find_kth_largest(nums, k), 4);
    }
}
